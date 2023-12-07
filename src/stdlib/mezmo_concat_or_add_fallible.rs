use crate::mezmo_to_string;
use vrl_compiler::prelude::*;

pub(crate) const ERROR_MESSAGE: &str = "Cannot add or concat other values that are not strings or numbers";

/// Concatenates if any of the parameters is a string.
/// Adds if both are numbers.
pub(crate) fn concat_or_add(left: Value, right: Value) -> Resolved {
    use Value::{Bytes, Float, Integer};
    match (&left, &right) {
        (Bytes(_), _) | (_, Bytes(_)) => {
            let left = mezmo_to_string::to_string(left);
            let right = mezmo_to_string::to_string(right);
            Ok(Value::from(left + &right))
        }
        (Float(l), Float(r)) => Ok(Value::from(l.into_inner() + r.into_inner())),
        (Float(l), Integer(r)) => Ok(Value::from(l.into_inner() + *r as f64)),
        (Integer(l), Float(r)) => Ok(Value::from(*l as f64 + r.into_inner())),
        (Integer(l), Integer(r)) => Ok(Value::from(l + r)),
        _ => Err(ERROR_MESSAGE.into()),
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MezmoConcatOrAddFallible;

impl Function for MezmoConcatOrAddFallible {
    fn identifier(&self) -> &'static str {
        "mezmo_concat_or_add_fallible"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                keyword: "left",
                kind: kind::ANY,
                required: true,
            },
            Parameter {
                keyword: "right",
                kind: kind::ANY,
                required: true,
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
        let left = arguments.required("left");
        let right = arguments.required("right");

        Ok(MezmoConcatOrAddFallibleFn { left, right }.as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoConcatOrAddFallibleFn {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl FunctionExpression for MezmoConcatOrAddFallibleFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let left = self.left.resolve(ctx)?;
        let right = self.right.resolve(ctx)?;
        concat_or_add(left, right)
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::bytes().or_integer().or_float().fallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_function![
        mezmo_concat_or_add_fallible => MezmoConcatOrAddFallible;

        integer {
            args: func_args![left: 1, right: 2],
            want: Ok(3),
            tdef: TypeDef::bytes().or_integer().or_float().fallible(),
        }

        float {
            args: func_args![left: 1.3, right: 8.6],
            want: Ok(9.9),
            tdef: TypeDef::bytes().or_integer().or_float().fallible(),
        }

        float_integer {
            args: func_args![left: 1.2, right: 2],
            want: Ok(3.2),
            tdef: TypeDef::bytes().or_integer().or_float().fallible(),
        }

        integer_float {
            args: func_args![left: 1, right: 2.9],
            want: Ok(3.9),
            tdef: TypeDef::bytes().or_integer().or_float().fallible(),
        }

        string {
            args: func_args![left: "abc", right: "d"],
            want: Ok("abcd"),
            tdef: TypeDef::bytes().or_integer().or_float().fallible(),
        }

        string_and_integer {
            args: func_args![left: "$ ", right: 1],
            want: Ok("$ 1"),
            tdef: TypeDef::bytes().or_integer().or_float().fallible(),
        }

        float_string {
            args: func_args![left: 123.45, right: " €"],
            want: Ok("123.45 €"),
            tdef: TypeDef::bytes().or_integer().or_float().fallible(),
        }

        string_null {
            args: func_args![left: "abc", right: value!(null)],
            want: Ok("abcnull"),
            tdef: TypeDef::bytes().or_integer().or_float().fallible(),
        }

        integer_null {
            args: func_args![left: 1, right: value!(null)],
            want: Err(ERROR_MESSAGE),
            tdef: TypeDef::bytes().or_integer().or_float().fallible(),
        }

        float_boolean {
            args: func_args![left: 1.1, right: true],
            want: Err(ERROR_MESSAGE),
            tdef: TypeDef::bytes().or_integer().or_float().fallible(),
        }
    ];
}
