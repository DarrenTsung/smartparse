use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub enum Source {
    Json,
    Logfmt,
    Custom(Cow<'static, str>),
}
