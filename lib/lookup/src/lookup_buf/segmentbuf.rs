use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use inherent::inherent;
#[cfg(any(test, feature = "arbitrary"))]
use quickcheck::{Arbitrary, Gen};

use crate::{field, LookSegment, Segment};

#[derive(Debug, Eq, Ord, Clone, Hash)]
pub struct FieldBuf {
    pub name: String,
    // This is a very lazy optimization to avoid having to scan for escapes.
    pub requires_quoting: bool,
    // LOG-17092: Track whether the original was quoted so by default we can generate
    // the same string as output from the parsed representation. This upholds the existing
    // API that was previously implemented.
    pub original_quoted: bool,
}

impl FieldBuf {
    pub fn as_str(&self) -> &str {
        &self.name
    }

    /// Returns the field name only quoting the name if the field name contains characters
    /// that would require it to be quoted. For fields that were parsed with quotes but do
    /// not contain any characters that require quotes, the quotes will be omitted.
    pub fn to_humanized_string(&self) -> String {
        if self.requires_quoting {
            format!(r#""{}""#, self.name)
        } else {
            self.name.clone()
        }
    }
}

// LOG-17092: Part of trying to validate whether a field segment is within a path
// exposed an issue where `.segment_name == ."segment_name"` would evaluate to false
// because the derived PartialEq and PartialOrd implementations checked the name and
// requires_quoting field. We don't need to check the requires_quoting field since it's
// only in the struct to avoid scanning the name field for quotable characters every time
// the Field is turned into a string/str.
impl PartialEq for FieldBuf {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl PartialOrd for FieldBuf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Display for FieldBuf {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.requires_quoting || self.original_quoted {
            write!(formatter, r#""{}""#, self.name)
        } else {
            write!(formatter, "{}", self.name)
        }
    }
}

impl From<String> for FieldBuf {
    fn from(mut name: String) -> Self {
        let mut requires_quoting = false;
        let mut original_quoted = false;

        if name.starts_with('\"') && name.ends_with('\"') {
            // There is unfortunately no way to make an owned substring of a string.
            // So we have to take a slice and clone it.
            let len = name.len();
            name = name[1..len - 1].to_string();
            original_quoted = true;
        }

        if !field::is_valid_fieldname(&name) {
            requires_quoting = true;
        }

        Self {
            name,
            requires_quoting,
            original_quoted,
        }
    }
}

impl From<&str> for FieldBuf {
    fn from(name: &str) -> Self {
        Self::from(name.to_string())
    }
}

#[cfg(any(test, feature = "arbitrary"))]
impl Arbitrary for FieldBuf {
    fn arbitrary(g: &mut Gen) -> Self {
        let chars = (32u8..90).map(|c| c as char).collect::<Vec<_>>();
        let len = u32::arbitrary(g) % 100 + 1;
        let name = (0..len)
            .map(|_| chars[usize::arbitrary(g) % chars.len()])
            .collect::<String>()
            .replace('"', r#"\""#);
        FieldBuf::from(name)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.name
                .shrink()
                .filter(|name| !name.is_empty())
                .map(|name| {
                    let name = name.replace('"', r#"/""#);
                    FieldBuf::from(name)
                }),
        )
    }
}

/// `SegmentBuf`s are chunks of a `LookupBuf`.
///
/// They represent either a field or an index. A sequence of `SegmentBuf`s can become a `LookupBuf`.
///
/// This is the owned, allocated side of a `Segment` for `LookupBuf.` It owns its fields unlike `Lookup`. Think of `String` to `&str` or `PathBuf` to `Path`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum SegmentBuf {
    Field(FieldBuf),
    Index(isize),
    // Indexes can be negative.
    // Coalesces hold multiple possible fields.
    Coalesce(Vec<FieldBuf>),
}

#[cfg(any(test, feature = "arbitrary"))]
impl Arbitrary for SegmentBuf {
    fn arbitrary(g: &mut Gen) -> Self {
        match u8::arbitrary(g) % 3 {
            0 => SegmentBuf::Field(FieldBuf::arbitrary(g)),
            1 => SegmentBuf::Index(isize::arbitrary(g) % 100),
            _ => SegmentBuf::Coalesce({
                let mut fields = Vec::arbitrary(g);
                // A coalesce always has at least two fields.
                fields.push(FieldBuf::arbitrary(g));
                fields.push(FieldBuf::arbitrary(g));
                fields
            }),
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            SegmentBuf::Field(field) => Box::new(field.shrink().map(SegmentBuf::Field)),
            SegmentBuf::Index(index) => Box::new(index.shrink().map(SegmentBuf::Index)),
            SegmentBuf::Coalesce(fields) => Box::new(
                fields
                    .shrink()
                    .filter(|fields| fields.len() > 2)
                    .map(SegmentBuf::Coalesce),
            ),
        }
    }
}

#[inherent]
impl<'a> LookSegment<'a> for SegmentBuf {
    type Field = FieldBuf;

    pub fn field(field: FieldBuf) -> SegmentBuf {
        SegmentBuf::Field(field)
    }

    pub fn is_field(&self) -> bool {
        matches!(self, SegmentBuf::Field(_))
    }

    pub fn index(v: isize) -> SegmentBuf {
        SegmentBuf::Index(v)
    }

    pub fn is_index(&self) -> bool {
        matches!(self, SegmentBuf::Index(_))
    }

    pub fn coalesce(v: Vec<FieldBuf>) -> SegmentBuf {
        SegmentBuf::Coalesce(v)
    }

    pub fn is_coalesce(&self) -> bool {
        matches!(self, SegmentBuf::Coalesce(_))
    }
}

impl Display for SegmentBuf {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SegmentBuf::Index(i) => write!(formatter, "{}", i),
            SegmentBuf::Field(field) => write!(formatter, "{}", field),
            SegmentBuf::Coalesce(v) => write!(
                formatter,
                "({})",
                v.iter()
                    .map(|field| field.to_string())
                    .collect::<Vec<_>>()
                    .join(" | ")
            ),
        }
    }
}

impl From<String> for SegmentBuf {
    fn from(name: String) -> Self {
        Self::Field(name.into())
    }
}

impl From<&str> for SegmentBuf {
    fn from(name: &str) -> Self {
        Self::from(name.to_string())
    }
}

impl From<isize> for SegmentBuf {
    fn from(value: isize) -> Self {
        Self::index(value)
    }
}

impl From<Vec<FieldBuf>> for SegmentBuf {
    fn from(value: Vec<FieldBuf>) -> Self {
        Self::coalesce(value)
    }
}

impl<'a> From<Segment<'a>> for SegmentBuf {
    fn from(value: Segment<'a>) -> Self {
        value.as_segment_buf()
    }
}
