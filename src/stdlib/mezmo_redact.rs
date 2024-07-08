use crate::compiler::prelude::*;
use crate::stdlib::mezmo_patterns::*;
use base64::engine::Engine;
use once_cell::sync::Lazy;
use std::collections::{BTreeMap, BTreeSet};
use std::{
    borrow::Cow,
    convert::{TryFrom, TryInto},
};

static US_SOCIAL_SECURITY_NUMBER: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(US_SOCIAL_SECURITY_NUMBER_PATTERN).unwrap());
static EMAIL_ADDRESS: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(EMAIL_ADDRESS_PATTERN).unwrap());
static CREDIT_CARD_NUMBER: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(CREDIT_CARD_PATTERN).unwrap());
static IPV4_ADDRESS: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(IPV4_ADDRESS_PATTERN).unwrap());
static PHONE_NUMBER: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(PHONE_NUMBER_PATTERN).unwrap());

#[derive(Clone, Copy, Debug)]
pub struct MezmoRedact;

impl Function for MezmoRedact {
    fn identifier(&self) -> &'static str {
        "mezmo_redact"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                keyword: "value",
                kind: kind::BYTES | kind::OBJECT | kind::ARRAY,
                required: true,
            },
            Parameter {
                keyword: "filters",
                kind: kind::ARRAY,
                required: true,
            },
        ]
    }

    fn examples(&self) -> &'static [Example] {
        &[
            Example {
                title: "regex",
                source: r#"redact("my id is 123456", filters: [r'\d+'])"#,
                result: Ok(r#"my id is [REDACTED]"#),
            },
            Example {
                title: "us_social_security_number",
                source: r#"redact({ "name": "John Doe", "ssn": "123-12-1234"}, filters: ["us_social_security_number"])"#,
                result: Ok(r#"{ "name": "John Doe", "ssn": "[REDACTED]" }"#),
            },
            Example {
                title: "text redactor",
                source: r#"redact("my id is 123456", filters: [{ "type": "pattern", "patterns": [r'\d+'], "redactor": {"type": "text", "replacement": "***"}}])"#,
                result: Ok(r#"my id is ***"#),
            },
            Example {
                title: "sha2",
                source: r#"redact("my id is 123456", filters: [{ "type": "pattern", "patterns": [r'\d+'], "redactor": "sha2" }])"#,
                result: Ok(r#"my id is GEtTedW1p6tC094dDKH+3B8P+xSnZz69AmpjaXRd63I="#),
            },
            Example {
                title: "sha3",
                source: r#"redact("my id is 123456", filters: [{ "type": "pattern", "patterns", [r'\d+'], redactor: "sha3" }])"#,
                result: Ok(
                    r#"my id is ZNCdmTDI7PeeUTFnpYjLdUObdizo+bIupZdl8yqnTKGdLx6X3JIqPUlUWUoFBikX+yTR+OcvLtAqWO11NPlNJw=="#,
                ),
            },
            Example {
                title: "sha256 hex",
                source: r#"redact("my id is 123456", filters: [{ "type": "pattern", "patterns": [r'\d+'], redactor: {"type": "sha2", "variant": "SHA-256", "encoding": "base16"} }])"#,
                result: Ok(
                    r#"my id is 8d969eef6ecad3c29a3a629280e686cf0c3f5d5a86aff3ca12020c923adc6c92"#,
                ),
            },
        ]
    }

    fn compile(
        &self,
        state: &state::TypeState,
        _ctx: &mut FunctionCompileContext,
        arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");

        let filters = arguments
            .required_array("filters")?
            .into_iter()
            .map(|expr| {
                expr.resolve_constant(state)
                    .ok_or(function::Error::ExpectedStaticExpression {
                        keyword: "filters",
                        expr,
                    })
            })
            .map(|value| {
                value.and_then(|value| {
                    value
                        .clone()
                        .try_into()
                        .map_err(|error| function::Error::InvalidArgument {
                            keyword: "filters",
                            value,
                            error,
                        })
                })
            })
            .collect::<std::result::Result<Vec<FilterWithRedactor>, _>>()?;

        Ok(RedactFn { value, filters }.as_expr())
    }
}

//-----------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct RedactFn {
    value: Box<dyn Expression>,
    filters: Vec<FilterWithRedactor>,
}

fn redact(value: Value, processor: &mut FilterProcessor) -> Value {
    // possible optimization. match the redactor here, and use different calls depending on
    // the value, so that we don't have to do the comparision in the loop of replacment.
    // that would complicate the code though.
    match value {
        Value::Bytes(bytes) => {
            let input = String::from_utf8_lossy(&bytes);
            Value::Bytes(processor.redact(input).into_owned().into())
        }
        Value::Array(values) => {
            let values = values
                .into_iter()
                .map(|value| redact(value, processor))
                .collect();
            Value::Array(values)
        }
        Value::Object(map) => {
            let map = map
                .into_iter()
                .map(|(key, value)| (key, redact(value, processor)))
                .collect();
            Value::Object(map)
        }
        _ => value,
    }
}

impl FunctionExpression for RedactFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        let mut processor = FilterProcessor::new(&self.filters);

        let redacted = redact(value, &mut processor);
        let res = Value::from(BTreeMap::from([
            (KeyString::from("matches"), processor.match_values()),
            (KeyString::from("data"), redacted),
        ]));
        Ok(res)
    }

    fn type_def(&self, state: &state::TypeState) -> TypeDef {
        self.value.type_def(state).infallible()
    }
}

