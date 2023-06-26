use std::borrow::Cow;

use ::value::Value;
use compiler::{value::VrlValueConvert, Expression};
use substring::Substring;
use vrl::prelude::*;
use vrl_core::Resolved;

fn string_slice(s: Cow<'_, str>, index_start: i64, index_end: Option<i64>) -> Value {
    let len = s.chars().count();

    let index_start = normalize_index(index_start, len);

    let index_end = match index_end {
        Some(index_end) => normalize_index(index_end, len),
        None => len,
    };

    if index_end > index_start {
        Value::from(s.substring(index_start, index_end))
    } else {
        Value::from("")
    }
}

fn normalize_index(index: i64, len: usize) -> usize {
    if index < 0 {
        let index = -index as usize;
        if len > index {
            len - index
        } else {
            0
        }
    } else {
        std::cmp::min(index as usize, len)
    }
}

/// Extracts a portion of a string using the provided indexes. Negative indexes
/// are computed from the end and out of range index are clamped to the bounds
/// of the string. Unlike substring this does not flip the start and end indexes
/// if end is less than start.
///
/// Behaves like the JavaScript's String.prototype.slice() method.
#[derive(Clone, Copy, Debug)]
pub struct MezmoStringSlice;

impl Function for MezmoStringSlice {
    fn identifier(&self) -> &'static str {
        "mezmo_string_slice"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                keyword: "value",
                kind: kind::BYTES,
                required: true,
            },
            Parameter {
                keyword: "index",
                kind: kind::INTEGER,
                required: true,
            },
            Parameter {
                keyword: "allow_negative",
                kind: kind::BOOLEAN,
                required: false,
            },
        ]
    }

    fn examples(&self) -> &'static [Example] {
        &[
            Example {
                title: "basic",
                source: "mezmo_string_slice(\"abc\", 1)",
                result: Ok("bc"),
            },
            Example {
                title: "bounds",
                source: "mezmo_string_slice(\"abc\", 1, 2)",
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

        Ok(MezmoStringSliceFn {
            value,
            index_start,
            index_end,
        }
        .as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoStringSliceFn {
    value: Box<dyn Expression>,
    index_start: Box<dyn Expression>,
    index_end: Option<Box<dyn Expression>>,
}

impl FunctionExpression for MezmoStringSliceFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        let index_start = self.index_start.resolve(ctx)?;
        let index_end = match &self.index_end {
            Some(v) => Some(v.resolve(ctx)?.try_integer()?),
            None => None,
        };

        Ok(string_slice(
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
        mezmo_string_slice => MezmoStringSlice;

        basic {
            args: func_args![value: "abc", index_start: 1],
            want: Ok("bc"),
            tdef: TypeDef::bytes().infallible(),
        }

        utf8 {
            args: func_args![value: "नमस्ते", index_start: 0, index_end: 1],
            want: Ok("न"),
            tdef: TypeDef::bytes().infallible(),
        }

        utf8_code_points {
            args: func_args![value: "नमस्ते", index_start: 0, index_end: -2],
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
            want: Ok("c"),
            tdef: TypeDef::bytes().infallible(),
        }

        negative_end {
            args: func_args![value: "abc", index_start: 0, index_end: -1],
            want: Ok("ab"),
            tdef: TypeDef::bytes().infallible(),
        }

        negative_start_and_end {
            args: func_args![value: "abc", index_start: -3, index_end: -2],
            want: Ok("a"),
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
            args: func_args![value: "", index_start: 0, index_end: -1],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        end_greater_than_start {
            args: func_args![value: "abc", index_start: 2, index_end: 0],
            want: Ok(""),
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
