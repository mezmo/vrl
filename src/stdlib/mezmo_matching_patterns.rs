use super::mezmo_patterns::{
    CREDIT_CARD_PATTERN, EMAIL_ADDRESS_PATTERN, IPV4_ADDRESS_PATTERN, PHONE_NUMBER_PATTERN,
    US_SOCIAL_SECURITY_NUMBER_PATTERN,
};
use crate::compiler::prelude::*;
use regex::bytes::RegexSet;
use std::collections::BTreeSet;

fn matching_patterns(value: Value, pattern: &RegexSet) -> Resolved {
    match value {
        Value::Bytes(bytes) => Ok(Value::Array(
            pattern
                .matches(&bytes)
                .into_iter()
                .map(Value::from)
                .collect(),
        )),
        Value::Array(values) => process_collection(values.into_iter(), pattern),
        Value::Object(map) => process_collection(map.into_iter().map(|(_, value)| value), pattern),
        _ => Ok(Value::Array(vec![])),
    }
}

fn process_collection(values: impl Iterator<Item = Value>, pattern: &RegexSet) -> Resolved {
    let mut matches: BTreeSet<i64> = BTreeSet::new();
    // skips recursing into nested elements if all patterns are matched in previous paths
    for value in values {
        matching_patterns(value, pattern)?
            .try_array()?
            .into_iter()
            .map(|value| {
                value
                    .try_integer()
                    .expect("matched pattern item must be integer")
            })
            .for_each(|index| {
                matches.insert(index);
            });

        if matches.len() == pattern.len() {
            break;
        }
    }
    Ok(Value::Array(matches.into_iter().map(Value::from).collect()))
}

#[derive(Clone, Copy, Debug)]
pub struct MezmoMatchingPatterns;

impl Function for MezmoMatchingPatterns {
    fn identifier(&self) -> &'static str {
        "mezmo_matching_patterns"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                keyword: "value",
                kind: kind::BYTES | kind::OBJECT | kind::ARRAY,
                required: true,
            },
            Parameter {
                keyword: "patterns",
                kind: kind::ARRAY,
                required: true,
            },
        ]
    }

    fn examples(&self) -> &'static [Example] {
        &[
            Example {
                title: "match on string",
                source: r#"matching_patterns("foobar", patterns: [r'\w+', r'\d+', r'\pL+', r'foo', r'bar', r'barfoo', r'foobar'])"#,
                result: Ok("[[0, 2, 3, 4, 6]"),
            },
            Example {
                title: "match on object",
                source: r#"matching_patterns({"name": "jon doe", "department": "sales", "notes": {"entry" => "foobar works"}}, patterns: [r'\w+', r'\d+', r'\pL+', r'foo', r'bar', r'barfoo', r'foobar'])"#,
                result: Ok("[[0, 2, 3, 4, 6]"),
            },
            Example {
                title: "match on object",
                source: r#"matching_patterns(["something", "strange", {"notes": {"entry" => "foobar works"}}], patterns: [r'\w+', r'\d+', r'\pL+', r'foo', r'bar', r'barfoo', r'foobar'])"#,
                result: Ok("[[0, 2, 3, 4, 6]"),
            },
            Example {
                title: "no match",
                source: r#"match_any("My name is John Doe", patterns: [r'\d+', r'Jane'])"#,
                result: Ok("[]"),
            },
        ]
    }

    fn compile(
        &self,
        state: &state::TypeState,
        _ctx: &mut FunctionCompileContext,
        arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");

        let re_strings = arguments
            .required_array("patterns")?
            .into_iter()
            .map(|expr| {
                expr.resolve_constant(state)
                    .ok_or(function::Error::ExpectedStaticExpression {
                        keyword: "patterns",
                        expr,
                    })
            })
            .map(|value| {
                value.and_then(|value| {
                    let err = function::Error::InvalidArgument {
                        keyword: "patterns",
                        value: value.clone(),
                        error: "unknown literal for pattern, must be a regex or pattern name",
                    };
                    if let Value::Bytes(bytes) = value.clone() {
                        match bytes.as_ref() {
                            b"credit_card" => Ok(CREDIT_CARD_PATTERN.to_string()),
                            b"email" => Ok(EMAIL_ADDRESS_PATTERN.to_string()),
                            b"ipv4" => Ok(IPV4_ADDRESS_PATTERN.to_string()),
                            b"phone_number" => Ok(PHONE_NUMBER_PATTERN.to_string()),
                            b"us_social_security_number" => {
                                Ok(US_SOCIAL_SECURITY_NUMBER_PATTERN.to_string())
                            }
                            _ => Err(err),
                        }
                    } else {
                        value
                            .try_regex()
                            .map_err(|_| err)
                            .and_then(|re| Ok((*re).to_string()))
                    }
                })
            })
            .collect::<std::result::Result<Vec<String>, _>>()?;

        let regex_set = RegexSet::new(re_strings).expect("regex were already valid");

        Ok(MatchingPatternsFn { value, regex_set }.as_expr())
    }
}