//-----------------------------------------------------------------------------
#[derive(Clone, Debug)]
struct FilterProcessor<'a> {
    matches: BTreeSet<usize>,
    filters: &'a [FilterWithRedactor],
}

impl<'a> FilterProcessor<'a> {
    fn new(filters: &'a [FilterWithRedactor]) -> Self {
        Self {
            filters,
            matches: BTreeSet::new(),
        }
    }

    fn redact<'t>(&mut self, input: Cow<'t, str>) -> Cow<'t, str> {
        self.filters
            .iter()
            .enumerate()
            .fold(input, |input, (index, redact_filter)| {
                let (res, found_match) =
                    redact_filter.filter.redact(&input, &redact_filter.redactor);
                if found_match {
                    self.matches.insert(index);
                }
                res.into_owned().into()
            })
            .into()
    }

    fn match_values(&self) -> Value {
        Value::Array(
            self.matches
                .iter()
                .map(|v| Value::from(*v as i64))
                .collect(),
        )
    }
}

#[derive(Clone, Debug)]
struct FilterWithRedactor {
    filter: Filter,
    redactor: Redactor,
}
impl TryFrom<Value> for FilterWithRedactor {
    type Error = &'static str;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let filter: Filter = value.clone().try_into()?;
        let redactor: Redactor = if let Value::Object(object) = value {
            match object.get("redactor") {
                Some(redactor) => (*redactor).clone().try_into()?,
                _ => Redactor::Full,
            }
        } else {
            Redactor::Full
        };

        Ok(FilterWithRedactor { filter, redactor })
    }
}

/// The redaction filter to apply to the given value.
#[derive(Debug, Clone)]
enum Filter {
    Pattern(Vec<Pattern>),
    UsSocialSecurityNumber,
    CreditCard,
    EmailAddress,
    IPv4Address,
    PhoneNumber,
}

#[derive(Debug, Clone)]
enum Pattern {
    Regex(regex::Regex),
    String(String),
}

impl TryFrom<Value> for Filter {
    type Error = &'static str;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        match value {
            Value::Object(object) => {
                let r#type = match object
                    .get("type")
                    .ok_or("filters specified as objects must have type parameter")?
                {
                    Value::Bytes(bytes) => Ok(bytes.clone()),
                    _ => Err("type key in filters must be a string"),
                }?;

                match r#type.as_ref() {
                    b"us_social_security_number" => Ok(Filter::UsSocialSecurityNumber),
                    b"credit_card" => Ok(Filter::CreditCard),
                    b"email" => Ok(Filter::EmailAddress),
                    b"ipv4" => Ok(Filter::IPv4Address),
                    b"phone_number" => Ok(Filter::PhoneNumber),
                    b"pattern" => {
                        let patterns = match object
                            .get("patterns")
                            .ok_or("pattern filter must have `patterns` specified")?
                        {
                            Value::Array(array) => Ok(array
                                .iter()
                                .map(|value| match value {
                                    Value::Regex(regex) => Ok(Pattern::Regex((**regex).clone())),
                                    Value::Bytes(bytes) => Ok(Pattern::String(
                                        String::from_utf8_lossy(bytes).into_owned(),
                                    )),
                                    _ => Err("`patterns` must be regular expressions"),
                                })
                                .collect::<std::result::Result<Vec<_>, _>>()?),
                            _ => Err("`patterns` must be array of regular expression literals"),
                        }?;
                        Ok(Filter::Pattern(patterns))
                    }
                    _ => Err("unknown filter name"),
                }
            }
            Value::Bytes(bytes) => match bytes.as_ref() {
                b"pattern" => Err("pattern cannot be used without arguments"),
                b"credit_card" => Ok(Filter::CreditCard),
                b"email" => Ok(Filter::EmailAddress),
                b"ipv4" => Ok(Filter::IPv4Address),
                b"phone_number" => Ok(Filter::PhoneNumber),
                b"us_social_security_number" => Ok(Filter::UsSocialSecurityNumber),
                _ => Err("unknown filter name"),
            },
            Value::Regex(regex) => Ok(Filter::Pattern(vec![Pattern::Regex((*regex).clone())])),
            _ => Err("unknown literal for filter, must be a regex, filter name, or object"),
        }
    }
}

