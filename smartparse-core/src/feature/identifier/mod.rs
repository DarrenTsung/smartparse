mod json;

use crate::feature::Feature;

pub(in crate::feature) trait FeatureIdentifier<'a> {
    fn identify(&self, input: &'a str) -> Option<Vec<Feature>>;
    fn source(&self) -> crate::feature::Source;
}
