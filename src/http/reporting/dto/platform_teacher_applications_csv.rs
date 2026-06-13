use crate::application::reporting::platform_csv_exports::PlatformTeacherApplicationExportRowOutput;
use crate::http::reporting::dto::csv::{csv_optional, csv_value};

pub fn platform_teacher_applications_csv(
    rows: &[PlatformTeacherApplicationExportRowOutput],
) -> String {
    let mut csv = String::from(
        "application_id,applicant_user_id,requested_scope,requested_organization_id,requested_course_id,organization_sponsor_id,status,reviewer_id,decision_reason,portfolio_links,created_at,updated_at,decided_at\n",
    );
    for row in rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            row.application_id,
            row.applicant_user_id,
            csv_value(&row.requested_scope),
            csv_optional(row.requested_organization_id),
            csv_optional(row.requested_course_id),
            csv_optional(row.organization_sponsor_id),
            csv_value(&row.status),
            csv_optional(row.reviewer_id),
            csv_value(&row.decision_reason),
            csv_value(&row.portfolio_links),
            row.created_at,
            row.updated_at,
            csv_optional(row.decided_at.as_ref())
        ));
    }
    csv
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::platform_teacher_applications_csv;
    use crate::application::reporting::platform_csv_exports::PlatformTeacherApplicationExportRowOutput;

    #[test]
    fn keeps_legacy_teacher_applications_csv_shape() {
        let now = Utc::now();
        let csv = platform_teacher_applications_csv(&[PlatformTeacherApplicationExportRowOutput {
            application_id: 7,
            applicant_user_id: 42,
            requested_scope: "platform".to_string(),
            requested_organization_id: None,
            requested_course_id: None,
            organization_sponsor_id: None,
            status: "submitted".to_string(),
            reviewer_id: None,
            decision_reason: "needs, review".to_string(),
            portfolio_links: "[\"url\"]".to_string(),
            created_at: now,
            updated_at: now,
            decided_at: None,
        }]);

        assert!(csv.starts_with("application_id,applicant_user_id"));
        assert!(csv.contains("7,42,platform"));
        assert!(csv.contains("\"needs, review\""));
    }
}
