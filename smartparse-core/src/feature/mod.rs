mod identifier;
mod metadata;

#[cfg(test)]
mod testutil;

use self::metadata::{Metadata, Source};

use std::borrow::Cow;
use std::str::FromStr;

pub use self::identifier::identify;

/// A Feature is an identifying object found on a given item that can be used
/// for comparison and clustering.
#[derive(Debug)]
pub struct Feature<'a> {
    key: Cow<'a, str>,
    raw_value: Cow<'a, str>,

    /// The typed value is only created (parsed) when needed.
    /// An example where this is not needed is if this feature
    /// does not match the cluster, and is discarded, knowing the type is not useful.
    typed_value: Option<TypedValue<'a>>,

    metadata: Metadata,
}

impl<'a> Feature<'a> {
    /// Create a new Feature from a key and a value.
    pub fn new(key: impl Into<Cow<'a, str>>, raw_value: impl Into<Cow<'a, str>>) -> Self {
        Self {
            key: key.into(),
            raw_value: raw_value.into(),
            typed_value: None,
            metadata: Metadata::default(),
        }
    }

    /// Create a new Feature from a key and a value, providing an already typed value.
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
            metadata: Metadata::default(),
        }
    }

    /// The key of the Feature (empty if doesn't exist).
    pub fn key(&self) -> &Cow<str> {
        &self.key
    }

    /// Set the source of the Feature.
    pub fn source(mut self, source: Source) -> Self {
        self.metadata.source(source);
        self
    }

    /// Returns a f32 between [0.0, 1.0] representing how similar a and b are.
    /// Returning 1.0 means that a and b are equal.
    /// Returning 0.0 means that a and b cannot be grouped in any form, anything above 0.0
    /// means there is some similarity.
    pub fn similarity(&mut self, other: &mut Feature) -> f32 {
        if self.key != other.key {
            return 0.0;
        }

        if self.value_type() != other.value_type() {
            return 0.3;
        }

        if self.typed_value() != other.typed_value() {
            return 0.7;
        }

        // At this point, we have an assumption that
        // self is basically equivalent to other. However, this does not necessarily
        // mean that self.raw_value == other.raw_value.
        1.0
    }

    /// Return the primative type of value.
    pub fn value_type(&mut self) -> Type {
        self.typed_value().primative_type()
    }

    /// Parse the raw string value into a typed value. Uses the
    /// cache / sets the cache.
    pub fn typed_value(&mut self) -> &TypedValue {
        if self.typed_value.is_some() {
            return self.typed_value.as_ref().expect("exists");
        }

        self.typed_value = Some(self.parse_typed_value());
        self.typed_value.as_ref().expect("exists")
    }

    /// Get a reference to the typed value, will not cache.
    /// This doesn't require mutable access.
    pub fn typed_value_no_cache(&self) -> Cow<TypedValue> {
        if self.typed_value.is_some() {
            return Cow::Borrowed(self.typed_value.as_ref().expect("exists"));
        }

        Cow::Owned(self.parse_typed_value())
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

#[derive(Debug, PartialEq, Clone)]
pub enum TypedValue<'a> {
    Null,
    Str(Cow<'a, str>),
    Bool(bool),
    I64(i64),
    F64(f64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_value_str_works() {
        assert_eq!(
            Feature::new("_", "some random garbage").typed_value(),
            &TypedValue::Str(Cow::Borrowed("some random garbage"))
        );
    }

    #[test]
    fn typed_value_i64_works() {
        assert_eq!(
            Feature::new("_", "100").typed_value(),
            &TypedValue::I64(100)
        );
        assert_eq!(
            Feature::new("_", "-52").typed_value(),
            &TypedValue::I64(-52)
        );
    }

    #[test]
    fn typed_value_f64_works() {
        assert_eq!(
            Feature::new("_", "-52.2").typed_value(),
            &TypedValue::F64(-52.2)
        );
        assert_eq!(
            Feature::new("_", "-52.0").typed_value(),
            &TypedValue::F64(-52.0)
        );
        assert_eq!(
            Feature::new("_", "3882.0").typed_value(),
            &TypedValue::F64(3882.0)
        );
    }

    #[test]
    fn typed_value_null_works() {
        assert_eq!(Feature::new("_", "null").typed_value(), &TypedValue::Null);
        assert_eq!(Feature::new("_", "nil").typed_value(), &TypedValue::Null);
    }
}
