use chrono::{DateTime, Utc};

use crate::application::reporting::platform_csv_exports::PlatformTeacherApplicationExportRowOutput;
use crate::domain::teacher_applications::portfolio::TeacherApplicationPortfolioLinks;
use crate::domain::teacher_applications::scope::TeacherApplicationScope;
use crate::domain::teacher_applications::status::TeacherApplicationStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PlatformTeacherApplicationExportFact {
    pub application_id: i64,
    pub applicant_user_id: i32,
    pub requested_scope: TeacherApplicationScope,
    pub requested_organization_id: Option<i32>,
    pub requested_course_id: Option<i32>,
    pub organization_sponsor_id: Option<i32>,
    pub status: TeacherApplicationStatus,
    pub reviewer_id: Option<i32>,
    pub decision_reason: Option<String>,
    pub portfolio_links: TeacherApplicationPortfolioLinks,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub decided_at: Option<DateTime<Utc>>,
}

pub(crate) fn platform_teacher_application_export_row(
    fact: PlatformTeacherApplicationExportFact,
) -> PlatformTeacherApplicationExportRowOutput {
    PlatformTeacherApplicationExportRowOutput {
        application_id: fact.application_id,
        applicant_user_id: fact.applicant_user_id,
        requested_scope: fact.requested_scope,
        requested_organization_id: fact.requested_organization_id,
        requested_course_id: fact.requested_course_id,
        organization_sponsor_id: fact.organization_sponsor_id,
        status: fact.status,
        reviewer_id: fact.reviewer_id,
        decision_reason: fact.decision_reason.unwrap_or_default(),
        portfolio_links: fact.portfolio_links.to_string(),
        created_at: fact.created_at,
        updated_at: fact.updated_at,
        decided_at: fact.decided_at,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn builds_teacher_application_export_row_from_fact() {
        let now = Utc::now();

        let row = platform_teacher_application_export_row(PlatformTeacherApplicationExportFact {
            application_id: 7,
            applicant_user_id: 42,
            requested_scope: TeacherApplicationScope::Organization,
            requested_organization_id: Some(11),
            requested_course_id: None,
            organization_sponsor_id: Some(12),
            status: TeacherApplicationStatus::Approved,
            reviewer_id: Some(3),
            decision_reason: Some("qualified".to_string()),
            portfolio_links: json!(["https://example.com"]),
            created_at: now,
            updated_at: now,
            decided_at: Some(now),
        });

        assert_eq!(row.application_id, 7);
        assert_eq!(row.applicant_user_id, 42);
        assert_eq!(row.requested_scope, TeacherApplicationScope::Organization);
        assert_eq!(row.status, TeacherApplicationStatus::Approved);
        assert_eq!(row.decision_reason, "qualified");
        assert_eq!(row.portfolio_links, "[\"https://example.com\"]");
        assert_eq!(row.decided_at, Some(now));
    }

    #[test]
    fn defaults_missing_decision_reason() {
        let now = Utc::now();

        let row = platform_teacher_application_export_row(PlatformTeacherApplicationExportFact {
            application_id: 1,
            applicant_user_id: 2,
            requested_scope: TeacherApplicationScope::Platform,
            requested_organization_id: None,
            requested_course_id: None,
            organization_sponsor_id: None,
            status: TeacherApplicationStatus::Submitted,
            reviewer_id: None,
            decision_reason: None,
            portfolio_links: json!([]),
            created_at: now,
            updated_at: now,
            decided_at: None,
        });

        assert_eq!(row.decision_reason, "");
        assert_eq!(row.portfolio_links, "[]");
    }
}
