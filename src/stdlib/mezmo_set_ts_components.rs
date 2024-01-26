use crate::compiler::prelude::*;
use chrono::{DateTime, Datelike, Timelike, Utc};
use std::fmt::Display;

const ONE_MILLISECOND_IN_NANOSECONDS: u32 = 1_000_000;

fn set_ts_components(
    ts: Value,
    years: Option<Value>,
    months: Option<Value>,
    days: Option<Value>,
    hours: Option<Value>,
    minutes: Option<Value>,
    seconds: Option<Value>,
    milliseconds: Option<Value>,
) -> Resolved {
    let mut ts: DateTime<Utc> = ts.try_timestamp()?;
    let years = get_arg_value::<i32>("years", years, 0, None)?;
    let months = get_arg_value::<u32>("months", months, 1, Some(12))?;
    let days = get_arg_value::<u32>("days", days, 1, Some(31))?;
    let hours = get_arg_value::<u32>("hours", hours, 0, Some(23))?;
    let minutes = get_arg_value::<u32>("minutes", minutes, 0, Some(59))?;
    let seconds = get_arg_value::<u32>("seconds", seconds, 0, Some(59))?;
    let milliseconds = get_arg_value::<u32>("milliseconds", milliseconds, 0, Some(999))?;

    if let Some(years) = years {
        ts = match ts.with_year(years) {
            Some(val) => val,
            None => {
                return Err("Invalid years".into());
            }
        };
    }
    if let Some(months) = months {
        ts = match ts.with_month(months) {
            Some(val) => val,
            None => {
                return Err("Invalid months".into());
            }
        }
    }
    if let Some(days) = days {
        ts = match ts.with_day(days) {
            Some(val) => val,
            None => {
                return Err("Invalid days".into());
            }
        }
    }
    if let Some(hours) = hours {
        ts = match ts.with_hour(hours) {
            Some(val) => val,
            None => {
                return Err("Invalid hours".into());
            }
        }
    }
    if let Some(minutes) = minutes {
        ts = match ts.with_minute(minutes) {
            Some(val) => val,
            None => {
                return Err("Invalid minutes".into());
            }
        }
    }
    if let Some(seconds) = seconds {
        ts = match ts.with_second(seconds) {
            Some(val) => val,
            None => {
                return Err("Invalid seconds".into());
            }
        }
    }
    if let Some(milliseconds) = milliseconds {
        ts = match ts.with_nanosecond(milliseconds * ONE_MILLISECOND_IN_NANOSECONDS) {
            Some(val) => val,
            None => {
                return Err("Invalid milliseconds".into());
            }
        }
    }

    Ok(Value::from(ts))
}

#[derive(Clone, Copy, Debug)]
pub struct MezmoSetTsComponents;

