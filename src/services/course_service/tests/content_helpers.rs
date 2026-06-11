use super::*;

// ── course_title_search_pattern ──

#[test]
fn wraps_search_with_wildcards() {
    assert_eq!(course_title_search_pattern("hello"), "%hello%");
}

#[test]
fn escapes_like_special_chars() {
    assert_eq!(course_title_search_pattern("50%"), r"%50\%%");
    assert_eq!(course_title_search_pattern("a_b"), r"%a\_b%");
    assert_eq!(course_title_search_pattern(r"a\b"), r"%a\\b%");
}

#[test]
fn escapes_combined_special_chars() {
    assert_eq!(course_title_search_pattern(r"a\%_"), r"%a\\\%\_%");
}

#[test]
fn empty_search_returns_wildcards() {
    assert_eq!(course_title_search_pattern(""), "%%");
}

// ── normalize_optional_string ──

#[test]
fn returns_trimmed_optional_string() {
    assert_eq!(
        normalize_optional_string(Some("  hello  ".into())),
        Some("hello".into())
    );
}

#[test]
fn returns_none_for_empty_or_whitespace_os() {
    assert_eq!(normalize_optional_string(None), None);
    assert_eq!(normalize_optional_string(Some("".into())), None);
    assert_eq!(normalize_optional_string(Some("   ".into())), None);
}

// ── content_display_state ──

#[test]
fn text_content_with_data_is_ready() {
    assert_eq!(content_display_state("text", Some("data"), &None), "ready");
    assert_eq!(
        content_display_state("article", Some("data"), &None),
        "ready"
    );
}

#[test]
fn text_content_without_data_is_unavailable() {
    assert_eq!(content_display_state("text", None, &None), "unavailable");
    assert_eq!(
        content_display_state("article", Some(""), &None),
        "unavailable"
    );
}

#[test]
fn media_content_without_data_is_unprocessed_upload() {
    assert_eq!(
        content_display_state("video", None, &None),
        "unprocessed_upload"
    );
    assert_eq!(
        content_display_state("video", Some(""), &None),
        "unprocessed_upload"
    );
    assert_eq!(
        content_display_state("document", None, &None),
        "unprocessed_upload"
    );
}

#[test]
fn media_with_no_processing_status_is_uploaded() {
    assert_eq!(
        content_display_state("video", Some("data"), &None),
        "uploaded"
    );
}

#[test]
fn processing_status_maps_correctly() {
    assert_eq!(
        content_display_state("video", Some("data"), &Some(("queued".into(), None))),
        "processing"
    );
    assert_eq!(
        content_display_state("video", Some("data"), &Some(("processing".into(), None))),
        "processing"
    );
    assert_eq!(
        content_display_state("video", Some("data"), &Some(("done".into(), None))),
        "ready"
    );
    assert_eq!(
        content_display_state(
            "video",
            Some("data"),
            &Some(("failed".into(), Some("error".into())))
        ),
        "failed_processing"
    );
}

#[test]
fn unknown_processing_status_is_uploaded() {
    assert_eq!(
        content_display_state("video", Some("data"), &Some(("unknown".into(), None))),
        "uploaded"
    );
}

// ── is_media_content_type ──

#[test]
fn recognizes_media_types() {
    assert!(is_media_content_type("video"));
    assert!(is_media_content_type("video/mp4"));
    assert!(is_media_content_type("document"));
    assert!(is_media_content_type("pdf"));
    assert!(is_media_content_type("application/pdf"));
}

#[test]
fn rejects_non_media_types() {
    assert!(!is_media_content_type("text"));
    assert!(!is_media_content_type("article"));
    assert!(!is_media_content_type(""));
    assert!(!is_media_content_type("image"));
}

// ── teacher_content_publication_status ──

#[test]
fn inherits_course_lifecycle_status() {
    assert_eq!(
        teacher_content_publication_status(COURSE_STATUS_DRAFT),
        "inherits_course_draft"
    );
    assert_eq!(
        teacher_content_publication_status(COURSE_STATUS_PUBLISHED),
        "inherits_course_published"
    );
}