impl Filter {
    fn redact<'t>(&self, input: &'t str, redactor: &Redactor) -> (Cow<'t, str>, bool) {
        match &self {
            Filter::Pattern(patterns) => {
                let mut found_match = false;
                let res =
                    patterns
                        .iter()
                        .fold(Cow::Borrowed(input), |input: Cow<'t, str>, pattern| {
                            let value = match pattern {
                                Pattern::Regex(regex) => {
                                    regex.replace_all(&input, redactor).into_owned()
                                }
                                Pattern::String(pattern) => {
                                    str_replace(&input, pattern, redactor).into()
                                }
                            };
                            if value != input {
                                found_match = true;
                            }
                            value.into()
                        });
                (res, found_match)
            }
            Filter::UsSocialSecurityNumber => {
                replace_with_pattern(&US_SOCIAL_SECURITY_NUMBER, input, redactor)
            }
            Filter::CreditCard => replace_with_pattern(&CREDIT_CARD_NUMBER, input, redactor),
            Filter::EmailAddress => replace_with_pattern(&EMAIL_ADDRESS, input, redactor),
            Filter::IPv4Address => replace_with_pattern(&IPV4_ADDRESS, input, redactor),
            Filter::PhoneNumber => replace_with_pattern(&PHONE_NUMBER, input, redactor),
        }
    }
}

fn str_replace(haystack: &str, pattern: &str, redactor: &Redactor) -> String {
    let mut result = String::new();
    let mut last_end = 0;
    for (start, original) in haystack.match_indices(pattern) {
        result.push_str(&haystack[last_end..start]);
        redactor.replace_str(original, &mut result);
        last_end = start + original.len();
    }
    result.push_str(&haystack[last_end..]);
    result
}

fn replace_with_pattern<'t>(
    pattern: &regex::Regex,
    input: &'t str,
    redactor: &Redactor,
) -> (Cow<'t, str>, bool) {
    let redacted = pattern.replace_all(input, redactor);
    if input != redacted {
        (redacted, true)
    } else {
        (redacted, false)
    }
}

/// The recipe for redacting the matched filters.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum Redactor {
    #[default]
    Full,
    /// Replace with a fixed string
    Text(String), // possible optimization: use Arc<str> instead of String to speed up cloning
    // using function pointers simplifies the code,
    // but the Debug implmentation probably isn't very useful
    // alternatively we could have a separate variant for each hash algorithm/variant combination
    // we could also create a custom Debug implementation that does a comparison of the fn pointer
    // to function pointers we might use.
    /// Replace with a hash of the redacted content
    Hash {
        encoder: Encoder,
        hasher: fn(Encoder, &[u8]) -> String,
    },
}

const REDACTED: &str = "[REDACTED]";

impl Redactor {
    fn replace_str(&self, original: &str, dst: &mut String) {
        match self {
            Redactor::Full => {
                dst.push_str(REDACTED);
            }
            Redactor::Text(s) => {
                dst.push_str(s);
            }
            Redactor::Hash { encoder, hasher } => {
                dst.push_str(&hasher(*encoder, original.as_bytes()))
            }
        }
    }

