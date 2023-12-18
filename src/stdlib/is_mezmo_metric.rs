use crate::compiler::prelude::*;
use crate::value;
use std::borrow::Borrow;

#[derive(Clone, Copy, Debug)]
pub struct IsMezmoMetric;

impl Function for IsMezmoMetric {
    fn identifier(&self) -> &'static str {
        "is_mezmo_metric"
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
            title: "counter",
            source: r#"is_mezmo_metric({"kind":"incremental","name":"metric","value":{"type":"counter","value":1}})"#,
            result: Ok("true"),
        }]
    }

    fn compile(
        &self,
        _state: &state::TypeState,
        _ctx: &mut FunctionCompileContext,
        arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");

        Ok(IsMezmoMetricFn { value }.as_expr())
    }
}

#[derive(Clone, Debug)]
struct IsMezmoMetricFn {
    value: Box<dyn Expression>,
}

trait IsNumber {
    fn is_number(&self) -> bool;
}

impl IsNumber for Value {
    fn is_number(&self) -> bool {
        self.is_float() || self.is_integer()
    }
}

impl FunctionExpression for IsMezmoMetricFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        validate_metric(&value)
    }

    fn type_def(&self, _: &state::TypeState) -> TypeDef {
        TypeDef::boolean().fallible()
    }
}

fn validate_metric(value: &Value) -> Result<Value, ExpressionError> {
    if !value.is_object() {
        return Err("expected an object".into());
    }

    if !value
        .get("name")
        .ok_or_else(|| "field \"name\" not found")?
        .is_bytes()
    {
        return Err("expected field \"name\" to contain a string".into());
    }

    let kind = value
        .get("kind")
        .ok_or_else(|| "field \"kind\" not found")?
        .as_str()
        .ok_or_else(|| "expected field \"kind\" to contain a string")?;

    if kind != "absolute" && kind != "incremental" {
        return Err("expected field \"kind\" to be either \"absolute\" or \"incremental\"".into());
    }

    validate_metric_value(
        value
            .get("value")
            .ok_or_else(|| "field \"value\" not found")?,
    )?;

    // Optional fields

    if let Some(namespace) = value.get("namespace") {
        if !namespace.is_bytes() {
            return Err("expected field \"namespace\" to contain a string".into());
        }
    }

    if let Some(tags) = value.get("tags") {
        let tags = tags
            .as_object()
            .ok_or_else(|| "expected field \"tags\" to contain an array")?;

        for (i, tag) in tags.values().enumerate() {
            if !tag.is_bytes() {
                return Err(format!(
                    "expected field tag at index {i} in \"tags\" to contain a string"
                )
                .into());
            }
        }
    }

    Ok(value!(true))
}

fn validate_metric_value(value: &Value) -> Result<Value, ExpressionError> {
    let value_type = value
        .get("type")
        .ok_or_else(|| "field \"value.type\" not found")?
        .as_str()
        .ok_or_else(|| "expected field \"value.type\" to contain a string")?;

    let value_value = value
        .get("value")
        .ok_or_else(|| "field \"value.value\" not found")?;

    match value_type.borrow() {
        "counter" | "gauge" => validate_counter_or_gauge(value_value),
        "set" => validate_set(value_value),
        "distribution" => validate_distribution(value_value),
        "histogram" => validate_histogram(value_value),
        "summary" => validate_summary(value_value),
        type_name => {
            Err(format!("field \"value.type\" has invalid metric type \"{type_name}\"").into())
        }
    }
}

fn validate_counter_or_gauge(value: &Value) -> Result<Value, ExpressionError> {
    if !value.is_number() {
        return Err(
            "expected counter/gauge metric field \"value.value\" to contain a number".into(),
        );
    }
    Ok(value!(true))
}

fn validate_set(value: &Value) -> Result<Value, ExpressionError> {
    let values = value
        .get("values")
        .ok_or_else(|| "field \"value.value.values\" not found")?
        .as_array()
        .ok_or_else(|| "expected set metric field \"value.value.values\" to contain an array")?;

    for (i, element) in values.iter().enumerate() {
        if !element.is_bytes() {
            return Err(format!("expected set metric field element at index {i} in \"value.value.values\" to contain a string").into());
        }
    }
    Ok(value!(true))
}

