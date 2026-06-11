use super::*;

// ── teacher_course_dashboard_permission_names ──

#[test]
fn includes_all_expected_permissions() {
    let names = teacher_course_dashboard_permission_names();
    assert!(names.contains(&Permissions::MANAGE_COURSE_SETTINGS.to_string()));
    assert!(names.contains(&Permissions::CREATE_CONTENT.to_string()));
    assert!(names.contains(&Permissions::MODIFY_CONTENT.to_string()));
    assert!(names.contains(&Permissions::APPROVE_COURSE_CONTENT.to_string()));
    assert!(names.contains(&Permissions::MANAGE_COURSE_ENROLLMENTS.to_string()));
    assert!(names.contains(&Permissions::APPROVE_COURSE_JOIN_REQUESTS.to_string()));
    assert!(names.contains(&Permissions::VIEW_COURSE_REWARD_STATUS.to_string()));
    assert!(names.contains(&Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string()));
    assert!(names.contains(&Permissions::MANAGE_COURSE_REWARD_RULES.to_string()));
    assert_eq!(names.len(), 9);
}

// ── TeacherCoursePermissionSummary::has_teacher_access ──

fn summary_with(permission: &str) -> TeacherCoursePermissionSummary {
    let mut summary = TeacherCoursePermissionSummary {
        can_manage_settings: false,
        can_manage_content: false,
        can_manage_enrollments: false,
        can_view_reward_candidates: false,
        can_approve_reward_candidates: false,
        can_manage_reward_rules: false,
    };
    match permission {
        "can_manage_settings" => summary.can_manage_settings = true,
        "can_manage_content" => summary.can_manage_content = true,
        "can_manage_enrollments" => summary.can_manage_enrollments = true,
        "can_view_reward_candidates" => summary.can_view_reward_candidates = true,
        "can_approve_reward_candidates" => summary.can_approve_reward_candidates = true,
        "can_manage_reward_rules" => summary.can_manage_reward_rules = true,
        _ => {}
    }
    summary
}

#[test]
fn has_access_when_any_permission_is_true() {
    let perms = [
        "can_manage_settings",
        "can_manage_content",
        "can_manage_enrollments",
        "can_view_reward_candidates",
        "can_approve_reward_candidates",
        "can_manage_reward_rules",
    ];
    for p in perms {
        assert!(
            summary_with(p).has_teacher_access(),
            "should have access with {}",
            p
        );
    }
}

#[test]
fn no_access_when_all_permissions_false() {
    let summary = TeacherCoursePermissionSummary {
        can_manage_settings: false,
        can_manage_content: false,
        can_manage_enrollments: false,
        can_view_reward_candidates: false,
        can_approve_reward_candidates: false,
        can_manage_reward_rules: false,
    };
    assert!(!summary.has_teacher_access());
}

// ── normalize_course_status ──

#[test]
fn normalizes_all_valid_course_statuses() {
    let statuses = vec![
        COURSE_STATUS_DRAFT,
        COURSE_STATUS_SUBMITTED,
        COURSE_STATUS_NEEDS_CHANGES,
        COURSE_STATUS_APPROVED,
        COURSE_STATUS_PUBLISHED,
        COURSE_STATUS_ARCHIVED,
        COURSE_STATUS_SUSPENDED,
    ];
    for status in statuses {
        assert_eq!(normalize_course_status(status).unwrap(), status);
    }
}

#[test]
fn normalizes_course_status_case_and_whitespace() {
    assert_eq!(
        normalize_course_status("  DRAFT  ").unwrap(),
        COURSE_STATUS_DRAFT
    );
    assert_eq!(
        normalize_course_status("Published").unwrap(),
        COURSE_STATUS_PUBLISHED
    );
}

#[test]
fn rejects_invalid_course_status() {
    assert!(normalize_course_status("").is_err());
    assert!(normalize_course_status("unknown").is_err());
    assert!(normalize_course_status("deleted").is_err());
}