    fn from_object(obj: ObjectMap) -> std::result::Result<Self, &'static str> {
        let r#type = match obj.get("type").ok_or(
            "redactor specified as objects must have type
        parameter",
        )? {
            Value::Bytes(bytes) => Ok(bytes.clone()),
            _ => Err("type key in redactor must be a string"),
        }?;

        match r#type.as_ref() {
            b"full" => Ok(Redactor::Full),
            b"text" => {
                match obj.get("replacement").ok_or(
                    "text redactor must have
                `replacement` specified",
                )? {
                    Value::Bytes(bytes) => {
                        Ok(Redactor::Text(String::from_utf8_lossy(bytes).into_owned()))
                    }
                    _ => Err("`replacement` must be a string"),
                }
            }
            b"sha2" => {
                let hasher = if let Some(variant) = obj.get("variant") {
                    match variant
                        .as_bytes()
                        .ok_or("`variant` must be a string")?
                        .as_ref()
                    {
                        b"SHA-224" => encoded_hash::<sha_2::Sha224>,
                        b"SHA-256" => encoded_hash::<sha_2::Sha256>,
                        b"SHA-384" => encoded_hash::<sha_2::Sha384>,
                        b"SHA-512" => encoded_hash::<sha_2::Sha512>,
                        b"SHA-512/224" => encoded_hash::<sha_2::Sha512_224>,
                        b"SHA-512/256" => encoded_hash::<sha_2::Sha512_256>,
                        _ => return Err("invalid sha2 variant"),
                    }
                } else {
                    encoded_hash::<sha_2::Sha512_256>
                };
                let encoder = obj
                    .get("encoding")
                    .map(Encoder::try_from)
                    .transpose()?
                    .unwrap_or(Encoder::Base64);
                Ok(Redactor::Hash { hasher, encoder })
            }
            b"sha3" => {
                let hasher = if let Some(variant) = obj.get("variant") {
                    match variant
                        .as_bytes()
                        .ok_or("`variant must be a string")?
                        .as_ref()
                    {
                        b"SHA3-224" => encoded_hash::<sha_3::Sha3_224>,
                        b"SHA3-256" => encoded_hash::<sha_3::Sha3_256>,
                        b"SHA3-384" => encoded_hash::<sha_3::Sha3_384>,
                        b"SHA3-512" => encoded_hash::<sha_3::Sha3_512>,
                        _ => return Err("invalid sha2 variant"),
                    }
                } else {
                    encoded_hash::<sha_3::Sha3_512>
                };
                let encoder = obj
                    .get("encoding")
                    .map(Encoder::try_from)
                    .transpose()?
                    .unwrap_or(Encoder::Base64);
                Ok(Redactor::Hash { hasher, encoder })
            }
            _ => Err("unknown `type` for `redactor`"),
        }
    }
}

impl regex::Replacer for &Redactor {
    fn replace_append(&mut self, caps: &regex::Captures, dst: &mut String) {
        self.replace_str(&caps[0], dst);
    }

    fn no_expansion(&mut self) -> Option<Cow<str>> {
        match self {
            Redactor::Full => Some(REDACTED.into()),
            Redactor::Text(s) => Some(s.into()),
            Redactor::Hash { .. } => None,
        }
    }
}

impl TryFrom<Value> for Redactor {
    type Error = &'static str;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        match value {
            Value::Object(object) => Redactor::from_object(object),
            Value::Bytes(bytes) => match bytes.as_ref() {
                b"full" => Ok(Redactor::Full),
                b"sha2" => Ok(Redactor::Hash {
                    hasher: encoded_hash::<sha_2::Sha512_256>,
                    encoder: Encoder::Base64,
                }),
                b"sha3" => Ok(Redactor::Hash {
                    hasher: encoded_hash::<sha_3::Sha3_512>,
                    encoder: Encoder::Base64,
                }),
                _ => Err("unknown name of redactor"),
            },
            _ => Err("unknown literal for redactor, must be redactor name or object"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Encoder {
    Base64,
    Base16,
}

impl TryFrom<&Value> for Encoder {
    type Error = &'static str;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        match value.as_bytes().ok_or("encoding must be string")?.as_ref() {
            b"base64" => Ok(Self::Base64),
            b"base16" | b"hex" => Ok(Self::Base16),
            _ => Err("unexpected encoding"),
        }
    }
}

impl Encoder {
    fn encode(self, data: &[u8]) -> String {
        use Encoder::{Base16, Base64};
        match self {
            Base64 => base64::engine::general_purpose::STANDARD.encode(data),
            Base16 => base16::encode_lower(data),
        }
    }
}

