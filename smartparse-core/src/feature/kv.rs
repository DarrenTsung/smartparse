use std::borrow::Cow;
use std::str::FromStr;

use super::Feature;

#[derive(Debug)]
pub struct KeyValue<'a> {
    key: Cow<'a, str>,
    raw_value: Cow<'a, str>,

    /// The typed value is only created (parsed) when needed.
    /// An example where this is not needed is if this KeyValue feature
    /// does not match the cluster, and is discarded, knowing the type is not useful.
    typed_value: Option<TypedValue<'a>>,
}

impl<'a> KeyValue<'a> {
    /// Create a new KeyValue from a key and a value.
    pub fn new(key: impl Into<Cow<'a, str>>, raw_value: impl Into<Cow<'a, str>>) -> Self {
        Self {
            key: key.into(),
            raw_value: raw_value.into(),
            typed_value: None,
        }
    }

    /// Create a new KeyValue from a key and a value, providing an already typed value.
    /// This is useful for removing redundant work - for example, if the JSON parser
    /// has already identified types in the process of parsing.
    pub(in crate::feature) fn new_typed(
        key: impl Into<Cow<'a, str>>,
        raw_value: impl Into<Cow<'a, str>>,
        typed_value: TypedValue<'a>,
    ) -> Self {
        Self {
            key: key.into(),
            raw_value: raw_value.into(),
            typed_value: Some(typed_value),
        }
    }

    fn value_type(&mut self) -> Type {
        self.typed_value().primative_type()
    }

    fn typed_value(&mut self) -> &TypedValue {
        if self.typed_value.is_some() {
            return self.typed_value.as_ref().expect("exists");
        }

        self.typed_value = Some(self.parse_typed_value());
        self.typed_value.as_ref().expect("exists")
    }

    fn parse_typed_value(&self) -> TypedValue<'a> {
        if let Ok(val) = i64::from_str(&self.raw_value) {
            return TypedValue::I64(val);
        }

        if let Ok(val) = f64::from_str(&self.raw_value) {
            return TypedValue::F64(val);
        }

        match self.raw_value.as_ref() {
            // Deseralize nulls.
            "null" | "nil" => return TypedValue::Null,
            // Deserialize booleans.
            "False" | "false" => return TypedValue::Bool(false),
            "True" | "true" => return TypedValue::Bool(true),
            _ => (),
        }

        TypedValue::Str(self.raw_value.clone())
    }
}

#[derive(Debug, PartialEq)]
pub(in crate::feature) enum TypedValue<'a> {
    Null,
    Str(Cow<'a, str>),
    Bool(bool),
    I64(i64),
    F64(f64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Type {
    Null,
    Str,
    Bool,
    I64,
    F64,
}

impl<'a> TypedValue<'a> {
    fn primative_type(&self) -> Type {
        match self {
            Self::Null => Type::Null,
            Self::Str(_) => Type::Str,
            Self::Bool(_) => Type::Bool,
            Self::I64(_) => Type::I64,
            Self::F64(_) => Type::F64,
        }
    }
}

impl<'a> Feature for KeyValue<'a> {
    fn similarity(a: &mut Self, b: &mut Self) -> f32 {
        if a.key != b.key {
            return 0.0;
        }

        if a.value_type() != b.value_type() {
            return 0.3;
        }

        if a.typed_value() != b.typed_value() {
            return 0.7;
        }

        // At this point, we have an assumption that
        // a is basically equivalent to b. However, this does not necessarily
        // mean that a.raw_value == b.raw_value.
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_value_str_works() {
        assert_eq!(
            KeyValue::new("_", "some random garbage").typed_value(),
            &TypedValue::Str(Cow::Borrowed("some random garbage"))
        );
    }

    #[test]
    fn typed_value_i64_works() {
        assert_eq!(
            KeyValue::new("_", "100").typed_value(),
            &TypedValue::I64(100)
        );
        assert_eq!(
            KeyValue::new("_", "-52").typed_value(),
            &TypedValue::I64(-52)
        );
    }

    #[test]
    fn typed_value_f64_works() {
        assert_eq!(
            KeyValue::new("_", "-52.2").typed_value(),
            &TypedValue::F64(-52.2)
        );
        assert_eq!(
            KeyValue::new("_", "-52.0").typed_value(),
            &TypedValue::F64(-52.0)
        );
        assert_eq!(
            KeyValue::new("_", "3882.0").typed_value(),
            &TypedValue::F64(3882.0)
        );
    }

    #[test]
    fn typed_value_null_works() {
        assert_eq!(KeyValue::new("_", "null").typed_value(), &TypedValue::Null);
        assert_eq!(KeyValue::new("_", "nil").typed_value(), &TypedValue::Null);
    }
}
