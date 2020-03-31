use super::super::Transform;

/// IdentityTransform does not modify the input.
pub struct IdentityTransform {}
impl<'a> Transform<'a, &'a str> for IdentityTransform {
    fn transform(&self, input: &'a str) -> &'a str {
        input
    }
}
