mod json;
mod tokenize;

use crate::feature::Feature;

pub(in crate::feature) trait FeatureIdentifier<'a> {
    fn identify(&self, input: &'a str) -> Option<Vec<Feature>>;
}
