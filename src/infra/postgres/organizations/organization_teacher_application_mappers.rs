use crate::application::organizations::list_organization_teacher_applications::{
    OrganizationTeacherApplicationCourseOutput, OrganizationTeacherApplicationItemOutput,
    OrganizationTeacherApplicationOrganizationOutput, TeacherApplicationUserSummaryOutput,
};
use crate::infra::postgres::organizations::organization_teacher_application_context::OrganizationTeacherApplicationContext;
use crate::models::teacher_application::TeacherApplication;

pub fn organization_teacher_application_item(
    application: &TeacherApplication,
    organization_id: i32,
    context: &OrganizationTeacherApplicationContext,
) -> OrganizationTeacherApplicationItemOutput {
    OrganizationTeacherApplicationItemOutput {
        id: application.id,
        applicant: context
            .users
            .get(&application.applicant_user_id)
            .cloned()
            .unwrap_or(TeacherApplicationUserSummaryOutput {
                id: application.applicant_user_id,
                name: "Unknown applicant".to_string(),
                email: "unknown@example.invalid".to_string(),
            }),
        requested_scope: application.requested_scope.clone(),
        requested_organization: application
            .requested_organization_id
            .map(|id| organization_summary(id, context)),
        requested_course: application.requested_course_id.map(|id| {
            OrganizationTeacherApplicationCourseOutput {
                id,
                title: context
                    .courses
                    .get(&id)
                    .cloned()
                    .unwrap_or_else(|| format!("Course {id}")),
            }
        }),
        sponsored_by_this_organization: application.organization_sponsor_id
            == Some(organization_id),
        requested_for_this_organization: application.requested_organization_id
            == Some(organization_id),
        experience_summary: application.experience_summary.clone(),
        portfolio_links: portfolio_links_from_json(&application.portfolio_links),
        status: application.status.clone(),
        reviewer: application
            .reviewer_id
            .and_then(|reviewer_id| context.users.get(&reviewer_id).cloned()),
        decision_reason: application.decision_reason.clone(),
        audit: context
            .audits
            .get(&application.id)
            .cloned()
            .unwrap_or_default(),
        created_at: application.created_at,
        updated_at: application.updated_at,
        decided_at: application.decided_at,
    }
}

fn organization_summary(
    id: i32,
    context: &OrganizationTeacherApplicationContext,
) -> OrganizationTeacherApplicationOrganizationOutput {
    OrganizationTeacherApplicationOrganizationOutput {
        id,
        name: context
            .organizations
            .get(&id)
            .cloned()
            .unwrap_or_else(|| format!("Organization {id}")),
    }
}

fn portfolio_links_from_json(portfolio_links: &serde_json::Value) -> Vec<String> {
    portfolio_links
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str())
                .map(|link| link.trim().to_string())
                .filter(|link| !link.is_empty())
                .collect()
        })
        .unwrap_or_default()
}
