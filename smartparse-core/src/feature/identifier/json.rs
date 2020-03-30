use std::borrow::Cow;

use serde_json::Value;

use super::FeatureIdentifier;
use crate::feature::{Feature, Source, TypedValue};

struct Json {}

impl<'a> FeatureIdentifier<'a> for Json {
    fn identify(&self, input: &'a str) -> Option<Vec<Feature>> {
        // Don't try to even parse as json object if it doesn't look like
        // a json hash (what we support right now).
        if !input.trim().starts_with("{") {
            return None;
        }

        let v: Value = if let Ok(val) = serde_json::from_str(input) {
            val
        } else {
            return None;
        };

        // TODO(darren): could support more complex JSON parsing, right now
        // we'll just parse the top-level keys / values.
        if let Value::Object(map) = v {
            let mut features = vec![];
            for (key, value) in map {
                // NOTE(darren): maybe there is a better way here, but to derive
                // the raw value here we convert it back to a string.. it's a bit hacky.
                let raw_value = value.to_string();
                let typed_value = match value {
                    Value::Null => TypedValue::Null,
                    Value::Bool(v) => TypedValue::Bool(v),
                    Value::Number(v) => {
                        if let Some(v) = v.as_i64() {
                            TypedValue::I64(v)
                        } else if let Some(v) = v.as_f64() {
                            TypedValue::F64(v)
                        } else {
                            // NOTE(darren): could support more types (u64, for example) here.
                            continue;
                        }
                    }
                    Value::String(v) => TypedValue::Str(Cow::Owned(dbg!(v))),
                    // TODO(darren): Implement, we skip more complex types for now.
                    Value::Array(_) | Value::Object(_) => continue,
                };
                features.push(Feature::new_typed(key, raw_value, typed_value).source(Source::Json))
            }
            Some(features)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::testutil::*;

    #[test]
    fn works_as_expected() {
        let features = Json {}.identify(r#"{ "a": "foo", "b": 100, "zz": 80.1 }"#);
        assert_similarity_equal(
            features.expect("found features"),
            vec![
                Feature::new("a", "foo"),
                Feature::new("b", "100"),
                Feature::new("zz", "80.1"),
            ],
        );
    }
}
