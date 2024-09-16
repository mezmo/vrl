use crate::compiler::prelude::*;

// Add some overhead to the array and object size
// The overhead here is carried over from the implementation of usage metrics
// to ensure alignment of values
const BASE_ARRAY_SIZE: usize = 8;
const BASE_BTREE_SIZE: usize = 8;

// Implementation borrows from usage_metrics computation in vector source
// See: https://github.com/answerbook/vector/blob/c6c061b225008047b508f8215226b508f7686549/lib/vector-core/src/usage_metrics/mod.rs#L557
fn value_size(value: &Value) -> usize {
    let res = match value {
        Value::Bytes(v) => v.len(),
        Value::Boolean(_) => 1,
        Value::Timestamp(_) | Value::Integer(_) | Value::Float(_) => 8,
        Value::Regex(v) => v.as_str().len(),
        Value::Object(v) => {
            BASE_BTREE_SIZE
                + v.iter()
                    .map(|(k, v)| k.len() + value_size(v))
                    .sum::<usize>()
        }
        Value::Array(v) => BASE_ARRAY_SIZE + v.iter().map(value_size).sum::<usize>(),
        Value::Null => 0, // No value, just the type definition
    };

    res
}

#[derive(Clone, Copy, Debug)]
pub struct MezmoValueSize;

impl Function for MezmoValueSize {
    fn identifier(&self) -> &'static str {
        "mezmo_value_size"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[Parameter {
            keyword: "value",
            kind: kind::ANY,
            required: true,
        }]
    }

    fn examples(&self) -> &'static [Example] {
        &[
            Example {
                title: "metrics for string",
                source: r#"mezmo_value_size("foobar")"#,
                result: Ok("{count: 1, bytes: 14, chars: 10}"),
            },
            Example {
                title: "metrics for object",
                source: r#"mezmo_value_size({"name": "jon doe", "department": "sales", "notes": {"entry" => "foobar works"}}, field_metrics: true)"#,
                result: Ok("{count: 1, bytes: 14, chars: 12}"),
            },
        ]
    }

    fn compile(
        &self,
        _: &TypeState,
        _: &mut FunctionCompileContext,
        arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");
        Ok(MezmoValueSizeFn { value }.as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoValueSizeFn {
    value: Box<dyn Expression>,
}

impl FunctionExpression for MezmoValueSizeFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        Ok(value_size(&value).into())
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::integer().infallible()
    }
}

#[cfg(test)]
#[allow(clippy::trivial_regex)]
mod tests {
    use super::*;
    use crate::value;
    use chrono::Utc;

    test_function![
        value_size => MezmoValueSize;

        byte_value {
            args: func_args![value: Bytes::from("foobar")],
            want: Ok(value!(6)),
            tdef: TypeDef::integer().infallible(),
        }

        integer_value {
            args: func_args![value: 10],
            want: Ok(value!(8)),
            tdef: TypeDef::integer().infallible(),
        }

        float_value {
            args: func_args![value: 15.2],
            want: Ok(value!(8)),
            tdef: TypeDef::integer().infallible(),
        }

        bool_value {
            args: func_args![value: true],
            want: Ok(value!(1)),
            tdef: TypeDef::integer().infallible(),
        }

        timestamp_value {
            args: func_args![value: Utc::now()],
            want: Ok(value!(8)),
            tdef: TypeDef::integer().infallible(),
        }

        object_value {
            args: func_args![
                value: value!({
                    "name": "ben johnson",
                    "age": 20,
                    "balance": 200.52,
                })],
            want: Ok(value!(49)),
            tdef: TypeDef::integer().infallible(),
        }

        array_value {
            args: func_args![
                value: value!([
                    {
                        "name": "ben johnson",
                        "age": 20,
                        "balance": 200.52,
                    },
                    "something just happened"
                ])],
            want: Ok(value!(80)),
            tdef: TypeDef::integer().infallible(),
        }

        null_value {
            args: func_args![
                value: Value::Null
            ],
            want: Ok(value!(0)),
            tdef: TypeDef::integer().infallible(),
        }
    ];
}
