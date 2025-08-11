use super::to_string;
use crate::compiler::prelude::*;

/// Converts any value into a string.
/// Returns "[Array]" for arrays, "[Object]" for objects and "" for nulls.
pub(crate) fn to_string(value: Value) -> String {
    use Value::{Array, Bytes, Null, Object};
    match value {
        Array(_) => "[Array]".into(),
        Object(_) => "[Object]".into(),
        Null => "null".into(),
        _ => {
            let bytes = to_string::to_string(value).unwrap_or_else(|_| Value::from(""));
            match bytes {
                Bytes(v) => std::str::from_utf8(&v).unwrap_or("").into(),
                _ => "".into(),
            }
        }
    }
}

/// Infallible counterpart of ToString
#[derive(Clone, Copy, Debug)]
pub struct MezmoToString;

impl Function for MezmoToString {
    fn identifier(&self) -> &'static str {
        "mezmo_to_string"
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
            title: "float",
            source: "mezmo_to_string(5.6)",
            result: Ok("5.6"),
        }]
    }

    fn compile(
        &self,
        _state: &state::TypeState,
        _ctx: &mut FunctionCompileContext,
        arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");

        Ok(MezmoToStringFn { value }.as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoToStringFn {
    value: Box<dyn Expression>,
}

impl FunctionExpression for MezmoToStringFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        Ok(to_string(value).into())
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::bytes().infallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value;
    use chrono::{TimeZone, Utc};

    test_function![
        mezmo_to_string => MezmoToString;

        float {
            args: func_args![value: 20.5],
            want: Ok("20.5"),
            tdef: TypeDef::bytes().infallible(),
        }

        integer {
            args: func_args![value: 0],
            want: Ok("0"),
            tdef: TypeDef::bytes().infallible(),
        }

        integer_negative {
            args: func_args![value: -111],
            want: Ok("-111"),
            tdef: TypeDef::bytes().infallible(),
        }

        integer_string {
            args: func_args![value: "my string"],
            want: Ok("my string"),
            tdef: TypeDef::bytes().infallible(),
        }

        null {
            args: func_args![value: value!(null)],
            want: Ok("null"),
            tdef: TypeDef::bytes().infallible(),
        }

        array {
            args: func_args![value: value!([1, 2])],
            want: Ok("[Array]"),
            tdef: TypeDef::bytes().infallible(),
        }

        object {
            args: func_args![value: value!({hello: 1})],
            want: Ok("[Object]"),
            tdef: TypeDef::bytes().infallible(),
        }

        timestamp {
            args: func_args![value: Utc.with_ymd_and_hms(2021, 1, 1, 5, 12, 0).unwrap()],
            want: Ok("2021-01-01T05:12:00Z"),
            tdef: TypeDef::bytes().infallible(),
        }

        boolean {
            args: func_args![value: false],
            want: Ok("false"),
            tdef: TypeDef::bytes().infallible(),
        }
    ];
}
