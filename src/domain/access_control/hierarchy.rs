use std::cmp::Ordering;

pub fn compare_hierarchy_levels(first: Option<i32>, second: Option<i32>) -> Ordering {
    match (first, second) {
        (Some(first_level), Some(second_level)) => second_level.cmp(&first_level),
        (None, None) => Ordering::Equal,
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_numbered_level_has_greater_authority() {
        assert_eq!(
            compare_hierarchy_levels(Some(0), Some(1)),
            Ordering::Greater
        );
        assert_eq!(compare_hierarchy_levels(Some(2), Some(1)), Ordering::Less);
    }

    #[test]
    fn assigned_role_beats_missing_role() {
        assert_eq!(compare_hierarchy_levels(Some(1), None), Ordering::Greater);
        assert_eq!(compare_hierarchy_levels(None, Some(1)), Ordering::Less);
        assert_eq!(compare_hierarchy_levels(None, None), Ordering::Equal);
    }
}
