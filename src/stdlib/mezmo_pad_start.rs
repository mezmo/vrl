use std::borrow::Cow;

use crate::compiler::prelude::*;

fn pad_start(value: Cow<'_, str>, target_length: i64, pad_value: Cow<'_, str>) -> Value {
    let current_length = value.chars().count();
    let target_length = if target_length < 0 {
        0
    } else {
        target_length as usize
    };

    if target_length > current_length {
        let pad_length = target_length - current_length;
        let mut pad = pad_value
            .chars()
            .cycle()
            .take(pad_length)
            .collect::<String>();
        pad.push_str(&value);
        pad.into()
    } else {
        value.into()
    }
}

/// Pads a string with the provided pad string, possibly multiple times, until
/// it reaches the given target length. The string is padded from the start.
///
/// Behaves like the JavaScript's String.prototype.padStart() method.
#[derive(Clone, Copy, Debug)]
pub struct MezmoPadStart;

impl Function for MezmoPadStart {
    fn identifier(&self) -> &'static str {
        "mezmo_pad_start"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                keyword: "value",
                kind: kind::BYTES,
                required: true,
            },
            Parameter {
                keyword: "target_length",
                kind: kind::INTEGER,
                required: true,
            },
            Parameter {
                keyword: "pad_value",
                kind: kind::BYTES,
                required: false,
            },
        ]
    }

    fn examples(&self) -> &'static [Example] {
        &[
            Example {
                title: "basic",
                source: "mezmo_pad_start(\"abc\", 6)",
                result: Ok("    abc"),
            },
            Example {
                title: "with_value",
                source: "mezmo_pad_start(\"abc\", 6, \"def\")",
                result: Ok("defabc"),
            },
        ]
    }

    fn compile(
        &self,
        _state: &state::TypeState,
        _ctx: &mut FunctionCompileContext,
        arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");
        let target_length = arguments.required("target_length");
        let pad_value = arguments.optional("pad_value");

        Ok(MezmoPadStartFn {
            value,
            target_length,
            pad_value,
        }
        .as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoPadStartFn {
    value: Box<dyn Expression>,
    target_length: Box<dyn Expression>,
    pad_value: Option<Box<dyn Expression>>,
}

impl FunctionExpression for MezmoPadStartFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        let target_length = self.target_length.resolve(ctx)?;
        let pad_value = match &self.pad_value {
            Some(pv) => pv.resolve(ctx)?,
            None => " ".into(),
        };
        Ok(pad_start(
            value.try_bytes_utf8_lossy()?,
            target_length.try_integer()?,
            pad_value.try_bytes_utf8_lossy()?,
        ))
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::bytes().infallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_function![
        mezmo_pad_start => MezmoPadStart;

        basic {
            args: func_args![value: "abc", target_length: 6],
            want: Ok("   abc"),
            tdef: TypeDef::bytes().infallible(),
        }

        repeat_pad {
            args: func_args![value: "abc", target_length: 10, pad_value: "foo"],
            want: Ok("foofoofabc"),
            tdef: TypeDef::bytes().infallible(),
        }

        negative_target_length {
            args: func_args![value: "abc", target_length: -10, pad_value: "foo"],
            want: Ok("abc"),
            tdef: TypeDef::bytes().infallible(),
        }

        empty {
            args: func_args![value: "", target_length: 10, pad_value: ""],
            want: Ok(""),
            tdef: TypeDef::bytes().infallible(),
        }

        pad_with_empty {
            args: func_args![value: "abc", target_length: 10, pad_value: ""],
            want: Ok("abc"),
            tdef: TypeDef::bytes().infallible(),
        }
    ];
}
