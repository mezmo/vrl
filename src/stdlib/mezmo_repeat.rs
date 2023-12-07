use crate::compiler::prelude::*;

/// Repeats a the given string by the provided count.
///
/// Behaves like the JavaScript's String.prototype.repeat() method except for
/// not returning an error for negative counts. In this case an empty string is
/// returned (behaves as if the count is 0).
#[derive(Clone, Copy, Debug)]
pub struct MezmoRepeat;

impl Function for MezmoRepeat {
    fn identifier(&self) -> &'static str {
        "mezmo_repeat"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                keyword: "value",
                kind: kind::BYTES,
                required: true,
            },
            Parameter {
                keyword: "count",
                kind: kind::INTEGER,
                required: true,
            },
        ]
    }

    fn examples(&self) -> &'static [Example] {
        &[Example {
            title: "basic",
            source: "mezmo_repeat(\"abc\", 3)",
            result: Ok("abcabcabc"),
        }]
    }

    fn compile(
        &self,
        _state: &state::TypeState,
        _ctx: &mut FunctionCompileContext,
        arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");
        let count = arguments.required("count");

        Ok(MezmoRepeatFn { value, count }.as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoRepeatFn {
    value: Box<dyn Expression>,
    count: Box<dyn Expression>,
}

impl FunctionExpression for MezmoRepeatFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        let count = self.count.resolve(ctx)?.try_integer()?;

        let count = std::cmp::max(count, 0) as usize; // Negative values clamped to 0
        Ok(value.try_bytes_utf8_lossy()?.repeat(count).into())
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::bytes().infallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_function![
        mezmo_repeat => MezmoRepeat;

        basic {
            args: func_args![value: "abc", count: 3],
            want: Ok("abcabcabc"),
            tdef: TypeDef::bytes().infallible(),
        }

        negative {
            args: func_args![value: "abc", count: -1],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        zero {
            args: func_args![value: "abc", count: 0],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        empty {
            args: func_args![value: "", count: 1],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }
    ];
}
