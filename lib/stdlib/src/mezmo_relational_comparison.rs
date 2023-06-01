use ::value::Value;
use vrl::prelude::*;
use vrl::value::Error;

type ComparisonFn = fn(Value, Value) -> bool;

fn gt(left: Value, right: Value) -> bool {
    value_to_boolean(left.try_gt(right))
}

fn gte(left: Value, right: Value) -> bool {
    value_to_boolean(left.try_ge(right))
}

fn lt(left: Value, right: Value) -> bool {
    value_to_boolean(left.try_lt(right))
}

fn lte(left: Value, right: Value) -> bool {
    value_to_boolean(left.try_le(right))
}

fn value_to_boolean(value: std::result::Result<Value, Error>) -> bool {
    use Value::Boolean;

    match value.unwrap_or_else(|_| Boolean(false)) {
        Boolean(v) => v,
        _ => false,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MezmoGt;

#[derive(Clone, Copy, Debug)]
pub struct MezmoGte;

#[derive(Clone, Copy, Debug)]
pub struct MezmoLt;

#[derive(Clone, Copy, Debug)]
pub struct MezmoLte;

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

                Ok(RelationalOperatorFn {
                    left,
                    right,
                    comparison_fn: $fn_name,
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

implement_function!(MezmoGt, "mezmo_gt", gt);
implement_function!(MezmoGte, "mezmo_gte", gte);
implement_function!(MezmoLt, "mezmo_lt", lt);
implement_function!(MezmoLte, "mezmo_lte", lte);

#[derive(Debug, Clone)]
struct RelationalOperatorFn {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
    comparison_fn: ComparisonFn,
}

impl FunctionExpression for RelationalOperatorFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let left = self.left.resolve(ctx)?;
        let right = self.right.resolve(ctx)?;
        Ok((self.comparison_fn)(left, right).into())
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::boolean().infallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_function![
        mezmo_gt => MezmoGt;

        integer {
            args: func_args![left: 20, right: 5],
            want: Ok(true),
            tdef: TypeDef::boolean().infallible(),
        }

        integer_float {
            args: func_args![left: 20, right: 100.23],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }

        float {
            args: func_args![left: 20.5, right: 0.0],
            want: Ok(true),
            tdef: TypeDef::boolean().infallible(),
        }

        float_equal {
            args: func_args![left: 1.1, right: 1.1],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }

        float_less_than {
            args: func_args![left: 1.1, right: 100.1],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }

        float_null {
            args: func_args![left: 1.1, right: value!(null)],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }
    ];

    test_function![
        mezmo_gte => MezmoGte;

        integer {
            args: func_args![left: 20, right: 5],
            want: Ok(true),
            tdef: TypeDef::boolean().infallible(),
        }

        integer_float {
            args: func_args![left: 20.1, right: 100],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }

        float {
            args: func_args![left: 20.5, right: 0.0],
            want: Ok(true),
            tdef: TypeDef::boolean().infallible(),
        }

        float_less_than {
            args: func_args![left: 1.1, right: 100.1],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }

        float_equal {
            args: func_args![left: 1.1, right: 1.1],
            want: Ok(true),
            tdef: TypeDef::boolean().infallible(),
        }

        integer_null {
            args: func_args![left: value!(null), right: 2],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }

        integer_array {
            args: func_args![left: value!([1, 2]), right: 2],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }
    ];

    test_function![
        mezmo_lt => MezmoLt;

        integer_float {
            args: func_args![left: 20, right: 100.23],
            want: Ok(true),
            tdef: TypeDef::boolean().infallible(),
        }

        integer_greater_than {
            args: func_args![left: 20, right: 5],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }

        integer_less_than {
            args: func_args![left: 0, right: 5],
            want: Ok(true),
            tdef: TypeDef::boolean().infallible(),
        }

        float_equal {
            args: func_args![left: 1.1, right: 1.1],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }

        object_left {
            args: func_args![left: value!({foo: "bar"}), right: 1],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }

        object_right {
            args: func_args![left: 1.1, right: value!({foo: "bar"})],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }
    ];

    test_function![
        mezmo_lte => MezmoLte;

        integer_float {
            args: func_args![left: 20, right: 100.23],
            want: Ok(true),
            tdef: TypeDef::boolean().infallible(),
        }

        integer_float_greater_than {
            args: func_args![left: 20.1, right: 5],
            want: Ok(false),
            tdef: TypeDef::boolean().infallible(),
        }

        integer_less_than {
            args: func_args![left: 0, right: 5],
            want: Ok(true),
            tdef: TypeDef::boolean().infallible(),
        }

        float_equal {
            args: func_args![left: 1.1, right: 1.1],
            want: Ok(true),
            tdef: TypeDef::boolean().infallible(),
        }
    ];
}
