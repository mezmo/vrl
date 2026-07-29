use crate::compiler::prelude::*;

/// Trims whitespace from the start of a string.
#[derive(Clone, Copy, Debug)]
pub struct MezmoTrimStart;

impl Function for MezmoTrimStart {
    fn identifier(&self) -> &'static str {
        "mezmo_trim_start"
    }

    fn usage(&self) -> &'static str {
        "Returns `value` with leading whitespace removed."
    }

    fn category(&self) -> &'static str {
        Category::String.as_ref()
    }

    fn return_kind(&self) -> u16 {
        kind::BYTES
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[Parameter {
            keyword: "value",
            kind: kind::BYTES,
            required: true,
            description: "The string to trim leading whitespace from.",
            default: None,
            enum_variants: None,
        }]
    }

    fn examples(&self) -> &'static [Example] {
        &[example! {
            title: "basic",
            source: "mezmo_trim_start(\"    abc\")",
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

        Ok(MezmoTrimStartFn { value }.as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoTrimStartFn {
    value: Box<dyn Expression>,
}

impl FunctionExpression for MezmoTrimStartFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        Ok(value.try_bytes_utf8_lossy()?.trim_start().into())
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::bytes().infallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_function![
        mezmo_trim_start => MezmoTrimStart;

        basic {
            args: func_args![value: "            abc"],
            want: Ok("abc"),
            tdef: TypeDef::bytes().infallible(),
        }

        not_trimming_the_end {
            args: func_args![value: "abc   "],
            want: Ok("abc   "),
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
