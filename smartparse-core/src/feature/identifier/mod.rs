mod json;

pub(in crate::feature) trait FeatureIdentifier<'a> {
    type Feature;

    fn identify(&self, input: &'a str) -> Option<Vec<Self::Feature>>;
    fn source(&self) -> crate::feature::Source;
}