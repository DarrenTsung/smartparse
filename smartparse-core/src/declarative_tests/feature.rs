use crate::feature::{Feature as ProtoFeature, TypedValue as ProtoValue};
use serde_derive::Deserialize;
use std::fmt;

#[derive(Debug, PartialEq, Deserialize)]
/// Copy of feature::TypedValue.
///
/// This struct is necessary because we don't need serde deserialization
/// on the real type, it's only used for declarative tests.
enum Value {
    Null,
    String(String),
    Bool(bool),
    I64(i64),
    F64(f64),
}

impl<'a> From<ProtoValue<'a>> for Value {
    fn from(v: ProtoValue<'a>) -> Value {
        match v {
            ProtoValue::Null => Value::Null,
            ProtoValue::Str(s) => Value::String(s.to_string()),
            ProtoValue::Bool(v) => Value::Bool(v),
            ProtoValue::I64(v) => Value::I64(v),
            ProtoValue::F64(v) => Value::F64(v),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Bool(v) => write!(f, "{}", v),
            Value::I64(v) => write!(f, "{}", v),
            Value::F64(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
/// A copy of feature::Feature, used because we have a copy of TypedValue.
pub struct Feature {
    #[serde(default)]
    key: String,

    value: Value,
}

impl<'a> From<ProtoFeature<'a>> for Feature {
    fn from(f: ProtoFeature<'a>) -> Self {
        Feature {
            key: f.key().to_string(),
            value: Value::from(f.typed_value_no_cache().into_owned()),
        }
    }
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.key.is_empty() {
            write!(f, "Feature({})", self.value)
        } else {
            write!(f, "Feature(key: {}, value: {})", self.key, self.value)
        }
    }
}

impl Feature {
    pub fn line_separated_format(features: &[Feature], tabs: usize) -> String {
        let tabs = "\t".repeat(tabs);
        let mut s = "[\n".to_string();
        for f in features {
            s.push_str(&format!("{}\t{}\n", tabs, f));
        }
        s.push_str(&format!("{}]", tabs));
        s
    }
}
