use crate::compiler::prelude::*;

const LETTER_NUMBER_DASH: &str = r#"[a-z0-9A-Z\-.]+"#;

static MEZMO_PATTERNS: &[(&str, &str)] = &[
    ("JSON_OBJECT", r#"{.*}"#),
    ("JSON_ARRAY", r#"\[.*\]"#),
    ("XML", r#"<[\w"=\s]+>.*<\/[\w\s]+>"#),
    ("CURLY_BRACKET", r#"{|}"#),
    ("SQUARE_BRACKET", r#"\[|\]"#),
    ("TAB", r#"\t"#),
    ("SINGLE_SPACE", r#"\s"#), // SPACE is already defined in builtin patterns but it's greedy
    ("ANY_CHAR", r#"."#),
    ("DOUBLE_QUOTE", r#"""#),
    ("SINGLE_QUOTE", r#"'"#),
    ("PERIOD", r#"\."#),
    ("UTF8_WORD", r#"\p{L}+"#),
    ("SENTENCE", r#"[\p{L},":;\s\-]+"#), // Sentence with UTF8 chars
    ("OPTIONAL_SENTENCE", r#"[\p{L},":;\s\-]*"#),
    ("PSQL_LOG", "%{TIMESTAMP_ISO8601:timestamp} %{DATA} %{SQUARE_BRACKET}%{INT:pid}%{SQUARE_BRACKET} %{EMAILLOCALPART:username}%{NOTSPACE}%{WORD:database} %{GREEDYDATA:message}"),
    ("GOLANG_LOG", "%{DATESTAMP:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}"),
    ("ELASTICSEARCH_LOG", "%{SQUARE_BRACKET}%{TIMESTAMP_ISO8601:timestamp}%{SQUARE_BRACKET}%{SQUARE_BRACKET}%{LOGLEVEL:level}%{SPACE}%{SQUARE_BRACKET}%{SQUARE_BRACKET}%{DATA:source}%{SQUARE_BRACKET} %{GREEDYDATA:message}"),
    ("K8S_POD_NAME", LETTER_NUMBER_DASH),
    ("K8S_NAMESPACE", LETTER_NUMBER_DASH),
    ("K8S_CONTAINER", LETTER_NUMBER_DASH),
    ("K8S_CONTAINER_ID", r"[a-z0-9]{64}"),
    ("K8S_MEZMO_AGENT_FILENAME", "/var/log/containers/%{K8S_POD_NAME:pod}_%{K8S_NAMESPACE:namespace}_%{K8S_CONTAINER:container}-%{K8S_CONTAINER_ID:containerid}\\.log")
];

#[cfg(not(target_arch = "wasm32"))]
mod non_wasm {
    use crate::compiler::prelude::*;
    use crate::diagnostic::{Label, Span};
    use crate::value::Value;
    pub(super) use std::sync::Arc;
    use std::{collections::BTreeMap, fmt, panic};

    fn parse_grok(value: Value, pattern: Arc<grok::Pattern>) -> Resolved {
        let bytes = value.try_bytes_utf8_lossy()?;

        // Onig regex library, used by the grok library, panics when it hits a retry-limit-in-match.
        // Fixing it in the grok library (by using another regex method) can be met
        // with resistance because it requires a new API function, i.e., pattern.try_match_against()
        let possible_panic = panic::catch_unwind(|| match pattern.match_against(&bytes) {
            Some(matches) => {
                let mut result = BTreeMap::new();

                for (name, value) in &matches {
                    result.insert(name.to_string().into(), Value::from(value));
                }

                Ok(Value::from(result))
            }
            None => Err("unable to parse input with grok pattern".into()),
        });

        match possible_panic {
            Ok(r) => r,
            Err(_) => Err(format!(
                "regex with grok pattern caused a panic. Input: '{}', pattern: {:?}",
                &bytes, pattern
            )
            .into()),
        }
    }

    #[derive(Debug)]
    pub(crate) enum Error {
        InvalidGrokPattern(grok::Error),
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::InvalidGrokPattern(err) => err.fmt(f),
            }
        }
    }

    impl std::error::Error for Error {}

    impl DiagnosticMessage for Error {
        fn code(&self) -> usize {
            109
        }

        fn labels(&self) -> Vec<Label> {
            match self {
                Error::InvalidGrokPattern(err) => {
                    vec![Label::primary(
                        format!("grok pattern error: {err}"),
                        Span::default(),
                    )]
                }
            }
        }
    }

    #[derive(Clone, Debug)]
    pub(super) struct ParseGrokFn {
        pub(super) value: Box<dyn Expression>,

        // Wrapping pattern in an Arc, as cloning the pattern could otherwise be expensive.
        pub(super) pattern: Arc<grok::Pattern>,
    }

    impl FunctionExpression for ParseGrokFn {
        fn resolve(&self, ctx: &mut Context) -> Resolved {
            let value = self.value.resolve(ctx)?;
            let pattern = self.pattern.clone();

            parse_grok(value, pattern)
        }

        fn type_def(&self, _: &TypeState) -> TypeDef {
            TypeDef::object(Collection::any()).fallible()
        }
    }
}

#[allow(clippy::wildcard_imports)]
#[cfg(not(target_arch = "wasm32"))]
use non_wasm::*;

#[derive(Clone, Copy, Debug)]
pub struct ParseGrok;

impl Function for ParseGrok {
    fn identifier(&self) -> &'static str {
        "parse_grok"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                keyword: "value",
                kind: kind::BYTES,
                required: true,
            },
            Parameter {
                keyword: "pattern",
                kind: kind::BYTES,
                required: true,
            },
        ]
    }

    fn examples(&self) -> &'static [Example] {
        &[Example {
            title: "parse grok pattern",
            source: indoc! {r#"
                value = "2020-10-02T23:22:12.223222Z info Hello world"
                pattern = "%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}"

                parse_grok!(value, pattern)
            "#},
            result: Ok(indoc! {r#"
                {
                    "timestamp": "2020-10-02T23:22:12.223222Z",
                    "level": "info",
                    "message": "Hello world"
                }
            "#}),
        }]
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn compile(
        &self,
        state: &state::TypeState,
        _ctx: &mut FunctionCompileContext,
        arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");

        let pattern = arguments
            .required_literal("pattern", state)?
            .try_bytes_utf8_lossy()
            .expect("grok pattern not bytes")
            .into_owned();

        let mut grok = grok_with_mezmo_patterns();
        let pattern =
            Arc::new(grok.compile(&pattern, true).map_err(|e| {
                Box::new(Error::InvalidGrokPattern(e)) as Box<dyn DiagnosticMessage>
            })?);

        Ok(ParseGrokFn { value, pattern }.as_expr())
    }

    #[cfg(target_arch = "wasm32")]
    fn compile(
        &self,
        _state: &state::TypeState,
        ctx: &mut FunctionCompileContext,
        _: ArgumentList,
    ) -> Compiled {
        Ok(super::WasmUnsupportedFunction::new(
            ctx.span(),
            TypeDef::object(Collection::any()).fallible(),
        )
        .as_expr())
    }
}

fn grok_with_mezmo_patterns() -> grok::Grok {
    let mut grok = grok::Grok::with_default_patterns();
    for &(key, value) in MEZMO_PATTERNS {
        grok.add_pattern(String::from(key), String::from(value));
    }
    grok
}

#[cfg(test)]
mod test {
    use crate::btreemap;
    use crate::value::Value;

    use super::*;

    test_function![
        parse_grok => ParseGrok;

        invalid_grok {
            args: func_args![ value: "foo",
                              pattern: "%{NOG}"],
            want: Err("The given pattern definition name \"NOG\" could not be found in the definition map"),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        error {
            args: func_args![ value: "an ungrokkable message",
                              pattern: "%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}"],
            want: Err("unable to parse input with grok pattern"),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        error2 {
            args: func_args![ value: "2020-10-02T23:22:12.223222Z an ungrokkable message",
                              pattern: "%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}"],
            want: Err("unable to parse input with grok pattern"),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed {
            args: func_args![ value: "2020-10-02T23:22:12.223222Z info Hello world",
                              pattern: "%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}"],
            want: Ok(Value::from(btreemap! {
                "timestamp" => "2020-10-02T23:22:12.223222Z",
                "level" => "info",
                "message" => "Hello world",
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed2 {
            args: func_args![ value: "2020-10-02T23:22:12.223222Z",
                              pattern: "(%{TIMESTAMP_ISO8601:timestamp}|%{LOGLEVEL:level})"],
            want: Ok(Value::from(btreemap! {
                "timestamp" => "2020-10-02T23:22:12.223222Z",
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_json_object {
            args: func_args![ value: r#"hello {"prop": 1} info"#,
                            pattern: "%{DATA}%{JSON_OBJECT:json} %{LOGLEVEL:level}"],
            want: Ok(Value::from(btreemap! {
                "json" => r#"{"prop": 1}"#,
                "level" => "info"
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_json_array {
            args: func_args![ value: "hello [1, 2, 3] info",
                            pattern: "%{DATA}%{JSON_ARRAY:json} %{LOGLEVEL:level}"],
            want: Ok(Value::from(btreemap! {
                "json" => "[1, 2, 3]",
                "level" => "info"
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_xml {
            args: func_args![ value: r#"hello <tag prop="value">inner</tag> info"#,
                            pattern: "%{DATA}%{XML:xml} %{LOGLEVEL:level}"],
            want: Ok(Value::from(btreemap! {
                "xml" => r#"<tag prop="value">inner</tag>"#,
                "level" => "info"
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_curly_bracket {
            args: func_args![ value: "hello {world} info",
                            pattern: "hello %{CURLY_BRACKET}%{DATA:body}%{CURLY_BRACKET} %{LOGLEVEL:level}"],
            want: Ok(Value::from(btreemap! {
                "body" => "world",
                "level" => "info"
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_square_bracket {
            args: func_args![ value: "hello [world] info",
                            pattern: "hello %{SQUARE_BRACKET}%{DATA:body}%{SQUARE_BRACKET} %{LOGLEVEL:level}"],
            want: Ok(Value::from(btreemap! {
                "body" => "world",
                "level" => "info"
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_any_char {
            args: func_args![ value: "hello `world* info",
                            pattern: "hello %{ANY_CHAR}%{WORD:body}%{ANY_CHAR} %{LOGLEVEL:level}"],
            want: Ok(Value::from(btreemap! {
                "body" => "world",
                "level" => "info"
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_tab {
            args: func_args![value: "hello	world	info", // separated by tabs
                            pattern: "hello%{TAB}%{WORD:body}%{TAB}%{LOGLEVEL:level}"],
            want: Ok(Value::from(btreemap! {
                "body" => "world",
                "level" => "info"
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_single_space {
            args: func_args![ value: "hello world info",
                            pattern: "hello%{SINGLE_SPACE}%{WORD:body}%{SINGLE_SPACE}%{LOGLEVEL:level}"],
            want: Ok(Value::from(btreemap! {
                "body" => "world",
                "level" => "info"
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_double_quote {
            args: func_args![ value: "hello \"world\" info",
                            pattern: "hello %{DOUBLE_QUOTE}%{WORD:body}%{DOUBLE_QUOTE} %{LOGLEVEL:level}"],
            want: Ok(Value::from(btreemap! {
                "body" => "world",
                "level" => "info"
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_single_quote {
            args: func_args![ value: "hello 'world' info",
                            pattern: "hello %{SINGLE_QUOTE}%{WORD:body}%{SINGLE_QUOTE} %{LOGLEVEL:level}"],
            want: Ok(Value::from(btreemap! {
                "body" => "world",
                "level" => "info"
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_period {
            args: func_args![ value: "hello .world info",
                            pattern: "hello %{PERIOD}%{WORD:body} %{LOGLEVEL:level}"],
            want: Ok(Value::from(btreemap! {
                "body" => "world",
                "level" => "info"
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_utf8_word {
            args: func_args![ value: "hello Visca Barça España 你好 info",
                            pattern: "hello %{UTF8_WORD:a} %{UTF8_WORD:b} %{UTF8_WORD:c} %{UTF8_WORD:d} %{LOGLEVEL:level}"],
            want: Ok(Value::from(btreemap! {
                "a" => "Visca",
                "b" => "Barça",
                "c" => "España",
                "d" => "你好",
                "level" => "info"
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_sentence {
            args: func_args![ value: "hello Visca Barça España 你好 info",
                            pattern: "hello %{SENTENCE:body} %{LOGLEVEL:level}"],
            want: Ok(Value::from(btreemap! {
                "body" => "Visca Barça España 你好",
                "level" => "info"
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_optional_sentence {
            args: func_args![ value: "hello Visca Barça España 你好 info",
                            pattern: "hello %{OPTIONAL_SENTENCE:body} %{LOGLEVEL:level}%{OPTIONAL_SENTENCE:empty}"],
            want: Ok(Value::from(btreemap! {
                "body" => "Visca Barça España 你好",
                "level" => "info",
                "empty" => ""
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_psql_log {
            args: func_args![
                value: "2023-07-30 08:31:50.628 UTC [2176] postgres@chinook starting PostgreSQL 15.3 (Ubuntu 15.3-0ubuntu0.22.04.1) on x86_64-pc-linux-gnu, compiled by gcc (Ubuntu 11.3.0-1ubuntu1~22.04.1) 11.3.0, 64-bit",
                pattern: "%{PSQL_LOG}"
            ],
            want: Ok(Value::from(btreemap! {
                "database" => "chinook",
                "message" => "starting PostgreSQL 15.3 (Ubuntu 15.3-0ubuntu0.22.04.1) on x86_64-pc-linux-gnu, compiled by gcc (Ubuntu 11.3.0-1ubuntu1~22.04.1) 11.3.0, 64-bit",
                "pid" => "2176",
                "timestamp" => "2023-07-30 08:31:50.628",
                "username" => "postgres"
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_golang_log {
            args: func_args![
                value: "01/23/2024 16:24:22 INFO this is a demo line",
                pattern: "%{GOLANG_LOG}"
            ],
            want: Ok(Value::from(btreemap! {
                "level" => "INFO",
                "message" => "this is a demo line",
                "timestamp" => "01/23/2024 16:24:22",
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_mezmo_elasticsearch_log {
            args: func_args![
                value: "[2023-02-24 10:45:49,124][WARN ][index.search.slowlog.query] took[2.9ms], took_millis[2], types[config], stats[], search_type[QUERY_AND_FETCH], total_shards[1], source[{\"size\":1000,\"sort\":[{\"timestamp\":{\"order\":\"desc\",\"ignore_unmapped\":true}}]}], extra_source[],",
                pattern: "%{ELASTICSEARCH_LOG}"
            ],
            want: Ok(Value::from(btreemap! {
                "level" => "WARN",
                "message" => "took[2.9ms], took_millis[2], types[config], stats[], search_type[QUERY_AND_FETCH], total_shards[1], source[{\"size\":1000,\"sort\":[{\"timestamp\":{\"order\":\"desc\",\"ignore_unmapped\":true}}]}], extra_source[],",
                "source" => "index.search.slowlog.query",
                "timestamp" => "2023-02-24 10:45:49,124"
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_kubernetes_pod_name {
            args: func_args![
                value: "calico-node-dr4wc",
                pattern: "%{K8S_POD_NAME:pod}"
            ],
            want: Ok(Value::from(btreemap! {
                "pod" => "calico-node-dr4wc",
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_kubernetes_namespace {
            args: func_args![
                value: "kube-system",
                pattern: "%{K8S_NAMESPACE:namespace}"
            ],
            want: Ok(Value::from(btreemap! {
                "namespace" => "kube-system",
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_kubernetes_container {
            args: func_args![
                value: "calico-node",
                pattern: "%{K8S_CONTAINER:container}"
            ],
            want: Ok(Value::from(btreemap! {
                "container" => "calico-node",
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_kubernetes_containerid {
            args: func_args![
                value: "abfd21da18f04db30f848e58e9d5f3c55fdcfb6a12b32e972e04b784b04d915a",
                pattern: "%{K8S_CONTAINER_ID:containerid}"
            ],
            want: Ok(Value::from(btreemap! {
                "containerid" => "abfd21da18f04db30f848e58e9d5f3c55fdcfb6a12b32e972e04b784b04d915a",
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }

        parsed_kubernetes_mezmo_agent_filename {
            args: func_args![
                value: "/var/log/containers/calico-node-dr4wc_kube-system_calico-node-abfd21da18f04db30f848e58e9d5f3c55fdcfb6a12b32e972e04b784b04d915a.log",
                pattern: "%{K8S_MEZMO_AGENT_FILENAME}"
            ],
            want: Ok(Value::from(btreemap! {
                "pod" => "calico-node-dr4wc",
                "namespace" => "kube-system",
                "container" => "calico-node",
                "containerid" => "abfd21da18f04db30f848e58e9d5f3c55fdcfb6a12b32e972e04b784b04d915a",
            })),
            tdef: TypeDef::object(Collection::any()).fallible(),
        }
    ];
}
