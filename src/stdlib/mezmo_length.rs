use crate::compiler::prelude::*;

/// Returns the string length (utf8), the array length or the property "length" value
fn mezmo_length(value: Value) -> Resolved {
    Ok(match value {
        Value::Bytes(text) => String::from_utf8_lossy(&text).chars().count().into(),
        Value::Object(obj) => obj.get("length").cloned().unwrap_or(Value::Null),
        Value::Array(arr) => arr.len().into(),
        _ => Value::Null,
    })
}

#[derive(Clone, Copy, Debug)]
pub struct MezmoLength;

impl Function for MezmoLength {
    fn identifier(&self) -> &'static str {
        "mezmo_length"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[Parameter {
            keyword: "value",
            kind: kind::ANY,
            required: true,
        }]
    }

    fn examples(&self) -> &'static [Example] {
        &[Example {
            title: "Characters",
            source: r#"mezmo_length("ñandú")"#,
            result: Ok("5"),
        }]
    }

    fn compile(
        &self,
        _state: &TypeState,
        _ctx: &mut FunctionCompileContext,
        arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");

        Ok(MezmoLengthFn { value }.as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoLengthFn {
    value: Box<dyn Expression>,
}

impl FunctionExpression for MezmoLengthFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;

        mezmo_length(value)
    }

    fn type_def(&self, _state: &TypeState) -> TypeDef {
        TypeDef::any().infallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value;

    test_function![
        mezmo_length => MezmoLength;

        string_value {
            args: func_args![value: value!("ñandú")],
            want: Ok(value!(5)),
            tdef: TypeDef::any().infallible(),
        }

        non_empty_array_value {
            args: func_args![value: value!([1, 2, 3, 4, true, "hello"])],
            want: Ok(value!(6)),
            tdef: TypeDef::any().infallible(),
        }

        empty_array_value {
            args: func_args![value: value!([])],
            want: Ok(value!(0)),
            tdef: TypeDef::any().infallible(),
        }

        object_value_without_length_property {
            args: func_args![value: value!({hello: "world"})],
            want: Ok(value!(null)),
            tdef: TypeDef::any().infallible(),
        }

        object_value_with_length_property {
            args: func_args![value: value!({"length": "my length"})],
            want: Ok(value!("my length")),
            tdef: TypeDef::any().infallible(),
        }

        number {
            args: func_args![value: value!(123)],
            want: Ok(value!(null)),
            tdef: TypeDef::any().infallible(),
        }
    ];
}
