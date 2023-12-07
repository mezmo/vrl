use crate::compiler::prelude::*;

fn to_object(value: Value) -> Resolved {
    use Value::Object;

    match value {
        Object(value) => Ok(Object(value)),
        v => Err(format!("unable to coerce {} into object", v.kind()).into()),
    }
}

/// Equivalent as the stdlib `object()` with non-conditional fallibility.
#[derive(Clone, Copy, Debug)]
pub struct MezmoToObject;

impl Function for MezmoToObject {
    fn identifier(&self) -> &'static str {
        "mezmo_to_object"
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
        to_object(value)
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::object(Collection::any()).fallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value;

    test_function![
        mezmo_to_object => MezmoToObject;

        integer {
            args: func_args![value: 20],
            want: Err("unable to coerce integer into object"),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        array {
            args: func_args![value: value!({hello: "object"})],
            want: Ok(value!({hello: "object"})),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        null {
            args: func_args![value: value!(null)],
            want: Err("unable to coerce null into object"),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }
    ];
}
