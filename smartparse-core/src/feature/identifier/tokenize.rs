use std::marker::PhantomData;

use super::FeatureIdentifier;

use crate::feature::Feature;

trait Transform<'a, TOutput> {
    fn transform(&self, input: &'a str) -> TOutput;
}

struct Tokenize<'a, T: Transform<'a, TOutput>, TOutput> {
    transformer: T,

    _output: PhantomData<&'a TOutput>,
}

struct IdentityTransform {}
impl<'a> Transform<'a, &'a str> for IdentityTransform {
    fn transform(&self, input: &'a str) -> &'a str {
        input
    }
}

impl<'a> Tokenize<'a, IdentityTransform, &'a str> {
    pub fn new() -> Self {
        Self {
            transformer: IdentityTransform {},
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
    fn identify(&self, input: &'a str) -> Option<Vec<Feature>> {
        self.tokenize(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_as_expected() {
        let tokens = Tokenize::new()
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
