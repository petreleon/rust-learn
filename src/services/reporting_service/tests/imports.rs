use super::*;

// ── csv_value ──

#[test]
fn plain_value_passes_through() {
    assert_eq!(csv_value("hello"), "hello");
    assert_eq!(csv_value("no special chars"), "no special chars");
}

#[test]
fn value_with_comma_is_quoted() {
    assert_eq!(csv_value("a,b"), "\"a,b\"");
}

#[test]
fn value_with_quote_is_escaped_and_quoted() {
    assert_eq!(csv_value(r#"say "hi""#), r#""say ""hi""""#);
}

#[test]
fn value_with_newline_is_quoted() {
    assert_eq!(csv_value("line1\nline2"), "\"line1\nline2\"");
}

#[test]
fn empty_value_passes_through() {
    assert_eq!(csv_value(""), "");
}

// ── csv_optional ──

#[test]
fn some_value_to_string() {
    assert_eq!(csv_optional(Some(42)), "42");
}

#[test]
fn none_is_empty() {
    assert_eq!(csv_optional(None::<i32>), "");
}

// ── platform_report_csv ──

#[test]
fn generates_platform_report_csv() {
    let summary = PlatformReportSummary {
        total_users: 10,
        total_organizations: 3,
        total_courses: 5,
        total_wallets: 8,
        total_notifications: 25,
    };
    let csv = platform_report_csv(&summary);
    assert!(csv.starts_with("metric,value\n"));
    assert!(csv.contains("users,10\n"));
    assert!(csv.contains("organizations,3\n"));
    assert!(csv.contains("courses,5\n"));
    assert!(csv.contains("wallets,8\n"));
    assert!(csv.contains("notifications,25\n"));
}
