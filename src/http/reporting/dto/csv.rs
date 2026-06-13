pub(crate) fn csv_value(value: impl AsRef<str>) -> String {
    let value = value.as_ref();
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

pub(crate) fn csv_optional(value: Option<impl ToString>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::{csv_optional, csv_value};

    #[test]
    fn escapes_csv_values_like_legacy_exports() {
        assert_eq!(csv_value("hello"), "hello");
        assert_eq!(csv_value("a,b"), "\"a,b\"");
        assert_eq!(csv_value(r#"say "hi""#), r#""say ""hi""""#);
        assert_eq!(csv_value("line1\nline2"), "\"line1\nline2\"");
        assert_eq!(csv_optional(Some(42)), "42");
        assert_eq!(csv_optional(None::<i32>), "");
    }
}
