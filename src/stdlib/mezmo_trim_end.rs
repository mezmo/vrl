use vrl_compiler::prelude::*;
use vrl_compiler::Resolved;


/// Trims whitespace from the end of a string.
#[derive(Clone, Copy, Debug)]
pub struct MezmoTrimEnd;

impl Function for MezmoTrimEnd {
    fn identifier(&self) -> &'static str {
        "mezmo_trim_end"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                keyword: "value",
                kind: kind::BYTES,
                required: true,
            },
        ]
    }

    fn examples(&self) -> &'static [Example] {
        &[Example {
            title: "basic",
            source: "mezmo_trim_end(\"abc      \")",
            result: Ok("abc"),
        }]
    }

    fn compile(
        &self,
        _state: &state::TypeState,
        _ctx: &mut FunctionCompileContext,
        arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");

        Ok(MezmoTrimEndFn {
            value,
        }
        .as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoTrimEndFn {
    value: Box<dyn Expression>,
}

impl FunctionExpression for MezmoTrimEndFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        Ok(value.try_bytes_utf8_lossy()?.trim_end().into())
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::bytes().infallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_function![
        mezmo_trim_end => MezmoTrimEnd;

        basic {
            args: func_args![value: "abc          "],
            want: Ok("abc"),
            tdef: TypeDef::bytes().infallible(),
        }

        not_trimming_the_start {
            args: func_args![value: "   abc"],
            want: Ok("   abc"),
            tdef: TypeDef::bytes().infallible(),
        }

        empty {
            args: func_args![value: ""],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        only_whitespace {
            args: func_args![value: "     "],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }
    ];
}
