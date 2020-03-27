use std::str::FromStr;

use super::Feature;

#[derive(Debug)]
pub struct KeyValue<'a> {
    key: &'a str,
    raw_value: &'a str,

    /// The typed value is only created (parsed) when needed.
    /// An example where this is not needed is if this KeyValue feature
    /// does not match the cluster, and is discarded, knowing the type is not useful.
    typed_value: Option<TypedValue<'a>>,
}

impl<'a> KeyValue<'a> {
    /// Create a new KeyValue from a key and a value.
    pub fn new(key: &'a str, raw_value: &'a str) -> Self {
        Self {
            key,
            raw_value,
            typed_value: None,
        }
    }

    fn value_type(&mut self) -> Type {
        self.typed_value().primative_type()
    }

    fn typed_value(&mut self) -> &TypedValue {
        if self.typed_value.is_some() {
            return self.typed_value.as_ref().expect("exists");
        }

        self.typed_value = Some(if let Ok(val) = i32::from_str(self.raw_value) {
            TypedValue::I32(val)
        } else if let Ok(val) = f32::from_str(self.raw_value) {
            TypedValue::F32(val)
        } else {
            TypedValue::Str(self.raw_value.clone())
        });
        self.typed_value.as_ref().expect("exists")
    }
}

#[derive(Debug, PartialEq)]
enum TypedValue<'a> {
    Str(&'a str),
    I32(i32),
    F32(f32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Type {
    Str,
    I32,
    F32,
}

impl<'a> TypedValue<'a> {
    fn primative_type(&self) -> Type {
        match self {
            Self::Str(_) => Type::Str,
            Self::I32(_) => Type::I32,
            Self::F32(_) => Type::F32,
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
        // a is equivalent to b.
        debug_assert!(a.key == b.key && a.raw_value == b.raw_value);
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
            &TypedValue::Str("some random garbage")
        );
    }

    #[test]
    fn typed_value_i32_works() {
        assert_eq!(
            KeyValue::new("_", "100").typed_value(),
            &TypedValue::I32(100)
        );
        assert_eq!(
            KeyValue::new("_", "-52").typed_value(),
            &TypedValue::I32(-52)
        );
    }

    #[test]
    fn typed_value_f32_works() {
        assert_eq!(
            KeyValue::new("_", "-52.2").typed_value(),
            &TypedValue::F32(-52.2)
        );
        assert_eq!(
            KeyValue::new("_", "-52.0").typed_value(),
            &TypedValue::F32(-52.0)
        );
        assert_eq!(
            KeyValue::new("_", "3882.0").typed_value(),
            &TypedValue::F32(3882.0)
        );
    }
}
