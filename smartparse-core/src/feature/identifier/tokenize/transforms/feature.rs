use super::super::Transform;
use crate::feature::Feature;

/// FeatureTransform parses the string as a feature.
pub struct FeatureTransform {}
impl<'a> Transform<'a, Feature<'a>> for FeatureTransform {
    fn transform(&self, input: &'a str) -> Feature<'a> {
        // No key, pass entire input as the value.
        Feature::new("", input)
    }
}
