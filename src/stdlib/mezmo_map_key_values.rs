use crate::compiler::prelude::*;

/// Implements depth first iteration of the keys and values of an object or the values of an array
///
/// Key points:
/// - Recursion depth is controlled via `max_depth`:
///   - -1: infinite recursion
///   -  0: no recursion (only current level)
///   -  N > 0: recurse up to N levels deeper
/// - Uses Value::into_iter(false) to process items at the current depth
/// - Recurses into nested objects/arrays up to the specified depth
/// - Mutates keys/values in-place
/// - Key collision behavior is based on the sort order of the BTreeMap.
///   - Value is a BTreeMap; keys are sorted
///   - On collision, overwrite is based on which key is last in the list
///   - See lib/tests/tests/mezmo/mezmo_map_key_values/key_collision_overwrite.vrl for an
///     example of how this may differ from a caller's expectations
/// - The overhead of this function is the same as using `Value::into_iter(true).by_ref()`
///   and collecting the final results as used in map_keys and map_values
fn mezmo_map_key_values<T>(
    value: Value,
    max_depth: i64,
    ctx: &mut Context,
    runner: &closure::Runner<T>,
) -> Resolved
where
    T: Fn(&mut Context) -> Resolved,
{
    let mut value_clone = value.clone();
    match map_key_values(&mut value_clone, max_depth, ctx, runner) {
        Ok(_) => Ok(value_clone),
        _ => Ok(value),
    }
}

/// Recursively transform `value` in-place up to `remaining_depth`
fn map_key_values<T>(
    value: &mut Value,
    remaining_depth: i64,
    ctx: &mut Context,
    runner: &closure::Runner<T>,
) -> Result<(), ExpressionError>
where
    T: Fn(&mut Context) -> Resolved,
{
    if !value.is_object() && !value.is_array() {
        return Ok(());
    }

    // Take ownership so we can build a non-recursive iterator over the current level.
    // into_iter requires an owned object
    let current = core::mem::replace(value, Value::Null);
    let mut iter = current.into_iter(false);

    for item in iter.by_ref() {
        match item {
            IterItem::KeyValue(key, val) => {
                if can_recurse(remaining_depth) && (val.is_object() || val.is_array()) {
                    map_key_values(val, next_depth(remaining_depth), ctx, runner)?;
                }

                let result = runner.run_key_value(ctx, key.as_str(), &*val)?;
                apply_result(result, Some(key), val)?;
            }
            IterItem::IndexValue(index, val) => {
                if can_recurse(remaining_depth) && (val.is_object() || val.is_array()) {
                    map_key_values(val, next_depth(remaining_depth), ctx, runner)?;
                }

                let result = runner.run_index_value(ctx, index, &*val)?;
                apply_result(result, None, val)?;
            }
            IterItem::Value(val) => {
                if can_recurse(remaining_depth) && (val.is_object() || val.is_array()) {
                    map_key_values(val, next_depth(remaining_depth), ctx, runner)?;
                }

                let result = runner.run_index_value(ctx, 0, &*val)?;
                apply_result(result, None, val)?;
            }
        }
    }

    *value = iter.into();
    Ok(())
}

#[inline]
fn can_recurse(remaining_depth: i64) -> bool {
    remaining_depth > 0
}

#[inline]
fn next_depth(remaining_depth: i64) -> i64 {
    if remaining_depth == i64::MAX {
        i64::MAX
    } else {
        remaining_depth - 1
    }
}

/// Apply a closure result of the form `[key, value]` to the in-place slots.
///
/// - If `key_slot` is provided (object key), we attempt to coerce the returned
///   key to UTF-8 and mutate the key in-place.
/// - If key coercion fails, we keep the original key and just update the value.
/// - If the result is not a 2-element array, or not an array at all, we ignore it.
fn apply_result(
    result: Value,
    key_slot: Option<&mut KeyString>,
    value_slot: &mut Value,
) -> Result<(), ExpressionError> {
    let Value::Array(elements) = result else {
        return Ok(());
    };

    if elements.len() != 2 {
        return Ok(());
    }

    let mut iter = elements.into_iter();
    let result_key = iter.next().expect("must have a key entry");
    let result_value = iter.next().expect("must have a value entry");

    if let Some(key) = key_slot {
        if let Ok(new_key) = result_key.try_bytes_utf8_lossy() {
            if !new_key.trim().is_empty() {
                *key = new_key.into_owned().into();
            }
        }
    }

    *value_slot = result_value;

    Ok(())
}