impl Function for MezmoSetTsComponents {
    fn identifier(&self) -> &'static str {
        "mezmo_set_ts_components"
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                keyword: "value",
                kind: kind::TIMESTAMP,
                required: true,
            },
            Parameter {
                keyword: "years",
                kind: kind::INTEGER,
                required: false,
            },
            Parameter {
                keyword: "months",
                kind: kind::INTEGER,
                required: false,
            },
            Parameter {
                keyword: "days",
                kind: kind::INTEGER,
                required: false,
            },
            Parameter {
                keyword: "hours",
                kind: kind::INTEGER,
                required: false,
            },
            Parameter {
                keyword: "minutes",
                kind: kind::INTEGER,
                required: false,
            },
            Parameter {
                keyword: "seconds",
                kind: kind::INTEGER,
                required: false,
            },
            Parameter {
                keyword: "milliseconds",
                kind: kind::INTEGER,
                required: false,
            },
        ]
    }

    fn examples(&self) -> &'static [Example] {
        &[
            Example {
                title: "set months",
                source: r#"mezmo_set_ts_components(t'2021-02-10T23:32:00+00:00', months: 6)"#,
                result: Ok(r#"2021-06-10T23:32:00+00:00"#),
            },
            Example {
                title: "set months and days",
                source: r#"mezmo_set_ts_components(t'2021-02-10T23:32:00+00:00', months: 3, days: 6)"#,
                result: Ok(r#"2021-03-0623:32:00+00:00"#),
            },
            Example {
                title: "set years, months, days, hours, minutes, seconds and milliseconds",
                source: r#"mezmo_set_ts_components(t'2021-02-10T23:32:00+00:00', years: 2000, months: 10, days: 4, hours: 4, minutes: 10, seconds: 55, milliseconds: 200)"#,
                result: Ok(r#"2000-10-04T04:10:55.200+00:00"#),
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
        let years = arguments.optional("years");
        let months = arguments.optional("months");
        let days = arguments.optional("days");
        let hours = arguments.optional("hours");
        let minutes = arguments.optional("minutes");
        let seconds = arguments.optional("seconds");
        let milliseconds = arguments.optional("milliseconds");

        Ok(MezmoSetTsComponentsFn {
            value,
            years,
            months,
            days,
            hours,
            minutes,
            seconds,
            milliseconds,
        }
        .as_expr())
    }
}

#[derive(Debug, Clone)]
struct MezmoSetTsComponentsFn {
    value: Box<dyn Expression>,
    years: Option<Box<dyn Expression>>,
    months: Option<Box<dyn Expression>>,
    days: Option<Box<dyn Expression>>,
    hours: Option<Box<dyn Expression>>,
    minutes: Option<Box<dyn Expression>>,
    seconds: Option<Box<dyn Expression>>,
    milliseconds: Option<Box<dyn Expression>>,
}

impl FunctionExpression for MezmoSetTsComponentsFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let ts = self.value.resolve(ctx)?;
        let years = self
            .years
            .as_ref()
            .map(|expr| expr.resolve(ctx))
            .transpose()?;
        let months = self
            .months
            .as_ref()
            .map(|expr| expr.resolve(ctx))
            .transpose()?;
        let days = self
            .days
            .as_ref()
            .map(|expr| expr.resolve(ctx))
            .transpose()?;
        let hours = self
            .hours
            .as_ref()
            .map(|expr| expr.resolve(ctx))
            .transpose()?;
        let minutes = self
            .minutes
            .as_ref()
            .map(|expr| expr.resolve(ctx))
            .transpose()?;
        let seconds = self
            .seconds
            .as_ref()
            .map(|expr| expr.resolve(ctx))
            .transpose()?;
        let milliseconds = self
            .milliseconds
            .as_ref()
            .map(|expr| expr.resolve(ctx))
            .transpose()?;

        set_ts_components(
            ts,
            years,
            months,
            days,
            hours,
            minutes,
            seconds,
            milliseconds,
        )
    }

    fn type_def(&self, _state: &state::TypeState) -> TypeDef {
        TypeDef::bytes().fallible()
    }
}

fn get_arg_value<T>(
    arg_name: &str,
    arg_value: Option<Value>,
    min_allowed: T,
    max_allowed: Option<T>,
) -> Result<Option<T>, ExpressionError>
where
    T: Display + TryFrom<i64> + PartialOrd + Copy,
{
    let val = match arg_value {
        Some(expr) => expr.try_integer()?,
        None => {
            return Ok(None);
        }
    };
    let error_message = match max_allowed {
        Some(max) => format!("{} must be between {} and {}", arg_name, min_allowed, max),
        None => format!(
            "{} must be greater than or equal to {}",
            arg_name, min_allowed
        ),
    };

    let val = match T::try_from(val) {
        Ok(new_val) => new_val,
        Err(_) => return Err(error_message.into()),
    };

    let is_valid = match max_allowed {
        Some(max) => val >= min_allowed && val <= max,
        None => val >= min_allowed,
    };
    if !is_valid {
        return Err(error_message.into());
    }

    Ok(Some(val))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value;
    use chrono::{DateTime, TimeZone, Utc};

    test_function![
      set_ts_components => MezmoSetTsComponents;

      set_nothing {
        args: func_args![value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap()],
        want: Ok(value!(Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      set_years {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          years: 1990,
        ],
        want: Ok(value!(Utc.with_ymd_and_hms(1990, 02, 10, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      set_months {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          months: 10,
        ],
        want: Ok(value!(Utc.with_ymd_and_hms(2021, 10, 10, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      set_days {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          days: 22,
        ],
        want: Ok(value!(Utc.with_ymd_and_hms(2021, 02, 22, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      set_hours_minutes_seconds_milliseconds {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          hours: 09,
          minutes: 55,
          seconds: 20,
          milliseconds: 100,
        ],
        want: Ok(value!(DateTime::parse_from_rfc3339("2021-02-10T09:55:20.100+00:00").unwrap().with_timezone(&Utc))),
        tdef: TypeDef::bytes().fallible(),
      }

      error_min_years {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          years: -200,
        ],
        want: Err("years must be greater than or equal to 0"),
        tdef: TypeDef::bytes().fallible(),
      }

      error_min_months {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          months: 0,
        ],
        want: Err("months must be between 1 and 12"),
        tdef: TypeDef::bytes().fallible(),
      }

      error_max_months {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          months: 13,
        ],
        want: Err("months must be between 1 and 12"),
        tdef: TypeDef::bytes().fallible(),
      }

      error_min_days {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          days: 0,
        ],
        want: Err("days must be between 1 and 31"),
        tdef: TypeDef::bytes().fallible(),
      }

      error_max_days {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          days: 32,
        ],
        want: Err("days must be between 1 and 31"),
        tdef: TypeDef::bytes().fallible(),
      }

      error_min_hours {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          hours: -1,
        ],
        want: Err("hours must be between 0 and 23"),
        tdef: TypeDef::bytes().fallible(),
      }

      error_max_hours {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          hours: 24,
        ],
        want: Err("hours must be between 0 and 23"),
        tdef: TypeDef::bytes().fallible(),
      }

      error_min_minutes {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          minutes: -1,
        ],
        want: Err("minutes must be between 0 and 59"),
        tdef: TypeDef::bytes().fallible(),
      }

      error_max_minutes {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          minutes: 60,
        ],
        want: Err("minutes must be between 0 and 59"),
        tdef: TypeDef::bytes().fallible(),
      }

      error_min_seconds {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          seconds: -1,
        ],
        want: Err("seconds must be between 0 and 59"),
        tdef: TypeDef::bytes().fallible(),
      }

      error_max_seconds {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          seconds: -1,
        ],
        want: Err("seconds must be between 0 and 59"),
        tdef: TypeDef::bytes().fallible(),
      }

      error_min_millisecondss {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          milliseconds: -1,
        ],
        want: Err("milliseconds must be between 0 and 999"),
        tdef: TypeDef::bytes().fallible(),
      }

      error_max_millisecondss {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          milliseconds: 1000,
        ],
        want: Err("milliseconds must be between 0 and 999"),
        tdef: TypeDef::bytes().fallible(),
      }

      error_set_days {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          // no leap years in 2021
          days: 29,
        ],
        want: Err("Invalid days"),
        tdef: TypeDef::bytes().fallible(),
      }
    ];
}