fn validate_distribution(value: &Value) -> Result<Value, ExpressionError> {
    let statistic = value
        .get("statistic")
        .ok_or_else(|| "field \"value.value.statistic\" not found")?
        .as_str()
        .ok_or_else(|| {
            "expected distribution metric field \"value.value.statistic\" to contain a string"
        })?;

    if statistic != "histogram" && statistic != "summary" {
        return Err("expected distribution metric field \"value.value.statistic\" to be either \"histogram\" or \"summary\"".into());
    }

    let samples = value
        .get("samples")
        .ok_or_else(|| "field \"value.value.samples\" not found")?
        .as_array()
        .ok_or_else(|| {
            "expected distribution metric field \"value.value.samples\" to contain an array"
        })?;

    for (i, sample) in samples.iter().enumerate() {
        if !sample
            .get("value")
            .ok_or_else(|| {
                format!(
                    "\"value\" field of sample at index {i} in \"value.value.samples\" not found"
                )
            })?
            .is_number()
        {
            return Err(format!("expected \"value\" field of sample at index {i} in \"value.value.samples\" to contain a number").into());
        }

        let rate = sample
            .get("rate")
            .ok_or_else(|| {
                format!("\"rate\" field of sample at index {i} in \"value.value.samples\" not found")
            })?
            .as_integer()
            .ok_or_else(|| {
                format!("expected \"rate\" field of sample at index {i} in \"value.value.samples\" to contain an integer"
                )
            })?;

        if rate < 0 || u32::try_from(rate).is_err() {
            return Err(format!("expected \"rate\" field of sample at index {i} in \"value.value.samples\" to contain a positive integer with a max value of {}", u32::MAX).into());
        }
    }
    Ok(value!(true))
}

fn validate_histogram(value: &Value) -> Result<Value, ExpressionError> {
    let sum = value
        .get("sum")
        .ok_or_else(|| "field \"value.value.sum\" not found")?;

    if !sum.is_number() {
        return Err(
            "expected histogram metric field \"value.value.sum\" to contain a number".into(),
        );
    }

    let count = value
        .get("count")
        .ok_or_else(|| "field \"value.value.count\" not found")?
        .as_integer()
        .ok_or_else(|| {
            "expected histogram metric field \"value.value.count\" to contain an integer"
        })?;

    if count < 0 {
        return Err(
            "expected histogram metric field \"value.value.count\" to contain a positive integer"
                .into(),
        );
    }

    let buckets = value
        .get("buckets")
        .ok_or_else(|| "field \"value.value.buckets\" not found")?
        .as_array()
        .ok_or_else(|| {
            "expected histogram metric field \"value.value.buckets\" to contain an array"
        })?;

    for (i, bucket) in buckets.iter().enumerate() {
        if !bucket
            .get("upper_limit")
            .ok_or_else(|| {
                format!("\"upper_limit\" field of bucket at index {i} in \"value.value.buckets\" not found")
            })?
            .is_number()
        {
            return Err(format!("expected \"upper_limit\" field of bucket at index {i} in \"value.value.buckets\" to contain a number").into());
        }

        let count = bucket
            .get("count")
            .ok_or_else(|| {
                format!("\"count\" field of bucket at index {i} in \"value.value.buckets\" not found")
            })?
            .as_integer()
            .ok_or_else(|| {
                format!("expected \"count\" field of bucket at index {i} in \"value.value.buckets\" to contain an integer"
                )
            })?;

        if count < 0 {
            return Err(format!("expected \"count\" field of bucket at index {i} in \"value.value.buckets\" to contain a positive integer").into());
        }
    }

    Ok(value!(true))
}

