use crate::parse_int;
use ::value::Value;
use vrl::prelude::*;

/// Converts any value into an int, defaulting to 0.
fn mezmo_parse_int(value: Value, base: Option<Value>) -> i64 {
    use Value::{Bytes, Float, Integer};
    match value {
        Integer(v) => v,
        Float(v) => v.into_inner() as i64,
        Bytes(_) => {
            let v = parse_int::parse_int(value, base).unwrap_or_else(|_| Value::from(0));
            match v {
                Integer(v) => v,
                _ => 0,
            }
        }
        _ => 0,
    }
}

/// Infallible counterpart of ParseInt
#[derive(Clone, Copy, Debug)]
pub struct MezmoParseInt;

impl Function for MezmoParseInt {
    fn identifier(&self) -> &'static str {
        "mezmo_parse_int"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                keyword: "value",
                kind: kind::ANY,
                required: true,
            },
            Parameter {
                keyword: "base",
                kind: kind::INTEGER,
                required: false,
            },
        ]
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
        let base = arguments.optional("base");

        Ok(ParseIntFn { value, base }.as_expr())
    }
}

#[derive(Debug, Clone)]
struct ParseIntFn {
    value: Box<dyn Expression>,
    base: Option<Box<dyn Expression>>,
}

impl FunctionExpression for ParseIntFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        let base = self
            .base
            .as_ref()
            .map(|expr| expr.resolve(ctx))
            .transpose()?;
        Ok(mezmo_parse_int(value, base).into())
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::integer().infallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_function![
        mezmo_parse_int => MezmoParseInt;

        float {
            args: func_args![value: 20.5],
            want: Ok(20),
            tdef: TypeDef::integer().infallible(),
        }

        integer {
            args: func_args![value: 100],
            want: Ok(100),
            tdef: TypeDef::integer().infallible(),
        }

        null {
            args: func_args![value: value!(null)],
            want: Ok(0),
            tdef: TypeDef::integer().infallible(),
        }

        text {
            args: func_args![value: "hello"],
            want: Ok(0),
            tdef: TypeDef::integer().infallible(),
        }

        boolean {
            args: func_args![value: true],
            want: Ok(0),
            tdef: TypeDef::integer().infallible(),
        }

        decimal {
            args: func_args![value: "-42"],
            want: Ok(-42),
            tdef: TypeDef::integer().infallible(),
        }

        binary {
            args: func_args![value: "0b1001"],
            want: Ok(9),
            tdef: TypeDef::integer().infallible(),
        }

        octal {
            args: func_args![value: "042"],
            want: Ok(34),
            tdef: TypeDef::integer().infallible(),
        }

        hexadecimal {
            args: func_args![value: "0x2a"],
            want: Ok(42),
            tdef: TypeDef::integer().infallible(),
        }

        zero {
            args: func_args![value: "0"],
            want: Ok(0),
            tdef: TypeDef::integer().infallible(),
        }

        explicit_hexadecimal {
            args: func_args![value: "2a", base: 16],
            want: Ok(42),
            tdef: TypeDef::integer().infallible(),
        }
    ];
}
