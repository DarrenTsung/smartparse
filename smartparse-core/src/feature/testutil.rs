use std::collections::HashSet;

use crate::feature::Feature;

#[cfg(test)]
pub(in crate::feature) fn assert_similarity_equal(mut a: Vec<Feature>, mut b: Vec<Feature>) {
    let a_debug = format!("{:?}", a);

    let mut used_indices = HashSet::new();
    for a_item in &mut a {
        let mut found_index = None;
        for (index, b_item) in b.iter_mut().enumerate() {
            if used_indices.contains(&index) {
                continue;
            }

            if Feature::similarity(a_item, b_item) == 1.0 {
                found_index = Some(index);
                break;
            }
        }

        if let Some(index) = found_index {
            used_indices.insert(index);
        } else {
            panic!(
                "Failed to assert_similarity_equal for item: {:?}\na: {}\nb: {:?}",
                a_item, a_debug, b
            );
        }
    }
}
