use std::marker::PhantomData;

use super::FeatureIdentifier;

use crate::feature::Feature;

pub(super) mod transforms;

pub(super) trait Transform<'a, TOutput> {
    fn transform(&self, input: &'a str) -> TOutput;
}

pub(super) struct Tokenize<'a, T: Transform<'a, TOutput>, TOutput> {
    transformer: T,

    _output: PhantomData<&'a TOutput>,
}

impl<'a> Tokenize<'a, transforms::feature::FeatureTransform, Feature<'a>> {
    pub fn new() -> Self {
        Self {
            transformer: transforms::feature::FeatureTransform {},
            _output: PhantomData,
        }
    }
}

impl<'a> Tokenize<'a, transforms::identity::IdentityTransform, &'a str> {
    /// Return a Tokenize that does not return Features. Only useful for tests.
    fn identity() -> Self {
        Self {
            transformer: transforms::identity::IdentityTransform {},
            _output: PhantomData,
        }
    }
}

impl<'a, T: Transform<'a, TOutput>, TOutput> Tokenize<'a, T, TOutput> {
    fn tokenize(&self, input: &'a str) -> Option<Vec<TOutput>> {
        let mut outputs = vec![];
        for token in input.split_whitespace() {
            outputs.push(self.transformer.transform(token));
        }
        Some(outputs)
    }
}

impl<'a, T: Transform<'a, Feature<'a>>> FeatureIdentifier<'a> for Tokenize<'a, T, Feature<'a>> {
    fn identify(&self, input: &'a str) -> Option<Vec<Feature<'a>>> {
        self.tokenize(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_as_expected() {
        let tokens = Tokenize::identity()
            .tokenize(r#"12:42:53.546 INFO AppDelegate.loadSplashscreen():153 - Opening trackers"#);
        assert_eq!(
            tokens.unwrap(),
            vec![
                "12:42:53.546",
                "INFO",
                "AppDelegate.loadSplashscreen():153",
                "-",
                "Opening",
                "trackers",
            ]
        );
    }
}
