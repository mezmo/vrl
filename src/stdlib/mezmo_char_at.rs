use std::borrow::Cow;
// use crate::value;
use crate::compiler::prelude::*;

fn char_at(s: Cow<'_, str>, index: i64) -> Value {
    if index >= 0 {
        s.chars()
            .nth(index as usize)
            .map(|c| c.to_string())
            .unwrap_or(String::new())
            .into()
    } else {
        String::new().into()
    }
}

/// Returns the char at the given index as a string. Negative and out of range
/// indexes return an empty string.
///
/// Behaves like the JavaScript's String.prototype.charAt() method.
#[derive(Clone, Copy, Debug)]
pub struct MezmoCharAt;

impl Function for MezmoCharAt {
    fn identifier(&self) -> &'static str {
        "mezmo_char_at"
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
            source: "mezmo_char_at(\"abc\", 0)",
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

        Ok(MezmoCharAtFn { value, index }.as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoCharAtFn {
    value: Box<dyn Expression>,
    index: Box<dyn Expression>,
}

impl FunctionExpression for MezmoCharAtFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        let index = self.index.resolve(ctx)?;

        Ok(char_at(value.try_bytes_utf8_lossy()?, index.try_integer()?))
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::bytes().infallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_function![
        mezmo_char_at => MezmoCharAt;

        basic {
            args: func_args![value: "abc", index: 0],
            want: Ok("a"),
            tdef: TypeDef::bytes().infallible(),
        }

        negative_index {
            args: func_args![value: "abc", index: -3],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        invalid_index {
            args: func_args![value: "abc", index: 4],
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
