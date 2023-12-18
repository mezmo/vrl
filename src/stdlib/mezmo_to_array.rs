use crate::compiler::prelude::*;

fn to_array(value: Value) -> Resolved {
    use Value::Array;

    match value {
        Array(value) => Ok(Array(value)),
        v => Err(format!("unable to coerce {} into array", v.kind()).into()),
    }
}

/// Equivalent as the stdlib `array()` with non-conditional fallibility.
#[derive(Clone, Copy, Debug)]
pub struct MezmoToArray;

impl Function for MezmoToArray {
    fn identifier(&self) -> &'static str {
        "mezmo_to_array"
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
        to_array(value)
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::array(Collection::any()).fallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value;

    test_function![
        mezmo_to_array => MezmoToArray;

        float {
            args: func_args![value: 20.5],
            want: Err("unable to coerce float into array"),
            tdef: TypeDef::array(Collection::any()).fallible(),
        }

        array {
            args: func_args![value: value!([1, 2, 3])],
            want: Ok(value!([1, 2, 3])),
            tdef: TypeDef::array(Collection::any()).fallible(),
        }

        null {
            args: func_args![value: value!(null)],
            want: Err("unable to coerce null into array"),
            tdef: TypeDef::array(Collection::any()).fallible(),
        }
    ];
}
