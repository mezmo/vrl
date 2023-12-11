use crate::compiler::prelude::*;
use std::borrow::Cow;

fn index_of(value: Cow<'_, str>, search_value: Cow<'_, str>, position: i64) -> Value {
    let position = std::cmp::max(position, 0); // Negative values clamped to 0

    if search_value.is_empty() {
        return Value::from(std::cmp::min(position, value.chars().count() as i64));
    }

    // Convert character position to the byte position in the string
    let byte_position = value
        .char_indices()
        .nth(position as usize)
        .map(|(pos, _)| pos)
        .unwrap_or(value.len());

    if byte_position == value.len() {
        // Nothing to search
        return Value::from(-1);
    }

    match value.as_ref()[byte_position..].find(search_value.as_ref()) {
        Some(found_byte_index) => {
            let found_byte_index = found_byte_index + byte_position; // Absolute index within string
                                                                     // Convert the byte index in the string to the character index in the string
            let found = value
                .char_indices()
                .enumerate()
                .find(|(_, (byte_index, _))| found_byte_index == *byte_index);
            match found {
                Some(found) => Value::from(found.0 as i64),
                None => Value::from(-1), // This means search value is not valid utf8?
            }
        }
        None => Value::from(-1),
    }
}

/// Searches a given string for a search string and returns the index of the
/// first occurance of the search string. Returns -1 if the search string is not
/// found.
///
/// If an optional position value is given then only occurences of the search
/// string greater than or equal to that position are considered. Position can
/// be negative and counts from the back of the string to be searched.
///
/// Behaves like the JavaScript's String.prototype.indexOf() method.
#[derive(Clone, Copy, Debug)]
pub struct MezmoIndexOf;

impl Function for MezmoIndexOf {
    fn identifier(&self) -> &'static str {
        "mezmo_index_of"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                keyword: "value",
                kind: kind::BYTES,
                required: true,
            },
            Parameter {
                keyword: "search_value",
                kind: kind::BYTES,
                required: true,
            },
            Parameter {
                keyword: "position",
                kind: kind::INTEGER,
                required: false,
            },
        ]
    }

    fn examples(&self) -> &'static [Example] {
        &[
            Example {
                title: "basic",
                source: "mezmo_index_of(\"abc\", \"bc\")",
                result: Ok("1"),
            },
            Example {
                title: "position",
                source: "mezmo_index_of(\"abcdefabcdef\", \"abc\", 6)",
                result: Ok("6"),
            },
        ]
    }

    fn compile(
        &self,
        _state: &state::TypeState,
        _ctx: &mut FunctionCompileContext,
        arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");
        let search_value = arguments.required("search_value");
        let position = arguments.optional("position");

        Ok(MezmoIndexOfFn {
            value,
            search_value,
            position,
        }
        .as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoIndexOfFn {
    value: Box<dyn Expression>,
    search_value: Box<dyn Expression>,
    position: Option<Box<dyn Expression>>,
}

impl FunctionExpression for MezmoIndexOfFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        let search_value = self.search_value.resolve(ctx)?;
        let position = match &self.position {
            Some(v) => v.resolve(ctx)?.try_integer()?,
            None => 0,
        };

        Ok(index_of(
            value.try_bytes_utf8_lossy()?,
            search_value.try_bytes_utf8_lossy()?,
            position,
        ))
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::integer().infallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_function![
        mezmo_index_of => MezmoIndexOf;

        basic {
            args: func_args![value: "abc", search_value: "bc"],
            want: Ok(1),
            tdef: TypeDef::integer().infallible(),
        }

        utf8 {
            args: func_args![value: "नमस्ते", search_value: "स्ते"],
            want: Ok(2),
            tdef: TypeDef::integer().infallible(),
        }

        not_found {
            args: func_args![value: "abc", search_value: "def"],
            want: Ok(-1),
            tdef: TypeDef::integer().infallible(),
        }

        position {
            args: func_args![value: "abcdefabcdef", search_value: "abc", position: 6],
            want: Ok(6),
            tdef: TypeDef::integer().infallible(),
        }

        position_greater_than_length {
            args: func_args![value: "abc", search_value: "bc", position: 100],
            want: Ok(-1),
            tdef: TypeDef::integer().infallible(),
        }

        negative_position {
            args: func_args![value: "abcdefabcdef", search_value: "abc", position: -6],
            want: Ok(0),
            tdef: TypeDef::integer().infallible(),
        }

        empty {
            args: func_args![value: "", search_value: ""],
            want: Ok(0),
            tdef: TypeDef::integer().infallible(),
        }

        search_non_empty_with_empty {
            args: func_args![value: "abc", search_value: ""],
            want: Ok(0),
            tdef: TypeDef::integer().infallible(),
        }

        empty_with_position {
            args: func_args![value: "abc", search_value: "", position: 3],
            want: Ok(3),
            tdef: TypeDef::integer().infallible(),
        }

        empty_with_negative_position {
            args: func_args![value: "abc", search_value: "", position: -1],
            want: Ok(0),
            tdef: TypeDef::integer().infallible(),
        }

        empty_with_position_exceed_length {
            args: func_args![value: "abc", search_value: "", position: 6],
            want: Ok(3),
            tdef: TypeDef::integer().infallible(),
        }
    ];
}
