use super::mezmo_concat_or_add_fallible::concat_or_add;
use crate::compiler::prelude::*;

#[deprecated(
    note = "This function uses conditional fallibility, use MezmoConcatOrAddFallible instead"
)]
#[derive(Clone, Copy, Debug)]
pub struct MezmoConcatOrAdd;

/// MezmoConcatOrAdd is deprecated, but usage may still exist in configs.
#[allow(deprecated)]
impl Function for MezmoConcatOrAdd {
    fn identifier(&self) -> &'static str {
        "mezmo_concat_or_add"
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

        Ok(MezmoConcatOrAddFn { left, right }.as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoConcatOrAddFn {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl FunctionExpression for MezmoConcatOrAddFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let left = self.left.resolve(ctx)?;
        let right = self.right.resolve(ctx)?;
        concat_or_add(left, right)
    }

    fn type_def(&self, state: &state::TypeState) -> TypeDef {
        let left_type = self.left.type_def(state);
        let right_type = self.right.type_def(state);
        if left_type.is_bytes() || right_type.is_bytes() {
            return TypeDef::bytes().infallible();
        } else if is_numeric(&left_type) && is_numeric(&right_type) {
            return if left_type.is_integer() && right_type.is_integer() {
                TypeDef::integer().infallible()
            } else {
                TypeDef::float().infallible()
            };
        }
        TypeDef::bytes().or_integer().or_float().fallible()
    }
}

fn is_numeric(def: &TypeDef) -> bool {
    return def.is_integer() || def.is_float();
}

#[allow(deprecated)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::mezmo_concat_or_add_fallible::ERROR_MESSAGE;
    use crate::value;

    test_function![
        mezmo_concat_or_add => MezmoConcatOrAdd;

        integer {
            args: func_args![left: 1, right: 2],
            want: Ok(3),
            tdef: TypeDef::integer().infallible(),
        }

        float {
            args: func_args![left: 1.3, right: 8.6],
            want: Ok(9.9),
            tdef: TypeDef::float().infallible(),
        }

        float_integer {
            args: func_args![left: 1.2, right: 2],
            want: Ok(3.2),
            tdef: TypeDef::float().infallible(),
        }

        integer_float {
            args: func_args![left: 1, right: 2.9],
            want: Ok(3.9),
            tdef: TypeDef::float().infallible(),
        }

        string {
            args: func_args![left: "abc", right: "d"],
            want: Ok("abcd"),
            tdef: TypeDef::bytes().infallible(),
        }

        string_and_integer {
            args: func_args![left: "$ ", right: 1],
            want: Ok("$ 1"),
            tdef: TypeDef::bytes().infallible(),
        }

        float_string {
            args: func_args![left: 123.45, right: " €"],
            want: Ok("123.45 €"),
            tdef: TypeDef::bytes().infallible(),
        }

        string_null {
            args: func_args![left: "abc", right: value!(null)],
            want: Ok("abcnull"),
            tdef: TypeDef::bytes().infallible(),
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
