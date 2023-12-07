use std::borrow::Cow;

use ::value::Value;
use vrl_compiler::prelude::*;
use vrl_compiler::Resolved;

fn last_index_of(value: Cow<'_, str>, search_value: Cow<'_, str>, position: Option<i64>) -> Value {
    if search_value.is_empty() {
        return match position {
            Some(position) => Value::from(std::cmp::min(
                std::cmp::max(position, 0), // Negative values clamped to 0
                value.chars().count() as i64,
            )),
            None => Value::from(value.chars().count() as i64),
        };
    }

    let byte_position = match position {
        Some(position) => {
            let position = std::cmp::max(position, 0); // Negative values clamped to 0

            // Convert character position to the byte position in the string
            let byte_position = value
                .char_indices()
                .nth(position as usize)
                .map(|(pos, _)| pos)
                .unwrap_or(value.len());

            // Starting from the search position include search value bytes
            std::cmp::min(byte_position + search_value.len(), value.len())
        }
        None => value.len(), // If position is not provided we search the whole string
    };

    match value.as_ref()[..byte_position].rfind(search_value.as_ref()) {
        Some(found_byte_index) => {
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
/// last occurance of the search string. Returns -1 if the search string is not
/// found.
///
/// If an optional position value is given then only occurences of the search
/// string greater than or equal to that position are considered. Position can
/// be negative and counts from the back of the string to be searched.
///
/// Behaves like the JavaScript's String.prototype.lastIndexOf() method.
#[derive(Clone, Copy, Debug)]
pub struct MezmoLastIndexOf;

impl Function for MezmoLastIndexOf {
    fn identifier(&self) -> &'static str {
        "mezmo_last_index_of"
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
                source: "mezmo_last_index_of(\"abcabc\", \"bc\")",
                result: Ok("4"),
            },
            Example {
                title: "position",
                source: "mezmo_last_index_of(\"abcabc\", \"bc\", 3)",
                result: Ok("1"),
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

        Ok(MezmoLastIndexOfFn {
            value,
            search_value,
            position,
        }
        .as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoLastIndexOfFn {
    value: Box<dyn Expression>,
    search_value: Box<dyn Expression>,
    position: Option<Box<dyn Expression>>,
}

impl FunctionExpression for MezmoLastIndexOfFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        let search_value = self.search_value.resolve(ctx)?;
        let position = match &self.position {
            Some(v) => Some(v.resolve(ctx)?.try_integer()?),
            None => None,
        };

        Ok(last_index_of(
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
        mezmo_last_index_of => MezmoLastIndexOf;

        basic {
            args: func_args![value: "abcabc", search_value: "bc"],
            want: Ok(4),
            tdef: TypeDef::integer().infallible(),
        }

        utf8 {
            args: func_args![value: "नमस्तेनमस्ते", search_value: "स्ते"],
            want: Ok(8),
            tdef: TypeDef::integer().infallible(),
        }

        not_found {
            args: func_args![value: "abc", search_value: "def"],
            want: Ok(-1),
            tdef: TypeDef::integer().infallible(),
        }

        position {
            args: func_args![value: "abcabc", search_value: "bc", position: 3],
            want: Ok(1),
            tdef: TypeDef::integer().infallible(),
        }

        position_boundary {
            args: func_args![value: "abcabc", search_value: "bc", position: 4],
            want: Ok(4),
            tdef: TypeDef::integer().infallible(),
        }

        position_greater_than_length {
            args: func_args![value: "abc", search_value: "bc", position: 100],
            want: Ok(1),
            tdef: TypeDef::integer().infallible(),
        }

        negative_position {
            args: func_args![value: "abcdefabcdef", search_value: "abc", position: -5],
            want: Ok(0),
            tdef: TypeDef::integer().infallible(),
        }

        zero_position {
            args: func_args![value: "abcdefabcdef", search_value: "abc", position: 0],
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
            want: Ok(3),
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
