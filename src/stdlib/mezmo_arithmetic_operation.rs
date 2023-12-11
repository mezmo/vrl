use crate::compiler::prelude::*;

const ERROR_MESSAGE: &str = "Invalid arithmetic operation";

type ArithmeticFn = fn(Value, Value) -> Resolved;

fn subtract(left: Value, right: Value) -> Resolved {
    left.try_sub(right).map_err(|_| ERROR_MESSAGE.into())
}

fn multiply(left: Value, right: Value) -> Resolved {
    left.try_mul(right).map_err(|_| ERROR_MESSAGE.into())
}

fn divide(left: Value, right: Value) -> Resolved {
    use Value::Integer;
    match (&left, &right) {
        // left.try_div() will always convert to f64 which is not correct for ints
        (Integer(l), Integer(r)) => {
            if *r == 0 {
                return Err(ERROR_MESSAGE.into());
            }
            Ok(Value::from(l / r))
        }
        _ => left.try_div(right).map_err(|_| ERROR_MESSAGE.into()),
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MezmoSubtract;

#[derive(Clone, Copy, Debug)]
pub struct MezmoMultiply;

#[derive(Clone, Copy, Debug)]
pub struct MezmoDivide;

macro_rules! implement_function {
    ($type_name: ident, $identifier: expr, $fn_name: ident) => {
        impl Function for $type_name {
            fn identifier(&self) -> &'static str {
                $identifier
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

                Ok(ArithmeticOperatorFn {
                    left,
                    right,
                    arithmetic_fn: $fn_name,
                }
                .as_expr())
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
        }
    };
}

implement_function!(MezmoSubtract, "mezmo_sub", subtract);
implement_function!(MezmoMultiply, "mezmo_mul", multiply);
implement_function!(MezmoDivide, "mezmo_div", divide);

#[derive(Debug, Clone)]
struct ArithmeticOperatorFn {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
    arithmetic_fn: ArithmeticFn,
}

impl FunctionExpression for ArithmeticOperatorFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let left = self.left.resolve(ctx)?;
        let right = self.right.resolve(ctx)?;
        (self.arithmetic_fn)(left, right)
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::integer().or_float().fallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value;

    test_function![
        mezmo_sub => MezmoSubtract;

        integer {
            args: func_args![left: 20, right: 5],
            want: Ok(15),
            tdef: TypeDef::integer().or_float().fallible(),
        }

        float {
            args: func_args![left: 20.5, right: 6.2],
            want: Ok(14.3),
            tdef: TypeDef::integer().or_float().fallible(),
        }

        integer_float {
            args: func_args![left: 20, right: 5.5],
            want: Ok(14.5),
            tdef: TypeDef::integer().or_float().fallible(),
        }

        float_integer {
            args: func_args![left: 20.5, right: 5],
            want: Ok(15.5),
            tdef: TypeDef::integer().or_float().fallible(),
        }

        float_object {
            args: func_args![left: 20.5, right: value!({hello: 1})],
            want: Err(ERROR_MESSAGE),
            tdef: TypeDef::integer().or_float().fallible(),
        }
    ];

    test_function![
        mezmo_mul => MezmoMultiply;

        integer {
            args: func_args![left: 20, right: 5],
            want: Ok(100),
            tdef: TypeDef::integer().or_float().fallible(),
        }

        float {
            args: func_args![left: 0.5, right: 4.2],
            want: Ok(2.1),
            tdef: TypeDef::integer().or_float().fallible(),
        }

        integer_float {
            args: func_args![left: 2, right: 5.5],
            want: Ok(11.0),
            tdef: TypeDef::integer().or_float().fallible(),
        }

        float_integer {
            args: func_args![left: 20.1, right: 5],
            want: Ok(100.5),
            tdef: TypeDef::integer().or_float().fallible(),
        }

        integer_object {
            args: func_args![left: 1, right: value!({hello: 1})],
            want: Err(ERROR_MESSAGE),
            tdef: TypeDef::integer().or_float().fallible(),
        }
    ];

    test_function![
        mezmo_div => MezmoDivide;

        integer {
            args: func_args![left: 20, right: 5],
            want: Ok(4),
            tdef: TypeDef::integer().or_float().fallible(),
        }

        float {
            args: func_args![left: 100, right: 4.8],
            want: Ok(20.833333333333336),
            tdef: TypeDef::integer().or_float().fallible(),
        }

        integer_float {
            args: func_args![left: 10.1, right: 0.5],
            want: Ok(20.2),
            tdef: TypeDef::integer().or_float().fallible(),
        }

        float_integer {
            args: func_args![left: 20.5, right: 5],
            want: Ok(4.1),
            tdef: TypeDef::integer().or_float().fallible(),
        }

        integer_null {
            args: func_args![left: 1, right: value!(null)],
            want: Err(ERROR_MESSAGE),
            tdef: TypeDef::integer().or_float().fallible(),
        }

        integer_zero {
            args: func_args![left: 1, right: 0],
            want: Err(ERROR_MESSAGE),
            tdef: TypeDef::integer().or_float().fallible(),
        }

        float_zero {
            args: func_args![left: 1.1, right: 0.0],
            want: Err(ERROR_MESSAGE),
            tdef: TypeDef::integer().or_float().fallible(),
        }
    ];
}