#[derive(Clone, Debug)]
struct MatchingPatternsFn {
    value: Box<dyn Expression>,
    regex_set: RegexSet,
}

impl FunctionExpression for MatchingPatternsFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        matching_patterns(value, &self.regex_set)
    }

    fn type_def(&self, _: &state::TypeState) -> TypeDef {
        TypeDef::array(Collection::from_unknown(Kind::integer())).infallible()
    }
}

#[cfg(test)]
#[allow(clippy::trivial_regex)]
mod tests {
    use super::*;
    use crate::value;
    use regex::Regex;

    test_function![
        matching_patterns => MezmoMatchingPatterns;

        string_value {
            args: func_args![value: "foobar",
                             patterns: vec![
                                 Regex::new("foo").unwrap(),
                                 Regex::new("baz").unwrap(),
                                 Regex::new("bar").unwrap(),
                                 Regex::new("baz|quux").unwrap(),
                                 Regex::new("foobar").unwrap(),
                             ]],
            want: Ok(value!([0, 2, 4])),
            tdef: TypeDef::array(Collection::from_unknown(Kind::integer())).infallible(),
        }

        object_value {
            args: func_args![
                value: value!({
                    "name": "ben johnson",
                    "age": 20,
                    "manager": {
                        "name": "jane thomson",
                        "title": "senior director",
                        "reportees": [
                            {
                                "name": "luka doncic",
                                "team": "dallas mavericks"
                            },
                            {
                                "name": "michael jordan",
                                "team": "chicago bulls"
                            },
                        ]
                    }
                }),
                patterns: vec![
                    Regex::new("jordan").unwrap(),
                    Regex::new("\\d{3}-\\d{2}").unwrap(),
                    Regex::new("\\w+@\\w.com").unwrap(),
                    Regex::new("luka").unwrap(),
                ]],
            want: Ok(value!([0, 3])),
            tdef: TypeDef::array(Collection::from_unknown(Kind::integer())).infallible(),
        }

        array_value {
            args: func_args![
                value: value!([
                    "welcome to the NBA",
                    "list of employees below",
                    {
                        "dallas": {
                            "coaches": ["jason kidd", "jared dudley"],
                            "players": ["luka", "kyrie"]
                        }
                    },
                    ["curry has 5 rings", "klay thomson is joining dallas"]
                ]),
                patterns: vec![
                    Regex::new("jordan").unwrap(),
                    Regex::new("\\d{3}-\\d{2}").unwrap(),
                    Regex::new("luka").unwrap(),
                    Regex::new("klay|jason").unwrap(),
                    Regex::new("\\w+@\\w.com").unwrap(),
                ]],
            want: Ok(value!([2, 3])),
            tdef: TypeDef::array(Collection::from_unknown(Kind::integer())).infallible(),
        }

        no_match {
            args: func_args![
                value: value!([
                    "welcome to the NBA",
                    "list of employees below",
                    {
                        "dallas": {
                            "coaches": ["jason kidd", "jared dudley"],
                            "players": ["luka", "kyrie"]
                        }
                    },
                    ["curry has 5 rings", "klay thomson is joining dallas"]
                ]),
                patterns: vec![
                    Regex::new("\\d{3}-\\d{2}").unwrap(),
                    Regex::new("\\w+@\\w.com").unwrap(),
                ]],
            want: Ok(value!([])),
            tdef: TypeDef::array(Collection::from_unknown(Kind::integer())).infallible(),
        }

        ssn_match {
            args: func_args![
                value: "my SSN is 122-33-4444",
                patterns: vec!["credit_card", "email", "us_social_security_number", "phone_number"],
            ],
            want: Ok(value!([2])),
            tdef: TypeDef::array(Collection::from_unknown(Kind::integer())).infallible(),
        }
    ];
}