fn validate_summary(value: &Value) -> Result<Value, ExpressionError> {
    let sum = value
        .get("sum")
        .ok_or_else(|| "field \"value.value.sum\" not found")?;

    if !sum.is_number() {
        return Err("expected summary metric field \"value.value.sum\" to contain a number".into());
    }

    let count = value
        .get("count")
        .ok_or_else(|| "field \"value.value.count\" not found")?
        .as_integer()
        .ok_or_else(|| {
            "expected summary metric field \"value.value.count\" to contain an integer"
        })?;

    if count < 0 {
        return Err(
            "expected summary metric field \"value.value.count\" to contain a positive integer"
                .into(),
        );
    }

    let quantiles = value
        .get("quantiles")
        .ok_or_else(|| "field \"value.value.quantiles\" not found")?
        .as_array()
        .ok_or_else(|| {
            "expected summary metric field \"value.value.quantiles\" to contain an array"
        })?;

    for (i, quantile) in quantiles.iter().enumerate() {
        if !quantile
            .get("quantile")
            .ok_or_else(|| {
                format!("\"quantile\" field of quantile at index {i} in \"value.value.quantiles\" not found")
            })?
            .is_number()
        {
            return Err(format!("expected \"quantile\" field of quantile at index {i} in \"value.value.quantiles\" to contain a number").into());
        }

        if !quantile
            .get("value")
            .ok_or_else(|| {
                format!("\"value\" field of quantile at index {i} in \"value.value.quantiles\" not found")
            })?
            .is_number()
        {
            return Err(format!("expected \"value\" field of quantile at index {i} in \"value.value.quantiles\" to contain a number").into());
        }
    }

    Ok(value!(true))
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Test empty string

    test_function![
        is_mezmo_metric => IsMezmoMetric;

        counter {
            args: func_args![value: value!({
                name: "counter",
                kind: "incremental",
                value: {
                    "type": "counter",
                    value: 1
                }
            })],
            want: Ok(value!(true)),
            tdef: TypeDef::boolean().fallible(),
        }

        counter_invalid_value {
            args: func_args![value: value!({
                name: "counter",
                kind: "incremental",
                value: {
                    "type": "counter",
                    value: "invalid"
                }
            })],
            want: Err("expected counter/gauge metric field \"value.value\" to contain a number"),
            tdef: TypeDef::boolean().fallible(),
        }

        gauge {
            args: func_args![value: value!({
                name: "gauge",
                kind: "incremental",
                value: {
                    "type": "gauge",
                    value: 1
                }
            })],
            want: Ok(value!(true)),
            tdef: TypeDef::boolean().fallible(),
        }

        gauge_invalid_value {
            args: func_args![value: value!({
                name: "gauge",
                kind: "incremental",
                value: {
                    "type": "gauge",
                    value: "invalid"
                }
            })],
            want: Err("expected counter/gauge metric field \"value.value\" to contain a number"),
            tdef: TypeDef::boolean().fallible(),
        }

        set {
            args: func_args![value: value!({
                name: "set",
                kind: "incremental",
                value: {
                    "type": "set",
                    value: {
                        values: ["a", "b", "c"]
                    }
                }
            })],
            want: Ok(value!(true)),
            tdef: TypeDef::boolean().fallible(),
        }

        set_no_values {
            args: func_args![value: value!({
                name: "set",
                kind: "incremental",
                value: {
                    "type": "set",
                    value: {
                        invalid: ["a", "b", "c"]
                    }
                }
            })],
            want: Err("field \"value.value.values\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        set_invalid_values {
            args: func_args![value: value!({
                name: "set",
                kind: "incremental",
                value: {
                    "type": "set",
                    value: {
                        values: "invalid"
                    }
                }
            })],
            want: Err("expected set metric field \"value.value.values\" to contain an array"),
            tdef: TypeDef::boolean().fallible(),
        }

        set_invalid_values_element {
            args: func_args![value: value!({
                name: "set",
                kind: "incremental",
                value: {
                    "type": "set",
                    value: {
                        values: ["a", 123, "c"]
                    }
                }
            })],
            want: Err("expected set metric field element at index 1 in \"value.value.values\" to contain a string"),
            tdef: TypeDef::boolean().fallible(),
        }

        summary {
            args: func_args![value: value!({
                name: "summary",
                kind: "absolute",
                value: {
                    "type": "summary",
                    value: {
                        quantiles: [
                            {
                                quantile: 0.0,
                                value: 0.000017039
                            },
                            {
                                quantile: 0.25,
                                value: 0.000018094
                            },
                            {
                                quantile: 0.5,
                                value: 0.000066005
                            },
                            {
                                quantile: 0.75,
                                value: 0.000090725
                            },
                            {
                                quantile: 1.0,
                                value: 0.000144948
                            }
                        ],
                        count: 6,
                        sum: 0.000368255,
                    }
                }
            })],
            want: Ok(value!(true)),
            tdef: TypeDef::boolean().fallible(),
        }

        summary_no_count {
            args: func_args![value: value!({
                name: "summary",
                kind: "absolute",
                value: {
                    "type": "summary",
                    value: {
                        quantiles: [],
                        sum: 0.000368255,
                    }
                }
            })],
            want: Err("field \"value.value.count\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        summary_invalid_count {
            args: func_args![value: value!({
                name: "summary",
                kind: "absolute",
                value: {
                    "type": "summary",
                    value: {
                        quantiles: [],
                        count: "invalid",
                        sum: 0.000368255,
                    }
                }
            })],
            want: Err("expected summary metric field \"value.value.count\" to contain an integer"),
            tdef: TypeDef::boolean().fallible(),
        }

        summary_negative_count {
            args: func_args![value: value!({
                name: "summary",
                kind: "absolute",
                value: {
                    "type": "summary",
                    value: {
                        quantiles: [],
                        count: (-1),
                        sum: 0.000368255,
                    }
                }
            })],
            want: Err("expected summary metric field \"value.value.count\" to contain a positive integer"),
            tdef: TypeDef::boolean().fallible(),
        }

        summary_no_sum {
            args: func_args![value: value!({
                name: "summary",
                kind: "absolute",
                value: {
                    "type": "summary",
                    value: {
                        quantiles: [],
                        count: 1,
                    }
                }
            })],
            want: Err("field \"value.value.sum\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        summary_invalid_sum {
            args: func_args![value: value!({
                name: "summary",
                kind: "absolute",
                value: {
                    "type": "summary",
                    value: {
                        quantiles: [],
                        count: 1,
                        sum: "invalid",
                    }
                }
            })],
            want: Err("expected summary metric field \"value.value.sum\" to contain a number"),
            tdef: TypeDef::boolean().fallible(),
        }

        summary_no_quantiles {
            args: func_args![value: value!({
                name: "summary",
                kind: "absolute",
                value: {
                    "type": "summary",
                    value: {
                        count: 1,
                        sum: 0.123,
                    }
                }
            })],
            want: Err("field \"value.value.quantiles\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        summary_invalid_quantiles {
            args: func_args![value: value!({
                name: "summary",
                kind: "absolute",
                value: {
                    "type": "summary",
                    value: {
                        quantiles: "invalid",
                        count: 1,
                        sum: 0.123,
                    }
                }
            })],
            want: Err("expected summary metric field \"value.value.quantiles\" to contain an array"),
            tdef: TypeDef::boolean().fallible(),
        }

        summary_invalid_quantiles_no_quantile {
            args: func_args![value: value!({
                name: "summary",
                kind: "absolute",
                value: {
                    "type": "summary",
                    value: {
                        quantiles: [
                            {
                                quantile: 0.0,
                                value: 0.000017039
                            },
                            {
                                // Missing quantile
                                value: 0.000018094
                            },
                        ],
                        count: 1,
                        sum: 0.123,
                    }
                }
            })],
            want: Err("\"quantile\" field of quantile at index 1 in \"value.value.quantiles\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        summary_invalid_quantiles_invalid_quantile {
            args: func_args![value: value!({
                name: "summary",
                kind: "absolute",
                value: {
                    "type": "summary",
                    value: {
                        quantiles: [
                            {
                                quantile: 0.0,
                                value: 0.000017039
                            },
                            {
                                quantile: "invalid",
                                value: 0.000018094
                            },
                        ],
                        count: 1,
                        sum: 0.123,
                    }
                }
            })],
            want: Err("expected \"quantile\" field of quantile at index 1 in \"value.value.quantiles\" to contain a number"),
            tdef: TypeDef::boolean().fallible(),
        }


        summary_invalid_quantiles_no_value {
            args: func_args![value: value!({
                name: "summary",
                kind: "absolute",
                value: {
                    "type": "summary",
                    value: {
                        quantiles: [
                            {
                                quantile: 0.0,
                                // Missing value
                            },
                            {
                                quantile: 0.0,
                                value: 0.000018094
                            },
                        ],
                        count: 1,
                        sum: 0.123,
                    }
                }
            })],
            want: Err("\"value\" field of quantile at index 0 in \"value.value.quantiles\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        summary_invalid_quantiles_invalid_value {
            args: func_args![value: value!({
                name: "summary",
                kind: "absolute",
                value: {
                    "type": "summary",
                    value: {
                        quantiles: [
                            {
                                quantile: 0.0,
                                value: "invalid"
                            },
                            {
                                quantile: 0.0,
                                value: 0.000018094
                            },
                        ],
                        count: 1,
                        sum: 0.123,
                    }
                }
            })],
            want: Err("expected \"value\" field of quantile at index 0 in \"value.value.quantiles\" to contain a number"),
            tdef: TypeDef::boolean().fallible(),
        }

        histogram {
            args: func_args![value: value!({
                name: "histogram",
                kind: "absolute",
                value: {
                    "type": "histogram",
                    value: {
                        buckets: [
                            {
                                upper_limit: 2.0,
                                count: 1
                            },
                            {
                                upper_limit: 4.0,
                                count: 2
                            },
                            {
                                upper_limit: 8.0,
                                count: 3
                            },
                            {
                                upper_limit: 16.0,
                                count: 4
                            },
                            {
                                upper_limit: 32.0,
                                count: 5
                            }
                            ],
                        count: 20,
                        sum: 123.0,
                    }
                }
            })],
            want: Ok(value!(true)),
            tdef: TypeDef::boolean().fallible(),
        }

        histogram_no_count {
            args: func_args![value: value!({
                name: "histogram",
                kind: "absolute",
                value: {
                    "type": "histogram",
                    value: {
                        buckets: [],
                        sum: 0.000368255,
                    }
                }
            })],
            want: Err("field \"value.value.count\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        histogram_invalid_count {
            args: func_args![value: value!({
                name: "histogram",
                kind: "absolute",
                value: {
                    "type": "histogram",
                    value: {
                        buckets: [],
                        count: "invalid",
                        sum: 0.000368255,
                    }
                }
            })],
            want: Err("expected histogram metric field \"value.value.count\" to contain an integer"),
            tdef: TypeDef::boolean().fallible(),
        }

        histogram_invalid_negative_count {
            args: func_args![value: value!({
                name: "histogram",
                kind: "absolute",
                value: {
                    "type": "histogram",
                    value: {
                        buckets: [],
                        count: (-1),
                        sum: 0.000368255,
                    }
                }
            })],
            want: Err("expected histogram metric field \"value.value.count\" to contain a positive integer"),
            tdef: TypeDef::boolean().fallible(),
        }

        histogram_no_sum {
            args: func_args![value: value!({
                name: "histogram",
                kind: "absolute",
                value: {
                    "type": "histogram",
                    value: {
                        buckets: [],
                        count: 1,
                    }
                }
            })],
            want: Err("field \"value.value.sum\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        histogram_invalid_sum {
            args: func_args![value: value!({
                name: "histogram",
                kind: "absolute",
                value: {
                    "type": "histogram",
                    value: {
                        buckets: [],
                        count: 1,
                        sum: "invalid",
                    }
                }
            })],
            want: Err("expected histogram metric field \"value.value.sum\" to contain a number"),
            tdef: TypeDef::boolean().fallible(),
        }

        histogram_no_buckets {
            args: func_args![value: value!({
                name: "histogram",
                kind: "absolute",
                value: {
                    "type": "histogram",
                    value: {
                        count: 1,
                        sum: 0.123,
                    }
                }
            })],
            want: Err("field \"value.value.buckets\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        histogram_invalid_buckets {
            args: func_args![value: value!({
                name: "histogram",
                kind: "absolute",
                value: {
                    "type": "histogram",
                    value: {
                        buckets: "invalid",
                        count: 1,
                        sum: 0.123,
                    }
                }
            })],
            want: Err("expected histogram metric field \"value.value.buckets\" to contain an array"),
            tdef: TypeDef::boolean().fallible(),
        }

        histogram_invalid_buckets_no_upper_limit {
            args: func_args![value: value!({
                name: "histogram",
                kind: "absolute",
                value: {
                    "type": "histogram",
                    value: {
                        buckets: [
                            {
                                upper_limit: 2.0,
                                count: 1
                            },
                            {
                                // Missing upper limit
                                count: 2
                            },
                        ],
                        count: 1,
                        sum: 0.123,
                    }
                }
            })],
            want: Err("\"upper_limit\" field of bucket at index 1 in \"value.value.buckets\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        histogram_invalid_buckets_invalid_upper_limit {
            args: func_args![value: value!({
                name: "histogram",
                kind: "absolute",
                value: {
                    "type": "histogram",
                    value: {
                        buckets: [
                            {
                                upper_limit: 2.0,
                                count: 1
                            },
                            {
                                upper_limit: "invalid",
                                count: 2
                            },
                        ],
                        count: 1,
                        sum: 0.123,
                    }
                }
            })],
            want: Err("expected \"upper_limit\" field of bucket at index 1 in \"value.value.buckets\" to contain a number"),
            tdef: TypeDef::boolean().fallible(),
        }

        histogram_invalid_buckets_no_count {
            args: func_args![value: value!({
                name: "histogram",
                kind: "absolute",
                value: {
                    "type": "histogram",
                    value: {
                        buckets: [
                            {
                                upper_limit: 2.0,
                                // Missing count
                            },
                            {
                                upper_limit: 4.0,
                                count: 2
                            },
                        ],
                        count: 1,
                        sum: 0.123,
                    }
                }
            })],
            want: Err("\"count\" field of bucket at index 0 in \"value.value.buckets\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        histogram_invalid_buckets_invalid_count {
            args: func_args![value: value!({
                name: "histogram",
                kind: "absolute",
                value: {
                    "type": "histogram",
                    value: {
                        buckets: [
                            {
                                upper_limit: 2.0,
                                count: "invalid"
                            },
                            {
                                upper_limit: 4.0,
                                count: 2
                            },
                        ],
                        count: 1,
                        sum: 0.123,
                    }
                }
            })],
            want: Err("expected \"count\" field of bucket at index 0 in \"value.value.buckets\" to contain an integer"),
            tdef: TypeDef::boolean().fallible(),
        }

        histogram_invalid_buckets_negative_count {
            args: func_args![value: value!({
                name: "histogram",
                kind: "absolute",
                value: {
                    "type": "histogram",
                    value: {
                        buckets: [
                            {
                                upper_limit: 2.0,
                                count: (-1)
                            },
                            {
                                upper_limit: 4.0,
                                count: 2
                            },
                        ],
                        count: 1,
                        sum: 0.123,
                    }
                }
            })],
            want: Err("expected \"count\" field of bucket at index 0 in \"value.value.buckets\" to contain a positive integer"),
            tdef: TypeDef::boolean().fallible(),
        }

        distribution {
            args: func_args![value: value!({
                name: "distribution",
                kind: "absolute",
                value: {
                    "type": "distribution",
                    value: {
                        samples: [
                            {value: 1.0, rate: 300},
                            {value: 2.2, rate: 500}
                        ],
                        statistic: "summary"
                    }
                }
            })],
            want: Ok(value!(true)),
            tdef: TypeDef::boolean().fallible(),
        }

        distribution_no_summary {
            args: func_args![value: value!({
                name: "distribution",
                kind: "absolute",
                value: {
                    "type": "distribution",
                    value: {
                        samples: [
                            {value: 1.0, rate: 300},
                            {value: 2.2, rate: 500}
                        ],
                    }
                }
            })],
            want: Err("field \"value.value.statistic\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        distribution_invalid_summary {
            args: func_args![value: value!({
                name: "distribution",
                kind: "absolute",
                value: {
                    "type": "distribution",
                    value: {
                        samples: [
                            {value: 1.0, rate: 300},
                            {value: 2.2, rate: 500}
                        ],
                        statistic: 123
                    }
                }
            })],
            want: Err("expected distribution metric field \"value.value.statistic\" to contain a string"),
            tdef: TypeDef::boolean().fallible(),
        }

        distribution_invalid_summary_value {
            args: func_args![value: value!({
                name: "distribution",
                kind: "absolute",
                value: {
                    "type": "distribution",
                    value: {
                        samples: [
                            {value: 1.0, rate: 300},
                            {value: 2.2, rate: 500}
                        ],
                        statistic: "invalid"
                    }
                }
            })],
            want: Err("expected distribution metric field \"value.value.statistic\" to be either \"histogram\" or \"summary\""),
            tdef: TypeDef::boolean().fallible(),
        }

        distribution_no_samples {
            args: func_args![value: value!({
                name: "distribution",
                kind: "absolute",
                value: {
                    "type": "distribution",
                    value: {
                        statistic: "summary"
                    }
                }
            })],
            want: Err("field \"value.value.samples\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        distribution_invalid_samples {
            args: func_args![value: value!({
                name: "distribution",
                kind: "absolute",
                value: {
                    "type": "distribution",
                    value: {
                        samples: "invalid",
                        statistic: "summary"
                    }
                }
            })],
            want: Err("expected distribution metric field \"value.value.samples\" to contain an array"),
            tdef: TypeDef::boolean().fallible(),
        }

        distribution_invalid_samples_no_value {
            args: func_args![value: value!({
                name: "distribution",
                kind: "absolute",
                value: {
                    "type": "distribution",
                    value: {
                        samples: [
                            {rate: 300}, // No value
                            {value: 2.2, rate: 500}
                        ],
                        statistic: "summary"
                    }
                }
            })],
            want: Err("\"value\" field of sample at index 0 in \"value.value.samples\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        distribution_invalid_samples_invalid_value {
            args: func_args![value: value!({
                name: "distribution",
                kind: "absolute",
                value: {
                    "type": "distribution",
                    value: {
                        samples: [
                            {value: "invalid", rate: 300},
                            {value: 2.2, rate: 500}
                        ],
                        statistic: "summary"
                    }
                }
            })],
            want: Err("expected \"value\" field of sample at index 0 in \"value.value.samples\" to contain a number"),
            tdef: TypeDef::boolean().fallible(),
        }

        distribution_invalid_samples_no_rate {
            args: func_args![value: value!({
                name: "distribution",
                kind: "absolute",
                value: {
                    "type": "distribution",
                    value: {
                        samples: [
                            {value: 1.0}, // No rate
                            {value: 2.2, rate: 500}
                        ],
                        statistic: "summary"
                    }
                }
            })],
            want: Err("\"rate\" field of sample at index 0 in \"value.value.samples\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        distribution_invalid_samples_invalid_rate {
            args: func_args![value: value!({
                name: "distribution",
                kind: "absolute",
                value: {
                    "type": "distribution",
                    value: {
                        samples: [
                            {value: 1.0, rate: "invalid"},
                            {value: 2.2, rate: 500}
                        ],
                        statistic: "summary"
                    }
                }
            })],
            want: Err("expected \"rate\" field of sample at index 0 in \"value.value.samples\" to contain an integer"),
            tdef: TypeDef::boolean().fallible(),
        }

        distribution_invalid_samples_negative_rate {
            args: func_args![value: value!({
                name: "distribution",
                kind: "absolute",
                value: {
                    "type": "distribution",
                    value: {
                        samples: [
                            {value: 1.0, rate: (-1)},
                            {value: 2.2, rate: 500}
                        ],
                        statistic: "summary"
                    }
                }
            })],
            want: Err("expected \"rate\" field of sample at index 0 in \"value.value.samples\" to contain a positive integer with a max value of 4294967295"),
            tdef: TypeDef::boolean().fallible(),
        }

        distribution_invalid_samples_too_big_rate {
            args: func_args![value: value!({
                name: "distribution",
                kind: "absolute",
                value: {
                    "type": "distribution",
                    value: {
                        samples: [
                            {value: 1.0, rate: 4294967296i64},
                            {value: 2.2, rate: 500}
                        ],
                        statistic: "summary"
                    }
                }
            })],
            want: Err("expected \"rate\" field of sample at index 0 in \"value.value.samples\" to contain a positive integer with a max value of 4294967295"),
            tdef: TypeDef::boolean().fallible(),
        }

        invalid {
            args: func_args![value: value!("invalid")],
            want: Err("expected an object"),
            tdef: TypeDef::boolean().fallible(),
        }

        no_name {
            args: func_args![value: value!({
                kind: "absolute",
                value: {
                    "type": "counter",
                    value: 1
                }
            })],
            want: Err("field \"name\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        no_kind {
            args: func_args![value: value!({
                name: "counter",
                value: {
                    "type": "counter",
                    value: 1
                }
            })],
            want: Err("field \"kind\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        invalid_kind {
            args: func_args![value: value!({
                name: "counter",
                kind: "invalid",
                value: {
                    "type": "counter",
                    value: 1
                }
            })],
            want: Err("expected field \"kind\" to be either \"absolute\" or \"incremental\""),
            tdef: TypeDef::boolean().fallible(),
        }

        no_value {
            args: func_args![value: value!({
                name: "counter",
                kind: "incremental",
            })],
            want: Err("field \"value\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }

        invalid_value_type {
            args: func_args![value: value!({
                name: "counter",
                kind: "incremental",
                value: {
                    "type": "invalid",
                    value: 1
                }
            })],
            want: Err("field \"value.type\" has invalid metric type \"invalid\""),
            tdef: TypeDef::boolean().fallible(),
        }

        no_value_value {
            args: func_args![value: value!({
                name: "counter",
                kind: "incremental",
                value: {
                    "type": "counter",
                }
            })],
            want: Err("field \"value.value\" not found"),
            tdef: TypeDef::boolean().fallible(),
        }


    ];
}