#[derive(Clone, Copy, Debug)]
pub struct MezmoMapKeyValues;

impl Function for MezmoMapKeyValues {
    fn identifier(&self) -> &'static str {
        "mezmo_map_key_values"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                keyword: "value",
                kind: kind::OBJECT | kind::ARRAY,
                required: true,
            },
            Parameter {
                keyword: "max_depth",
                kind: kind::INTEGER,
                required: false,
            },
        ]
    }

    fn examples(&self) -> &'static [Example] {
        &[
            Example {
                title: "rename keys and values",
                source: r#"mezmo_map_key_values({ "Foo": "bar" }) -> |key, value| { [ downcase(key), upcase(value) ] }"#,
                result: Ok(r#"{ "foo": "BAR" }"#),
            },
            Example {
                title: "infinite recursion",
                source: r#"mezmo_map_key_values({ 
                    "a": { "b": 1 }, 
                    "c": [ { "d": 2 }, "x", ["nested arr"] ],
                    "labels": [ "prod" ] 
                }, max_depth: -1) -> |key, value| {
                    new_key = key
                    new_value = if is_integer(value) { int!(value) + 1 } else if is_string(value) { upcase(string!(value)) } else { value }
                    [ new_key, new_value ]
                }"#,
                result: Ok(
                    r#"{ "a": { "b": 2 }, "c": [ { "d": 3 }, "X", ["NESTED ARR"] ], "labels": ["PROD"] }"#,
                ),
            },
            Example {
                title: "limit recursion to one level",
                source: r#"mezmo_map_key_values({ 
                    "a": { "b": 1 }, 
                    "c": [ { "d": 2 }, "x", ["not updated"] ],
                    "labels": [ "prod" ]
                }, max_depth: 1) -> |key, value| {
                    new_key = key
                    new_value = if is_integer(value) { int!(value) + 1 } else if is_string(value) { upcase(string!(value)) } else { value }
                    [ new_key, new_value ]
                }"#,
                result: Ok(
                    r#"{ "a": { "b": 2 }, "c": [ { "d": 2 }, "X", ["not updated"] ], "labels": ["PROD"] }"#,
                ),
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
        let max_depth = arguments.optional("max_depth");
        let closure = arguments.required_closure()?;

        Ok(MezmoMapKeyValuesFn {
            value,
            max_depth,
            closure,
        }
        .as_expr())
    }

    fn closure(&self) -> Option<closure::Definition> {
        use closure::{Definition, Input, Output, Variable, VariableKind};

        Some(Definition {
            inputs: vec![Input {
                parameter_keyword: "value",
                kind: Kind::object(Collection::any()).or_array(Collection::any()),
                variables: vec![
                    Variable {
                        kind: VariableKind::TargetInnerKey,
                    },
                    Variable {
                        kind: VariableKind::TargetInnerValue,
                    },
                ],
                output: Output::Array {
                    elements: vec![Kind::integer().or_bytes(), Kind::any()],
                },
                example: Example {
                    title: "rename keys and values",
                    source: r#"mezmo_map_key_values({ "Foo": "bar" }) -> |key, value| { [ downcase(key), upcase(value) ] }"#,
                    result: Ok(r#"{ "foo": "BAR" }"#),
                },
            }],
            is_iterator: true,
        })
    }
}

#[derive(Debug, Clone)]
struct MezmoMapKeyValuesFn {
    value: Box<dyn Expression>,
    max_depth: Option<Box<dyn Expression>>,
    closure: Closure,
}

impl FunctionExpression for MezmoMapKeyValuesFn {
    fn resolve(&self, ctx: &mut Context) -> ExpressionResult<Value> {
        // Default to 0 when max_depth is not provided. Translate -1 to infinite (i64::MAX).
        let max_depth = if let Some(expr) = &self.max_depth {
            let d = expr.resolve(ctx)?.try_integer()?;
            if d == -1 { i64::MAX } else { d }
        } else {
            0
        };

        let value = self.value.resolve(ctx)?;
        let Closure {
            variables,
            block,
            block_type_def: _,
        } = &self.closure;
        let runner = closure::Runner::new(variables, |ctx| block.resolve(ctx));

        mezmo_map_key_values(value, max_depth, ctx, &runner)
    }

    fn type_def(&self, ctx: &state::TypeState) -> TypeDef {
        self.value.type_def(ctx)
    }
}

// Tests for this function are located in VRL test files at:
// lib/tests/tests/mezmo/mezmo_map_key_values/
