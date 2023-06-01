use crate::to_float;
use ::value::Value;
use vrl::prelude::*;

/// Converts any value to float, defaulting to f64(0).
fn mezmo_parse_float(value: Value) -> f64 {
    use Value::Float;
    let v = to_float::to_float(value).unwrap_or_else(|_| Value::from(0.0));
    match v {
        Float(v) => v.into_inner(),
        _ => 0.0,
    }
}

/// Infallible counterpart of ToFloat
#[derive(Clone, Copy, Debug)]
pub struct MezmoParseFloat;

impl Function for MezmoParseFloat {
    fn identifier(&self) -> &'static str {
        "mezmo_parse_float"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[Parameter {
            keyword: "value",
            kind: kind::ANY,
            required: true,
        }]
    }

    fn examples(&self) -> &'static [Example] {
        &[]
    }

    fn compile(
        &self,
        _state: &state::TypeState,
        _ctx: &mut FunctionCompileContext,
        arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");

        Ok(ParseFloatFn { value }.as_expr())
    }
}

#[derive(Debug, Clone)]
struct ParseFloatFn {
    value: Box<dyn Expression>,
}

impl FunctionExpression for ParseFloatFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        Ok(mezmo_parse_float(value).into())
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::float().infallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_function![
        mezmo_parse_float => MezmoParseFloat;

        float {
            args: func_args![value: 20.5],
            want: Ok(20.5),
            tdef: TypeDef::float().infallible(),
        }

        integer {
            args: func_args![value: 100],
            want: Ok(100.0),
            tdef: TypeDef::float().infallible(),
        }

        string {
            args: func_args![value: "100.1"],
            want: Ok(100.1),
            tdef: TypeDef::float().infallible(),
        }

        null {
            args: func_args![value: value!(null)],
            want: Ok(0.0),
            tdef: TypeDef::float().infallible(),
        }
    ];
}
