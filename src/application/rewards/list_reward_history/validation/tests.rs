use super::*;

#[test]
fn normalizes_status_and_clamps_pagination() {
    let filter = validated_filter(
        42,
        StudentRewardHistoryQuery {
            course_id: Some(7),
            status: Some(" wallet-credited ".to_string()),
            limit: Some(500),
            offset: Some(-3),
        },
    )
    .unwrap();

    assert_eq!(filter.student_user_id, 42);
    assert_eq!(filter.course_id, Some(7));
    assert_eq!(filter.status.as_deref(), Some("wallet_credited"));
    assert_eq!(filter.limit, 100);
    assert_eq!(filter.offset, 0);
}

#[test]
fn rejects_unknown_status() {
    assert!(validated_filter(
        42,
        StudentRewardHistoryQuery {
            status: Some("not-real".to_string()),
            ..Default::default()
        },
    )
    .is_err());
}
