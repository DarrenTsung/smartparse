use std::borrow::Cow;

use super::Feature;

#[derive(Debug, PartialEq)]
pub enum Source {
    Json,
    Logfmt,
    Custom(Cow<'static, str>),
}

impl Feature for Source {
    fn similarity(a: &mut Self, b: &mut Self) -> f32 {
        if a == b {
            1.0
        } else {
            0.0
        }
    }
}