/// Compute the hash of `data` using `T` as the digest, then encode it using `encoder`
/// to get a String
fn encoded_hash<T: digest::Digest>(encoder: Encoder, data: &[u8]) -> String {
    encoder.encode(&T::digest(data))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::btreemap;
    use crate::value;
    use regex::Regex;

    test_function![
        mezmo_redact => MezmoRedact;

        regex {
            args: func_args![
                value: "hello 123456 world",
                filters: vec![Regex::new(r"\d+").unwrap()],
            ],
            want: Ok(value!({
                "data": "hello [REDACTED] world",
                "matches": [0]
            })),
            tdef: TypeDef::bytes().infallible(),
       }

       patterns {
            args: func_args![
                value: "hello 123456 world",
                filters: vec![
                    value!({
                        "type": "pattern",
                        "patterns": ["123456"]
                    })
                ],
            ],
            want: Ok(value!({
                "data": "hello [REDACTED] world",
                "matches": [0]
            })),
            tdef: TypeDef::bytes().infallible(),
       }

       us_social_security_number {
            args: func_args![
                value: "hello 123-12-1234 world",
                filters: vec!["us_social_security_number"],
            ],
            want: Ok(value!({
                "data": "hello [REDACTED] world",
                "matches": [0]
            })),
            tdef: TypeDef::bytes().infallible(),
       }

       invalid_filter {
            args: func_args![
                value: "hello 123456 world",
                filters: vec!["not a filter"],
            ],
            want: Err("invalid argument"),
            tdef: TypeDef::bytes().infallible(),
       }

       missing_patterns {
            args: func_args![
                value: "hello 123456 world",
                filters: vec![
                    value!({
                        "type": "pattern",
                    })
                ],
            ],
            want: Err("invalid argument"),
            tdef: TypeDef::bytes().infallible(),
       }

       text_redactor {
        args: func_args![
            value: "my id is 123456",
            filters: vec![btreemap!{
                "type" => "pattern",
                "patterns" => vec![Regex::new(r"\d+").unwrap()],
                "redactor" => btreemap!{"type" => "text", "replacement" => "***"}
            }],
        ],
        want: Ok(value!({
            "data": "my id is ***",
            "matches": [0]
        })),
        tdef: TypeDef::bytes().infallible(),
    }

    sha2 {
        args: func_args![
            value: "my id is 123456",
            filters: vec![btreemap!{
                "type" => "pattern",
                "patterns" => vec![Regex::new(r"\d+").unwrap()],
                "redactor" => "sha2"
            }],
        ],
        want: Ok(value!({
            "data": "my id is GEtTedW1p6tC094dDKH+3B8P+xSnZz69AmpjaXRd63I=",
            "matches": [0]
        })),
        tdef: TypeDef::bytes().infallible(),
    }

    sha3 {
        args: func_args![
            value: "my id is 123456",
            filters: vec![btreemap!{
                "type" => "pattern",
                "patterns" => vec![Regex::new(r"\d+").unwrap()],
                "redactor" => "sha3"
            }],
        ],
        want: Ok(value!({
            "data": "my id is ZNCdmTDI7PeeUTFnpYjLdUObdizo+bIupZdl8yqnTKGdLx6X3JIqPUlUWUoFBikX+yTR+OcvLtAqWO11NPlNJw==",
            "matches": [0]
        })),
        tdef: TypeDef::bytes().infallible(),
    }

    sha256_hex {
        args: func_args![
            value: "my id is 123456",
            filters: vec![btreemap!{
                "type" => "pattern",
                "patterns" => vec![Regex::new(r"\d+").unwrap()],
                "redactor" => btreemap!{"type" => "sha2", "variant" => "SHA-256", "encoding" =>
                "base16"}
            }],
        ],
        want: Ok(value!({
            "data": "my id is 8d969eef6ecad3c29a3a629280e686cf0c3f5d5a86aff3ca12020c923adc6c92",
            "matches": [0]
        })),
        tdef: TypeDef::bytes().infallible(),
    }

    invalid_redactor {
         args: func_args![
             value: "hello 123456 world",
             filters: vec![btreemap!{
                "type" => "us_social_security_number",
                "redactor" => "not a redactor"
             }],
         ],
         want: Err("invalid argument"),
         tdef: TypeDef::bytes().infallible(),
    }

    invalid_redactor_obj {
         args: func_args![
             value: "hello 123456 world",
             filters: vec![btreemap!{
                "type" => "us_social_security_number",
                "redactor" => btreemap!{"type" => "wrongtype"},
             }],
         ],
         want: Err("invalid argument"),
         tdef: TypeDef::bytes().infallible(),
    }

    invalid_redactor_no_type {
         args: func_args![
             value: "hello 123456 world",
             filters: vec![btreemap!{
                "type" => "us_social_security_number",
                "redactor" => btreemap!{"key" => "value"},
             }],
         ],
         want: Err("invalid argument"),
         tdef: TypeDef::bytes().infallible(),
    }

    invalid_hash_variant {
         args: func_args![
             value: "hello 123456 world",
             filters: vec![btreemap!{
                "type" => "us_social_security_number",
                "redactor" => btreemap!{"type" => "sha2", "variant" => "MD5"}
             }],
         ],
         want: Err("invalid argument"),
         tdef: TypeDef::bytes().infallible(),
    }

        us_social_security_number_additional {
            args: func_args![
                value: vec![
                    "242-22-1348 you said?",
                    "I guess that is valid. How about: ",
                    "333 44 5231, 400-20 2000 and 525 44-9000",
                    "Those seem to be valid too. However, ",
                    "666 33 1442, 5872-22-2244, 187-223-3342 are not valid"
                ].join("\n"),
                filters: vec!["us_social_security_number"],
            ],
            want: Ok(Value::from(btreemap!{
                "data" => vec![
                    "[REDACTED] you said?",
                    "I guess that is valid. How about: ",
                    "[REDACTED], [REDACTED] and [REDACTED]",
                    "Those seem to be valid too. However, ",
                    "666 33 1442, 5872-22-2244, 187-223-3342 are not valid"
                ].join("\n"),
                "matches" => value!([0])
            })),
            tdef: TypeDef::bytes().infallible(),
       }

        email_address {
            args: func_args![
                value: vec![
                    "jon@doe.com, jon.doe@company.com are valid",
                    "So are second+user@example.com and last_user@example.com",
                    "dev@null and @null.com are not valid",
                    "IP based emails such as local@127.0.0.1 and user@1.1.1.1 are also supported"
                ].join("\n"),
                filters: vec!["email"],
            ],
            want: Ok(Value::from(btreemap!{
                "data" => vec![
                    "[REDACTED], [REDACTED] are valid",
                    "So are [REDACTED] and [REDACTED]",
                    "dev@null and @null.com are not valid",
                    "IP based emails such as [REDACTED] and [REDACTED] are also supported"
                ].join("\n"),
                "matches" => value!([0])
            })),
            tdef: TypeDef::bytes().infallible(),
        }

        phone_number {
            args: func_args![
                value: vec![
                    "+1 (422) 386 4425, 1 222 333 4343, (422) 000 4000 and 318 111 3000 are valid",
                    "So are the dot-delimited numbers +1.(422).386.4425, 1.222.333.4343, (422).000.4000 and 318.111.3000",
                    "Hypenated numbers are also valid",
                    "Examples: +1-(422)-386-4425, 1-222-333-4343, (422)-000-4000, 318-111-3000",
                    "Next up is numbers with a mix of delimiters.",
                    "Examples are: +1 (422)-386.4425, 1-222.333-4343, (422)-000 4000, 318 111.3000 and +20 (422)-388-4421",
                    "Lastly, no delimiter numbers are also valid.",
                    "These include: +12223334444, 12223334444, +1(000)2225555, 1(000)2225555 and (288)4279999",
                    "The pattern matches parts of some sequences such as +1_316.222.4000, ",
                    "22.4237.333.5555 and +200 388 2222 5000",
                    "Invalid numbers are: +20 (422)-3888-4421, 1 200 3000 4000, ",
                    "+1 388 2222 4000, and +22 (289) 477 50001 are also not valid"
                ].join("\n"),
                filters: vec!["phone_number"],
            ],
            want: Ok(Value::from(btreemap!{
                "data" => vec![
                    "[REDACTED], [REDACTED], [REDACTED] and [REDACTED] are valid",
                        "So are the dot-delimited numbers [REDACTED], [REDACTED], [REDACTED] and [REDACTED]",
                        "Hypenated numbers are also valid",
                        "Examples: [REDACTED], [REDACTED], [REDACTED], [REDACTED]",
                        "Next up is numbers with a mix of delimiters.",
                        "Examples are: [REDACTED], [REDACTED], [REDACTED], [REDACTED] and [REDACTED]",
                        "Lastly, no delimiter numbers are also valid.",
                        "These include: [REDACTED], [REDACTED], [REDACTED], [REDACTED] and [REDACTED]",
                        "The pattern matches parts of some sequences such as +1_[REDACTED], ",
                        "22.[REDACTED] and +[REDACTED] 5000",
                        "Invalid numbers are: +20 (422)-3888-4421, 1 200 3000 4000, ",
                        "+1 388 2222 4000, and +22 (289) 477 50001 are also not valid",
                ].join("\n"),
                "matches" => value!([0])
            })),
            tdef: TypeDef::bytes().infallible(),
        }

        credit_card_visa {
            args: func_args![
                value: vec![
                    "4012102240330 and 4111000222333444 are valid",
                    "400011122233 is not valid"
                ].join("\n"),
                filters: vec!["credit_card"],
            ],
            want: Ok(Value::from(btreemap!{
                "data" => vec![
                    "[REDACTED] and [REDACTED] are valid",
                    "400011122233 is not valid"
                ].join("\n"),
                "matches" => value!([0])
            })),
            tdef: TypeDef::bytes().infallible(),
        }

        credit_card_mastercard {
            args: func_args![
                value: vec![
                    "Older range like 5100112233445566 and 5222334455667788 will match",
                    "Newer range: 2288776655443322 and 2700112233445566 should also match",
                    "22887766554433 and 52223344556677 are invalid"
                ].join("\n"),
                filters: vec!["credit_card"],
            ],
            want: Ok(Value::from(btreemap!{
                "data" => vec![
                    "Older range like [REDACTED] and [REDACTED] will match",
                        "Newer range: [REDACTED] and [REDACTED] should also match",
                        "22887766554433 and 52223344556677 are invalid"
                ].join("\n"),
                "matches" => value!([0])
            })),
            tdef: TypeDef::bytes().infallible(),
        }

        credit_card_discover {
            args: func_args![
                value: vec![
                    "start with 6011 or 65 and have a total of 16 digits",
                    "6011000111222333 and 6511122233344477 are valid",
                    "60110001112223 and 6511122233344 are invalid"
                ].join("\n"),
                filters: vec!["credit_card"],
            ],
            want: Ok(Value::from(btreemap!{
                "data" => vec![
                    "start with 6011 or 65 and have a total of 16 digits",
                        "[REDACTED] and [REDACTED] are valid",
                        "60110001112223 and 6511122233344 are invalid"
                ].join("\n"),
                "matches" => value!([0])
            })),
            tdef: TypeDef::bytes().infallible(),
        }

        credit_card_amex {
            args: func_args![
                value: vec![
                    "Both 340001112223334 and 371111111122223 are valid",
                    "34000111222333 and 3700022233344 are not valid"
                ].join("\n"),
                filters: vec!["credit_card"],
            ],
            want: Ok(Value::from(btreemap!{
                "data" => vec![
                    "Both [REDACTED] and [REDACTED] are valid",
                        "34000111222333 and 3700022233344 are not valid"
                ].join("\n"),
                "matches" => value!([0])
            })),
            tdef: TypeDef::bytes().infallible(),
        }

        credit_card_diners {
            args: func_args![
                value: vec![
                    "30011122233344, 38900011122233 are valid",
                    "3011112223334, 380111222333 are not valid"
                ].join("\n"),
                filters: vec!["credit_card"],
            ],
            want: Ok(Value::from(btreemap!{
                "data" => vec![
                    "[REDACTED], [REDACTED] are valid",
                        "3011112223334, 380111222333 are not valid"
                ].join("\n"),
                "matches" => value!([0])
            })),
            tdef: TypeDef::bytes().infallible(),
        }

        credit_card_jcb {
            args: func_args![
                value: vec![
                    "213100000000011, 180011111111122, 3500011111111122 and 3522244444444411 are valid",
                    "3500111222333 is not valid"
                ].join("\n"),
                filters: vec!["credit_card"],
            ],
            want: Ok(Value::from(btreemap!{
                "data" => vec![
                    "[REDACTED], [REDACTED], [REDACTED] and [REDACTED] are valid",
                    "3500111222333 is not valid"
                ].join("\n"),
                "matches" => value!([0])
            })),
            tdef: TypeDef::bytes().infallible(),
        }

        with_multiple_filters {
            args: func_args![
                value: value!({
                    "arr": [
                        "contains 180011111111122, a JCB credit card",
                        {
                            "user": "JB Daniels",
                            "email": "jb_daniels@user.com is the previous email",
                        }
                    ],
                    "pinfo": {
                        "about": "information about the user",
                        "position": "product manager",
                        "payment_info": {
                            "contact": {
                                "primary": "+22 330 (447)-819213 (invalid)",
                                "secondary": ["user@info.com", "220-330-4444"]
                            },
                            "ssn": "old: 123-443-5555 (invalid), new: 221-55-7788"
                        }
                    },
                    "favorite_players": {
                        "dallas": ["luka doncic", "kyrie irving"],
                        "wolves": ["antman"]
                    }
                }),
                filters: vec![
                    btreemap!{
                        "type" => "pattern",
                        "patterns" => vec![r"\w+@org.net", r"\w+\d{6}$"]
                    },
                    btreemap!{
                        "type" => "credit_card",
                        "redactor" => btreemap!{"type" => "text", "replacement" => "[CREDIT_CARD]" },
                    },
                    btreemap!{
                        "type" => "us_social_security_number",
                        "redactor" => btreemap!{"type" => "text", "replacement" => "[SSN]" },
                    },
                    btreemap!{
                        "type" => "pattern",
                        "patterns" => vec![r"^Johnson"],
                    },
                    btreemap!{
                        "type" => "phone_number",
                        "redactor" => btreemap!{"type" => "text", "replacement" => "[PHONE_NUMBER]" },
                    },
                    btreemap!{
                        "type" => "email",
                        "redactor" => btreemap!{"type" => "text", "replacement" => "[EMAIL_ADDRESS]" },
                    },
                ]
            ],
            want: Ok(Value::from(btreemap!{
                "data" => value!({
                    "arr": [
                        "contains [CREDIT_CARD], a JCB credit card",
                        {
                            "user": "JB Daniels",
                            "email": "[EMAIL_ADDRESS] is the previous email",
                        }
                    ],
                    "pinfo": {
                        "about": "information about the user",
                        "position": "product manager",
                        "payment_info": {
                            "contact": {
                                "primary": "+22 330 (447)-819213 (invalid)",
                                "secondary": ["[EMAIL_ADDRESS]", "[PHONE_NUMBER]"]
                            },
                            "ssn": "old: [PHONE_NUMBER] (invalid), new: [SSN]"
                        }
                    },
                    "favorite_players": {
                        "dallas": ["luka doncic", "kyrie irving"],
                        "wolves": ["antman"]
                    }
                }),
                "matches" => value!([1, 2, 4, 5])
            })),
            tdef: TypeDef::object(btreemap!{
                Field::from("arr") => TypeDef::array(btreemap!{
                    Index::from(0) => Kind::bytes(),
                    Index::from(1) => TypeDef::object(btreemap!{
                        Field::from("user") => Kind::bytes(),
                        Field::from("email") => Kind::bytes()
                    }),
                }),
                Field::from("pinfo") => TypeDef::object(btreemap!{
                    Field::from("about") => Kind::bytes(),
                    Field::from("position") => Kind::bytes(),
                    Field::from("payment_info") => TypeDef::object(btreemap!{
                        Field::from("contact") => TypeDef::object(btreemap!{
                            Field::from("primary") => Kind::bytes(),
                            Field::from("secondary") => TypeDef::array(btreemap!{
                                Index::from(0) => Kind::bytes(),
                                Index::from(1) => Kind::bytes()
                            }),
                        }),
                        Field::from("ssn") => Kind::bytes(),
                    })
                }),
                Field::from("favorite_players") => TypeDef::object(btreemap!{
                    Field::from("dallas") => TypeDef::array(btreemap!{
                        Index::from(0) => Kind::bytes(),
                        Index::from(1) => Kind::bytes()
                    }),
                    Field::from("wolves") => TypeDef::array(btreemap!{
                        Index::from(0) => Kind::bytes()
                    })
                })
            }).infallible(),
        }
    ];
}
