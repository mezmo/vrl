use crate::parse_int;
use ::value::Value;
use vrl::prelude::*;
use vrl_core::Resolved;

fn mezmo_parse_int(value: Value, base: Option<Value>) -> Resolved {
    use Value::{Bytes, Float, Integer};
    match value {
        Integer(_) => Ok(value),
        Float(v) => Ok(Integer(v.into_inner() as i64)),
        Bytes(_) => parse_int::parse_int(value, base),
        v => Err(format!("unable to parse {} into integer", v.kind()).into()),
    }
}

/// Converts int, float, and string value to an integer while matching the
/// behavior of JavaScript's `parseInt()`. All other types results in an
/// error. This is also different from `to_int()` in that fallibility is
/// determined completely at runtime (there's no `type_def()` check on the
/// parameter).
/// 
/// FIXME: This doesn't properly handle whitespace or invalid chars at the end
/// of the integer.
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
        mezmo_parse_int(value, base)
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::integer().fallible()
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
            tdef: TypeDef::integer().fallible(),
        }

        integer {
            args: func_args![value: 100],
            want: Ok(100),
            tdef: TypeDef::integer().fallible(),
        }

        null {
            args: func_args![value: value!(null)],
            want: Err("unable to parse null into integer"),
            tdef: TypeDef::integer().fallible(),
        }

        text {
            args: func_args![value: "hello"],
            want: Err("could not parse integer: invalid digit found in string"),
            tdef: TypeDef::integer().fallible(),
        }

        boolean {
            args: func_args![value: true],
            want: Err("unable to parse boolean into integer"),
            tdef: TypeDef::integer().fallible(),
        }

        decimal {
            args: func_args![value: "-42"],
            want: Ok(-42),
            tdef: TypeDef::integer().fallible(),
        }

        binary {
            args: func_args![value: "0b1001"],
            want: Ok(9),
            tdef: TypeDef::integer().fallible(),
        }

        octal {
            args: func_args![value: "042"],
            want: Ok(34),
            tdef: TypeDef::integer().fallible(),
        }

        hexadecimal {
            args: func_args![value: "0x2a"],
            want: Ok(42),
            tdef: TypeDef::integer().fallible(),
        }

        zero {
            args: func_args![value: "0"],
            want: Ok(0),
            tdef: TypeDef::integer().fallible(),
        }

        explicit_hexadecimal {
            args: func_args![value: "2a", base: 16],
            want: Ok(42),
            tdef: TypeDef::integer().fallible(),
        }
    ];
}
