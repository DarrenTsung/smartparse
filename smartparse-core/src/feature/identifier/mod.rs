mod json;
mod tokenize;

use crate::feature::Feature;

use self::json::Json;
use self::tokenize::Tokenize;

pub(in crate::feature) trait FeatureIdentifier<'a> {
    fn identify(&self, input: &'a str) -> Option<Vec<Feature<'a>>>;
}

pub fn identify(input: &str) -> Vec<Feature> {
    // Try Json, then tokenize parsing. If Json parsing succeeds, then
    // we don't need to try tokenize.
    if let Some(features) = Json::new().identify(input) {
        features
    } else if let Some(features) = Tokenize::new().identify(input) {
        features
    } else {
        vec![]
    }
}
