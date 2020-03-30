#[derive(Debug, PartialEq, Default)]
pub struct Metadata {
    source: Option<Source>,
}

impl Metadata {
    /// Set the source. An example of this would be Source::Json, which
    /// indicates that the Feature was created by the Json identifier.
    pub fn source(&mut self, source: Source) {
        self.source = Some(source);
    }
}

#[derive(Debug, PartialEq)]
pub enum Source {
    Json,
    Tokenize,
}
