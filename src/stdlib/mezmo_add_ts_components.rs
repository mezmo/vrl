use crate::compiler::prelude::*;
use chrono::{DateTime, Duration, Months, TimeZone, Utc};

const MONTHS_IN_A_YEAR: u8 = 12;
const ERROR_INVALID_TIME_COMPONENTS: &str = "Invalid time components";
const MILLISECONDS_IN_SECOND: u32 = 1000;
const MILLISECONDS_IN_MINUTE: u32 = 60 * MILLISECONDS_IN_SECOND;
const MILLISECONDS_IN_HOUR: u32 = 3600 * MILLISECONDS_IN_SECOND;
const MILLISECONDS_IN_DAY: u32 = 24 * MILLISECONDS_IN_HOUR;

fn add_ts_components(
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
    let months = total_months(years, months)?;
    let total_ms = total_milliseconds(days, hours, minutes, seconds, milliseconds)?;

    ts = add_months(ts, months)?;
    ts = add_milliseconds(ts, total_ms)?;

    Ok(Value::from(ts))
}

#[derive(Clone, Copy, Debug)]
pub struct MezmoAddTsComponents;

impl Function for MezmoAddTsComponents {
    fn identifier(&self) -> &'static str {
        "mezmo_add_ts_components"
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
                title: "add months",
                source: r#"mezmo_add_ts_components(t'2021-02-10T23:32:00+00:00', months: 2)"#,
                result: Ok(r#"2021-04-10T23:32:00+00:00"#),
            },
            Example {
                title: "add months and days",
                source: r#"mezmo_add_ts_components(t'2021-02-10T23:32:00+00:00', months: 2, days: 6)"#,
                result: Ok(r#"2021-04-1623:32:00+00:00"#),
            },
            Example {
                title: "add years, months, days, hours, minutes, seconds and milliseconds",
                source: r#"mezmo_add_ts_components(t'2021-02-10T23:32:00+00:00', years: 3, months: 2, days: 6, hours: 4, minutes: 10, seconds: 5, milliseconds: 200)"#,
                result: Ok(r#"2024-04-17T03:42:05.200+00:00"#),
            },
            Example {
                title: "subtract years, months and days",
                source: r#"mezmo_add_ts_components(t'2021-02-10T23:32:00+00:00', years: -5, months: -2, days: -6)"#,
                result: Ok(r#"2015-12-04T23:32:00+00:00"#),
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

        Ok(MezmoAddTsComponentsFn {
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
struct MezmoAddTsComponentsFn {
    value: Box<dyn Expression>,
    years: Option<Box<dyn Expression>>,
    months: Option<Box<dyn Expression>>,
    days: Option<Box<dyn Expression>>,
    hours: Option<Box<dyn Expression>>,
    minutes: Option<Box<dyn Expression>>,
    seconds: Option<Box<dyn Expression>>,
    milliseconds: Option<Box<dyn Expression>>,
}

impl FunctionExpression for MezmoAddTsComponentsFn {
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

        add_ts_components(
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

fn timezone_component_or_default(val: Option<Value>) -> Result<i64, ValueError> {
    match val {
        Some(expr) => expr.try_integer(),
        None => Ok(0),
    }
}

fn total_months(years: Option<Value>, months: Option<Value>) -> Result<i64, ExpressionError> {
    let years = timezone_component_or_default(years)?;
    let years = match years.checked_mul(MONTHS_IN_A_YEAR as i64) {
        Some(val) => val,
        None => {
            return Err(ERROR_INVALID_TIME_COMPONENTS.into());
        }
    };
    let months = timezone_component_or_default(months)?;
    match years.checked_add(months) {
        Some(val) => Ok(val),
        None => Err(ERROR_INVALID_TIME_COMPONENTS.into()),
    }
}

fn add_months<T: TimeZone>(ts: DateTime<T>, months: i64) -> Result<DateTime<T>, ExpressionError> {
    if months == 0 {
        return Ok(ts);
    }
    let perform_add = months > 0;
    let months = match u32::try_from(months.abs()) {
        Ok(val) => Months::new(val),
        Err(_) => {
            return Err(ERROR_INVALID_TIME_COMPONENTS.into());
        }
    };

    if perform_add {
        match ts.checked_add_months(months) {
            Some(val) => Ok(val),
            None => Err(ERROR_INVALID_TIME_COMPONENTS.into()),
        }
    } else {
        match ts.checked_sub_months(months) {
            Some(val) => Ok(val),
            None => Err(ERROR_INVALID_TIME_COMPONENTS.into()),
        }
    }
}

fn total_milliseconds(
    days: Option<Value>,
    hours: Option<Value>,
    minutes: Option<Value>,
    seconds: Option<Value>,
    milliseconds: Option<Value>,
) -> Result<i64, ExpressionError> {
    let mut total = 0 as i64;
    let values = vec![
        to_milliseconds(timezone_component_or_default(days)?, MILLISECONDS_IN_DAY)?,
        to_milliseconds(timezone_component_or_default(hours)?, MILLISECONDS_IN_HOUR)?,
        to_milliseconds(
            timezone_component_or_default(minutes)?,
            MILLISECONDS_IN_MINUTE,
        )?,
        to_milliseconds(
            timezone_component_or_default(seconds)?,
            MILLISECONDS_IN_SECOND,
        )?,
        timezone_component_or_default(milliseconds)?,
    ];

    for val in values.iter() {
        total = match total.checked_add(*val) {
            Some(new_val) => new_val,
            None => {
                return Err(ERROR_INVALID_TIME_COMPONENTS.into());
            }
        }
    }

    Ok(total)
}

fn to_milliseconds(val: i64, multiplier: u32) -> Result<i64, ExpressionError> {
    match val.checked_mul(multiplier as i64) {
        Some(val) => Ok(val),
        None => {
            return Err(ERROR_INVALID_TIME_COMPONENTS.into());
        }
    }
}

fn add_milliseconds<T: TimeZone>(
    ts: DateTime<T>,
    milliseconds: i64,
) -> Result<DateTime<T>, ExpressionError> {
    if milliseconds == 0 {
        return Ok(ts);
    }
    let duration = Duration::milliseconds(milliseconds);
    match ts.checked_add_signed(duration) {
        Some(val) => Ok(val),
        None => Err(ERROR_INVALID_TIME_COMPONENTS.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value;
    use chrono::Utc;

    test_function![
      add_ts_components => MezmoAddTsComponents;

      add_nothing {
        args: func_args![value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap()],
        want: Ok(value!(Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      add_years {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          years: 2
        ],
        want: Ok(value!(Utc.with_ymd_and_hms(2023, 02, 10, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      add_years_and_months {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          years: 2,
          months: 6
        ],
        want: Ok(value!(Utc.with_ymd_and_hms(2023, 08, 10, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      add_years_months_days_hours_minutes_seconds_milliseconds {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          years: 4,
          months: 10,
          days: 22,
          hours: 3,
          minutes: 20,
          seconds: 40,
          milliseconds: 100,
        ],
        want: Ok(value!(DateTime::parse_from_rfc3339("2026-01-02T02:52:40.100+00:00").unwrap().with_timezone(&Utc))),
        tdef: TypeDef::bytes().fallible(),
      }

      add_day_in_leap_year {
        args: func_args![
          value: Utc.with_ymd_and_hms(2024, 02, 28, 23, 32, 0).unwrap(),
          days: 1
        ],
        want: Ok(value!(Utc.with_ymd_and_hms(2024, 02, 29, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      add_month_in_leap_year {
        args: func_args![
          value: Utc.with_ymd_and_hms(2024, 01, 31, 23, 32, 0).unwrap(),
          months: 1
        ],
        want: Ok(value!(Utc.with_ymd_and_hms(2024, 02, 29, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      add_day_in_non_leap_year {
        args: func_args![
          value: Utc.with_ymd_and_hms(2023, 02, 28, 23, 32, 0).unwrap(),
          days: 1
        ],
        want: Ok(value!(Utc.with_ymd_and_hms(2023, 03, 01, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      add_month_in_non_leap_year {
        args: func_args![
          value: Utc.with_ymd_and_hms(2023, 01, 31, 23, 32, 0).unwrap(),
          months: 1
        ],
        want: Ok(value!(Utc.with_ymd_and_hms(2023, 02, 28, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      add_week_in_leap_year {
        args: func_args![
          value: Utc.with_ymd_and_hms(2024, 02, 26, 23, 32, 0).unwrap(),
          days: 7
        ],
        want: Ok(value!(Utc.with_ymd_and_hms(2024, 03, 04, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      add_week_in_non_leap_year {
        args: func_args![
          value: Utc.with_ymd_and_hms(2023, 02, 27, 23, 32, 0).unwrap(),
          days: 7
        ],
        want: Ok(value!(Utc.with_ymd_and_hms(2023, 03, 06, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      add_year_to_leap_year {
        args: func_args![
          value: Utc.with_ymd_and_hms(2024, 02, 29, 23, 32, 0).unwrap(),
          years: 1
        ],
        want: Ok(value!(Utc.with_ymd_and_hms(2025, 02, 28, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      add_year_to_non_leap_year {
        args: func_args![
          value: Utc.with_ymd_and_hms(2023, 02, 28, 23, 32, 0).unwrap(),
          years: 1
        ],
        want: Ok(value!(Utc.with_ymd_and_hms(2024, 02, 28, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      subtract_nothing {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
        ],
        want: Ok(value!(Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      subtract_years {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          years: -5
        ],
        want: Ok(value!(Utc.with_ymd_and_hms(2016, 02, 10, 23, 32, 0).unwrap())),
        tdef: TypeDef::bytes().fallible(),
      }

      subtract_years_months_days_hours_minutes_seconds_milliseconds {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          years: -5,
          months: -4,
          days: -10,
          hours: -22,
          minutes: -70,
          seconds: -10,
          milliseconds: -200,
        ],
        want: Ok(value!(DateTime::parse_from_rfc3339("2015-09-30T00:21:49.800+00:00").unwrap().with_timezone(&Utc))),
        tdef: TypeDef::bytes().fallible(),
      }

      // errors
      error_add_years {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          years: i64::MAX,
        ],
        want: Err("Invalid time components"),
        tdef: TypeDef::bytes().fallible(),
      }

      error_add_months {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          months: i64::MAX,
        ],
        want: Err("Invalid time components"),
        tdef: TypeDef::bytes().fallible(),
      }

      error_subtract_time {
        args: func_args![
          value: Utc.with_ymd_and_hms(2021, 02, 10, 23, 32, 0).unwrap(),
          days: i64::MIN,
          minutes: i64::MIN,
        ],
        want: Err("Invalid time components"),
        tdef: TypeDef::bytes().fallible(),
      }

    ];
}
