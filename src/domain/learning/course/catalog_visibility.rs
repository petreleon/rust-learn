pub fn is_generated_course_title(title: &str) -> bool {
    let Some(token) = title.trim().split_whitespace().next() else {
        return false;
    };
    let parts = token.rsplitn(4, '_').collect::<Vec<_>>();

    parts.len() == 4
        && !parts[3].is_empty()
        && parts[..3].iter().all(|part| is_non_empty_number(part))
}

fn is_non_empty_number(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|ch| ch.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::is_generated_course_title;

    #[test]
    fn detects_unique_test_course_titles() {
        assert!(is_generated_course_title(
            "LifecycleCourse_123_1781600000000000000_4"
        ));
        assert!(is_generated_course_title(
            "CatalogCourse_123_1781600000000000000_4 Published Rust"
        ));
    }

    #[test]
    fn keeps_human_course_titles_visible() {
        assert!(!is_generated_course_title("Rust Ownership Systems"));
        assert!(!is_generated_course_title("Course 123"));
        assert!(!is_generated_course_title("Rust_2026"));
        assert!(!is_generated_course_title("123_456_789"));
    }
}
