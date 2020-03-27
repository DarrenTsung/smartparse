mod kv;
mod source;

/// A Feature is an identifying object found on a given item that can be used
/// for comparison and clustering.
pub trait Feature {
    /// Returns a f32 between [0.0, 1.0] representing how similar a and b are.
    /// Returning 1.0 means that a and b are equal.
    /// Returning 0.0 means that a and b cannot be grouped in any form, anything above 0.0
    /// means there is some similarity.
    fn similarity(a: &mut Self, b: &mut Self) -> f32;
}
