use std::borrow::Cow;

use crate::compiler::prelude::*;
use substring::Substring;

fn substring(s: Cow<'_, str>, index_start: i64, index_end: Option<i64>) -> Value {
    let len = s.chars().count();
    let index_start = normalize_index(index_start, len);
    let index_end = match index_end {
        Some(index_end) => normalize_index(index_end, len),
        None => len,
    };
    if index_end < index_start {
        Value::from(s.substring(index_end, index_start))
    } else {
        Value::from(s.substring(index_start, index_end))
    }
}

fn normalize_index(index: i64, len: usize) -> usize {
    if index < 0 {
        0
    } else if (index as usize) > len {
        len
    } else {
        index as usize
    }
}

/// Extracts a portion of a string using the provided indexes. Negative indexes
/// are clamped to 0 and indexes larger than the string length are clamped to
/// the length. If the start index is larger than the end index the indexes are
/// flipped.
///
/// Behaves like the JavaScript's String.prototype.substring() method.
#[derive(Clone, Copy, Debug)]
pub struct MezmoSubstring;

impl Function for MezmoSubstring {
    fn identifier(&self) -> &'static str {
        "mezmo_substring"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                keyword: "value",
                kind: kind::BYTES,
                required: true,
            },
            Parameter {
                keyword: "index_start",
                kind: kind::INTEGER,
                required: true,
            },
            Parameter {
                keyword: "index_end",
                kind: kind::INTEGER,
                required: false,
            },
        ]
    }

    fn examples(&self) -> &'static [Example] {
        &[
            Example {
                title: "basic",
                source: "mezmo_substring(\"abc\", 1)",
                result: Ok("bc"),
            },
            Example {
                title: "bounds",
                source: "mezmo_substring(\"abc\", 1, 2)",
                result: Ok("b"),
            },
            Example {
                title: "flipped_bounds",
                source: "mezmo_substring(\"abc\", 2, 1)",
                result: Ok("b"),
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
        let index_start = arguments.required("index_start");
        let index_end = arguments.optional("index_end");

        Ok(MezmoSubstringFn {
            value,
            index_start,
            index_end,
        }
        .as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoSubstringFn {
    value: Box<dyn Expression>,
    index_start: Box<dyn Expression>,
    index_end: Option<Box<dyn Expression>>,
}

impl FunctionExpression for MezmoSubstringFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        let index_start = self.index_start.resolve(ctx)?;
        let index_end = match &self.index_end {
            Some(v) => Some(v.resolve(ctx)?.try_integer()?),
            None => None,
        };

        Ok(substring(
            value.try_bytes_utf8_lossy()?,
            index_start.try_integer()?,
            index_end,
        ))
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::bytes().infallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_function![
        mezmo_substring => MezmoSubstring;

        basic {
            args: func_args![value: "abc", index_start: 1],
            want: Ok("bc"),
            tdef: TypeDef::bytes().infallible(),
        }

        flipped_indexes {
            args: func_args![value: "abc", index_start: 8, index_end: 0],
            want: Ok("abc"),
            tdef: TypeDef::bytes().infallible(),
        }

        utf8 {
            args: func_args![value: "नमस्ते", index_start: 0, index_end: 1],
            want: Ok("न"),
            tdef: TypeDef::bytes().infallible(),
        }

        utf8_code_points {
            args: func_args![value: "नमस्ते", index_start: 0, index_end: 4],
            want: Ok("नमस्"),
            tdef: TypeDef::bytes().infallible(),
        }

        start_and_end {
            args: func_args![value: "abc", index_start: 1, index_end: 2],
            want: Ok("b"),
            tdef: TypeDef::bytes().infallible(),
        }

        same_start_and_end {
            args: func_args![value: "abc", index_start: 2, index_end: 2],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        index_end_greater_than_length {
            args: func_args![value: "abc", index_start: 1, index_end: 100],
            want: Ok("bc"),
            tdef: TypeDef::bytes().infallible(),
        }

        negative_start {
            args: func_args![value: "abc", index_start: -1, index_end: 100],
            want: Ok("abc"),
            tdef: TypeDef::bytes().infallible(),
        }

        negative_end {
            args: func_args![value: "abc", index_start: 0, index_end: -1],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        negative_start_and_end {
            args: func_args![value: "abc", index_start: -3, index_end: -2],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        zero_indexes {
            args: func_args![value: "abc", index_start: 0, index_end: 0],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        empty {
            args: func_args![value: "", index_start: 0],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        empty_start_and_end {
            args: func_args![value: "", index_start: 0, index_end: 1],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        end_greater_than_start {
            args: func_args![value: "abc", index_start: 2, index_end: 0],
            want: Ok("ab"),
            tdef: TypeDef::bytes().infallible(),
        }

        negative_end_greater_than_start {
            args: func_args![value: "abc", index_start: -2, index_end: -3],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        negative_greater_than_length {
            args: func_args![value: "abc", index_start: -100],
            want: Ok("abc"),
            tdef: TypeDef::bytes().infallible(),
        }
    ];
}
