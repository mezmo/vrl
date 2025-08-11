use crate::compiler::conversion::Conversion;
use crate::compiler::prelude::*;

fn to_float(value: Value) -> Resolved {
    use Value::{Boolean, Bytes, Float, Integer, Null, Timestamp};
    match value {
        Float(_) => Ok(value),
        Integer(v) => Ok(Value::from_f64_or_zero(v as f64)),
        Boolean(v) => Ok(NotNan::new(if v { 1.0 } else { 0.0 }).unwrap().into()),
        Null => Ok(NotNan::new(0.0).unwrap().into()),
        Timestamp(v) => Ok(Value::from_f64_or_zero(
            (v.timestamp_nanos_opt().unwrap_or(0)) as f64 / 1_000_000_000_f64,
        )),
        Bytes(v) => {
            let s = String::from_utf8_lossy(&v);
            let s = s.trim().to_owned();
            Conversion::Float
                .convert(s.into())
                .map_err(|e| e.to_string().into())
        }
        v => Err(format!("unable to coerce {} into float", v.kind()).into()),
    }
}

/// Same as the stdlib `to_float()`, but fallibility is determined at runtime
/// (there are no `type_def()` checks) and it trims whitespace.
#[derive(Clone, Copy, Debug)]
pub struct MezmoToFloat;

impl Function for MezmoToFloat {
    fn identifier(&self) -> &'static str {
        "mezmo_to_float"
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

        Ok(ToFloatFn { value }.as_expr())
    }
}

#[derive(Debug, Clone)]
struct ToFloatFn {
    value: Box<dyn Expression>,
}

impl FunctionExpression for ToFloatFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        to_float(value)
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::float().fallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value;

    test_function![
        mezmo_to_float => MezmoToFloat;

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

        string_whitespace {
            args: func_args![value: "    100.1  "],
            want: Ok(100.1),
            tdef: TypeDef::float().fallible(),
        }

        string_whitespace_neg_exp {
            args: func_args![value: "    1e-3  "],
            want: Ok(0.001),
            tdef: TypeDef::float().fallible(),
        }

        string_whitespace_pos_exp {
            args: func_args![value: "    1e+9  "],
            want: Ok(1000000000.0),
            tdef: TypeDef::float().fallible(),
        }

        null {
            args: func_args![value: value!(null)],
            want: Ok(0.0),
            tdef: TypeDef::float().fallible(),
        }

        array {
            args: func_args![value: value!([])],
            want: Err("unable to coerce array into float"),
            tdef: TypeDef::float().fallible(),
        }

        object {
            args: func_args![value: value!({})],
            want: Err("unable to coerce object into float"),
            tdef: TypeDef::float().fallible(),
        }
    ];
}
