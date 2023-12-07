use ::value::Value;
use vrl_compiler::prelude::*;

fn to_int(value: Value) -> Resolved {
    use Value::{Boolean, Bytes, Float, Integer, Null, Timestamp};

    match value {
        Integer(_) => Ok(value),
        Float(v) => Ok(Integer(v.into_inner() as i64)),
        Boolean(v) => Ok(Integer(i64::from(v))),
        Null => Ok(0.into()),
        Bytes(v) => {
            let s = String::from_utf8_lossy(&v);
            let parsed = s.trim().parse::<f64>().map_err(|e| {
                <std::string::String as std::convert::Into<ExpressionError>>::into(e.to_string())
            })?;
            if parsed.is_nan() {
                Err("NaN is not supported".into())
            } else {
                Ok(Value::Integer(parsed as i64))
            }
        }
        Timestamp(v) => Ok(v.timestamp().into()),
        v => Err(format!("unable to coerce {} into integer", v.kind()).into()),
    }
}

/// Same as the stdlib `to_int()`, but fallibility is determined at runtime
/// (there are no `type_def()` checks) and it trims whitespace.
#[derive(Clone, Copy, Debug)]
pub struct MezmoToInt;

impl Function for MezmoToInt {
    fn identifier(&self) -> &'static str {
        "mezmo_to_int"
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

        Ok(ToIntFn { value }.as_expr())
    }
}

#[derive(Debug, Clone)]
struct ToIntFn {
    value: Box<dyn Expression>,
}

impl FunctionExpression for ToIntFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        to_int(value)
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::integer().fallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_function![
        mezmo_to_int => MezmoToInt;

        float {
            args: func_args![value: 20.5],
            want: Ok(20),
            tdef: TypeDef::integer().fallible(),
        }

        integer {
            args: func_args![value: 100],
            want: Ok(100),
            tdef: TypeDef::integer().fallible(),
        }

        string {
            args: func_args![value: "100.1"],
            want: Ok(100),
            tdef: TypeDef::integer().fallible(),
        }

        string_whitespace {
            args: func_args![value: "    100.1  "],
            want: Ok(100),
            tdef: TypeDef::integer().fallible(),
        }

        string_whitespace_neg_exp {
            args: func_args![value: "    1e-9  "],
            want: Ok(0),
            tdef: TypeDef::integer().fallible(),
        }

        string_whitespace_pos_exp {
            args: func_args![value: "    1e+9  "],
            want: Ok(1000000000),
            tdef: TypeDef::integer().fallible(),
        }

        null {
            args: func_args![value: value!(null)],
            want: Ok(0),
            tdef: TypeDef::integer().fallible(),
        }
    ];
}
