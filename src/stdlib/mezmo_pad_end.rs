use crate::compiler::prelude::*;
use std::borrow::Cow;

fn pad_end(value: Cow<'_, str>, target_length: i64, pad_value: Cow<'_, str>) -> Value {
    let current_length = value.chars().count();
    let target_length = if target_length < 0 {
        0
    } else {
        target_length as usize
    };

    if target_length > current_length {
        let pad_length = target_length - current_length;
        let pad = pad_value
            .chars()
            .cycle()
            .take(pad_length)
            .collect::<String>();
        let mut s = value.to_string();
        s.push_str(&pad);
        s.into()
    } else {
        value.into()
    }
}

/// Pads a string with the provided pad string, possibly multiple times, until
/// it reaches the given target length. The string is padded from the end.
///
/// Behaves like the JavaScript's String.prototype.padEnd() method.
#[derive(Clone, Copy, Debug)]
pub struct MezmoPadEnd;

impl Function for MezmoPadEnd {
    fn identifier(&self) -> &'static str {
        "mezmo_pad_end"
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
                source: "mezmo_pad_end(\"abc\", 6)",
                result: Ok("abc   "),
            },
            Example {
                title: "with_value",
                source: "mezmo_pad_end(\"abc\", 6, \"def\")",
                result: Ok("abcdef"),
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

        Ok(MezmoPadEndFn {
            value,
            target_length,
            pad_value,
        }
        .as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoPadEndFn {
    value: Box<dyn Expression>,
    target_length: Box<dyn Expression>,
    pad_value: Option<Box<dyn Expression>>,
}

impl FunctionExpression for MezmoPadEndFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        let target_length = self.target_length.resolve(ctx)?;
        let pad_value = match &self.pad_value {
            Some(pv) => pv.resolve(ctx)?,
            None => " ".into(),
        };
        Ok(pad_end(
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
        mezmo_pad_end => MezmoPadEnd;

        basic {
            args: func_args![value: "abc", target_length: 6],
            want: Ok("abc   "),
            tdef: TypeDef::bytes().infallible(),
        }

        repeat_pad {
            args: func_args![value: "abc", target_length: 10, pad_value: "foo"],
            want: Ok("abcfoofoof"),
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
