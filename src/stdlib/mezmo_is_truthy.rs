use crate::compiler::prelude::*;

fn is_truthy(value: Value) -> bool {
    use Value::{Boolean, Bytes, Float, Integer, Null};
    match value {
        Float(v) => v != 0.0,
        Integer(v) => v != 0,
        Boolean(v) => v,
        Null => false,
        Bytes(v) => v.len() > 0,
        _ => true,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MezmoIsTruthy;

impl Function for MezmoIsTruthy {
    fn identifier(&self) -> &'static str {
        "mezmo_is_truthy"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[Parameter {
            keyword: "value",
            kind: kind::ANY,
            required: true,
        }]
    }

    fn examples(&self) -> &'static [Example] {
        &[
            Example {
                title: "integer",
                source: "mezmo_is_truthy(5)",
                result: Ok("true"),
            },
            Example {
                title: "float",
                source: "mezmo_is_truthy(5.6)",
                result: Ok("true"),
            },
            Example {
                title: "integer",
                source: "mezmo_is_truthy(0)",
                result: Ok("false"),
            },
            Example {
                title: "true",
                source: "mezmo_is_truthy(true)",
                result: Ok("true"),
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

        Ok(MezmoIsTruthyFn { value }.as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoIsTruthyFn {
    value: Box<dyn Expression>,
}

impl FunctionExpression for MezmoIsTruthyFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        Ok(is_truthy(value).into())
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::boolean().infallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value;
    use chrono::{prelude::*, TimeZone};

    test_function![
        is_truthy => MezmoIsTruthy;

        float {
            args: func_args![value: 20.5],
            want: Ok(true),
            tdef: TypeDef::boolean().infallible(),
        }

        float_zero {
            args: func_args![value: 0.0],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }

        integer_zero {
            args: func_args![value: 0],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }

        integer_negative {
            args: func_args![value: -1],
            want: Ok(true),
            tdef: TypeDef::boolean().infallible(),
        }

        string {
            args: func_args![value: "abc"],
            want: Ok(true),
            tdef: TypeDef::boolean().infallible(),
        }

        string_empty {
            args: func_args![value: ""],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }

        null {
            args: func_args![value: value!(null)],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }

        timestamp {
             args: func_args![value: Utc.ymd(2014, 7, 8).and_hms_milli(9, 10, 11, 12)],
             want: Ok(true),
             tdef: TypeDef::boolean().infallible(),
        }
    ];
}
