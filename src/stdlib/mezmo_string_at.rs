use std::borrow::Cow;

use crate::compiler::prelude::*;

fn string_at(s: Cow<'_, str>, index: i64) -> Resolved {
    if index >= 0 {
        Ok(s.chars()
            .nth(index as usize)
            .map(|c| c.to_string())
            .unwrap_or(String::new())
            .into())
    } else {
        Ok(s.chars()
            .nth_back((-(index + 1)) as usize)
            .map(|c| c.to_string())
            .unwrap_or(String::new())
            .into())
    }
}

/// Returns the char at the given index as a string. Allows negative indexes,
/// but indexes out of range, including out of range negative indexes, return an
/// emtpy string.
///
/// Behaves like the JavaScript's String.prototype.at() method except for not
/// returning an error for out of range indexes. In this case an empty string is
/// returned.
#[derive(Clone, Copy, Debug)]
pub struct MezmoStringAt;

impl Function for MezmoStringAt {
    fn identifier(&self) -> &'static str {
        "mezmo_string_at"
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
        ]
    }

    fn examples(&self) -> &'static [Example] {
        &[Example {
            title: "basic",
            source: "mezmo_string_at(\"abc\", 0)",
            result: Ok("a"),
        }]
    }

    fn compile(
        &self,
        _state: &state::TypeState,
        _ctx: &mut FunctionCompileContext,
        arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");
        let index = arguments.required("index");

        Ok(MezmoStringAtFn { value, index }.as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoStringAtFn {
    value: Box<dyn Expression>,
    index: Box<dyn Expression>,
}

impl FunctionExpression for MezmoStringAtFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        let index = self.index.resolve(ctx)?;
        string_at(value.try_bytes_utf8_lossy()?, index.try_integer()?)
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::bytes().infallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_function![
        mezmo_string_at => MezmoStringAt;

        basic {
            args: func_args![value: "abc", index: 0],
            want: Ok("a"),
            tdef: TypeDef::bytes().infallible(),
        }

        negative_index {
            args: func_args![value: "abc", index: -3],
            want: Ok("a"),
            tdef: TypeDef::bytes().infallible(),
        }

        invalid_index {
            args: func_args![value: "abc", index: 4],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        invalid_negative_index {
            args: func_args![value: "abc", index: -4],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        empty {
            args: func_args![value: "", index: 0],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        empty_non_zero {
            args: func_args![value: "", index: 1],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        empty_negative {
            args: func_args![value: "", index: -1],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }
    ];
}
