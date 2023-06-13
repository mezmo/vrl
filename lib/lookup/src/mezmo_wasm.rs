use wasm_bindgen::prelude::*;
use js_sys::Array;
use crate::{Lookup, Segment, FieldBuf, SegmentBuf};

extern crate console_error_panic_hook;

fn try_parse_isize(value: &JsValue) -> Option<isize> {
    match value.as_f64() {
        None => None,
        Some(value) => {
            if value.fract().is_normal() {
                // If the fractional value is not 0, subnormal (e.g. a small value around 0)
                // NaN or INF then we have an intentional floating point number that can't
                // be used as an array index.
                None
            } else {
                // The value in the fractional side of the number is safe to truncate and then
                // coerce into an isize value from f64. Numbers in JS are smaller (40-bits) from
                // the platform word sizes this will be deployed to (64 bit).
                Some(value.trunc() as isize)
            }
        }
    }
}

fn try_parse_coalesce(value: &JsValue) -> Option<SegmentBuf> {
    if value.is_array() {
        let value = Array::from(value);
        let mut res = Vec::with_capacity(value.length() as usize);
        for field in value {
            let field = field.as_string().expect("coalesce field values can only be strings");
            res.push(FieldBuf::from(field));
        }
        return Some(SegmentBuf::coalesce(res));
    }
    None
}

impl<'a> From<Segment<'a>> for JsValue {
    fn from(segment: Segment) -> Self {
        match segment {
            Segment::Coalesce(coalesce) => {
                let arr = Array::new_with_length(coalesce.len() as u32);
                for (idx, field) in coalesce.iter().enumerate() {
                    arr.set(idx as u32, JsValue::from(field.to_humanized_string()));
                }
                JsValue::from(arr)
            },
            Segment::Index(index) => JsValue::from(index),
            Segment::Field(field) => JsValue::from(field.to_humanized_string()),
        }
    }
}

impl From<SegmentBuf> for JsValue {
    fn from(segment: SegmentBuf) -> Self {
        match segment {
            SegmentBuf::Coalesce(coalesce) => {
                let arr = Array::new_with_length(coalesce.len() as u32);
                for (idx, field) in coalesce.iter().enumerate() {
                    arr.set(idx as u32, JsValue::from(field.to_humanized_string()));
                }
                JsValue::from(arr)
            },
            SegmentBuf::Index(index) => JsValue::from(index),
            SegmentBuf::Field(field) => JsValue::from(field.to_humanized_string()),
        }
    }
}

#[wasm_bindgen]
pub fn parse_lookup_path(lookup: &str) -> JsValue {
    console_error_panic_hook::set_once();

    let lookup = Lookup::from_str(lookup)
        .expect("input string is a valid path lookup expression");
    JsValue::from(
        lookup.iter()
            .map(|s| JsValue::from(s.to_owned()))
            .collect::<Array>()
    )
}

#[wasm_bindgen]
pub fn join_into_lookup_path(fields: JsValue) -> String {
    console_error_panic_hook::set_once();

    // Do a very JS thing and if the input is anything other than an array, put the
    // argument into an array before continue processing. This might still fail later
    // on if the element is anything other than a string or string like type.
    let fields = if fields.is_array() {
        Array::try_from(fields).expect("input should be convertable to an array")
    } else {
        let arr = Array::new_with_length(1);
        arr.set(0, fields);
        arr
    };

    let mut res = Lookup::root().into_buf(); // use owned variant of lookup
    for field in fields.iter() {
        if let Some(field) = try_parse_isize(&field) {
            res.push_back(field)
        } else if let Some(field) = try_parse_coalesce(&field) {
            res.push_back(field);
        } else if let Some(field) = field.as_string() {
            res.push_back(field);
        } else {
            panic!("field values must either be a string, array or a number, got {field:?}");
        }
    }
    res.to_string()
}

#[cfg(test)]
pub mod test {
    use super::*;
    use js_sys::eval;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_parsing_sufficiently_complex() {
        let expected = vec![
            JsValue::from_str("regular"),
            JsValue::from_str("quoted"),
            JsValue::from_str("\"quoted but spaces\""),
            JsValue::from_str("\"quoted.but.periods\""),
            JsValue::from_str("lookup"),
            JsValue::from_f64(0.0),
            JsValue::from_str("nested_lookup"),
            JsValue::from_f64(0.0),
            JsValue::from_f64(0.0),
        ];
        // Input string is from tests/fixtures/lookup/sufficiently_complex file
        let actual = parse_lookup_path(r#"regular."quoted"."quoted but spaces"."quoted.but.periods".lookup[0].nested_lookup[0][0]"#);
        let actual = Array::from(&actual).to_vec();
        assert_eq!(actual, expected);
    }

    #[wasm_bindgen_test]
    fn test_parsing_coalesced() {
        // Input string is from tests/fixtures/lookup/coalesced file
        let actual = parse_lookup_path("snoot.(boop | beep)");
        let actual = Array::from(&actual);
        assert_eq!(actual.get(0), JsValue::from_str("snoot"));
        assert_eq!(
            Array::from(&actual.get(1)).to_vec(),
            vec![ JsValue::from_str("boop"), JsValue::from_str("beep")]
        );
    }

    #[wasm_bindgen_test]
    fn test_parsing_nested_quote() {
        let expected = vec![JsValue::from_str("\"boo\\\"p\"")];
        // Input string is from tests/fixtures/lookup/nested_quote file
        let actual = parse_lookup_path(r#""boo\"p""#);
        let actual = Array::from(&actual).to_vec();
        assert_eq!(actual, expected);
    }

    #[wasm_bindgen_test]
    fn test_join_simple_fields() {
        let actual = join_into_lookup_path(eval("['top', 'middle', 'child']").unwrap());
        assert_eq!(actual, "top.middle.child");
    }

    #[wasm_bindgen_test]
    fn test_join_simple_quoted_fields() {
        let actual = join_into_lookup_path(eval("['top', 'middle-1', 'middle.2', '3', 'child']").unwrap());
        assert_eq!(actual, "top.\"middle-1\".\"middle.2\".\"3\".child");
    }

    #[wasm_bindgen_test]
    fn test_join_simple_index() {
        let actual = join_into_lookup_path(eval("['top', 0, 'middle', '0', 'child', 0, -1]").unwrap());
        assert_eq!(actual, "top[0].middle.\"0\".child[0][-1]");
    }

    #[wasm_bindgen_test]
    fn test_join_simple_coalesced() {
        let actual = join_into_lookup_path(eval("['err', ['message', 'reason']]").unwrap());
        assert_eq!(actual, "err.(message | reason)");
    }

    #[wasm_bindgen_test]
    fn test_join_simple_complex() {
        let actual = join_into_lookup_path(
            eval("['events', 0, 'err', ['message', 'reason']]").unwrap()
        );
        assert_eq!(actual, "events[0].err.(message | reason)");
    }
}
