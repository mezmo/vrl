use ::value::Value;
use vrl::prelude::*;
use vrl_core::{conversion::Conversion, Resolved};

fn mezmo_parse_float(value: Value) -> Resolved {
    use Value::{Bytes, Float, Integer};
    match value {
        Float(_) => Ok(value),
        Integer(v) => Ok(Value::from_f64_or_zero(v as f64)),
        Bytes(v) => Conversion::Float
            .convert(v)
            .map_err(|e| e.to_string().into()),
        v => Err(format!("unable to parse {} into float", v.kind()).into()),
    }
}

/// Converts int, float, and string value to a float while matching the behavior
/// of JavaScript's `parseFloat()`. All other types results in an error. This is
/// also different from `to_float()` in that fallibility is determined
/// completely at runtime (there's no `type_def()` check on the parameter).
/// 
/// FIXME: This doesn't properly handle whitespace or invalid chars at the end
/// of the float.
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
        mezmo_parse_float(value)
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::float().fallible()
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
            tdef: TypeDef::float().fallible(),
        }

        integer {
            args: func_args![value: 100],
            want: Ok(100.0),
            tdef: TypeDef::float().fallible(),
        }

        string {
            args: func_args![value: "100.1"],
            want: Ok(100.1),
            tdef: TypeDef::float().fallible(),
        }

        null {
            args: func_args![value: value!(null)],
            want: Err("unable to parse null into float"),
            tdef: TypeDef::float().fallible(),
        }
    ];
}
