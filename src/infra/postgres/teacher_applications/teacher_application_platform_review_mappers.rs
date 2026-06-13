use serde_json::Value;

use crate::application::teacher_applications::list_platform_review::{
    TeacherApplicationPlatformReviewCourseOutput, TeacherApplicationPlatformReviewItemOutput,
    TeacherApplicationPlatformReviewOrganizationOutput, TeacherApplicationPlatformReviewUserOutput,
};
use crate::infra::postgres::teacher_applications::teacher_application_platform_review_context::TeacherApplicationPlatformReviewContext;
use crate::models::teacher_application::TeacherApplication;

pub fn platform_review_item(
    application: &TeacherApplication,
    context: &TeacherApplicationPlatformReviewContext,
) -> TeacherApplicationPlatformReviewItemOutput {
    TeacherApplicationPlatformReviewItemOutput {
        id: application.id,
        applicant: context
            .users
            .get(&application.applicant_user_id)
            .cloned()
            .unwrap_or_else(|| unknown_user(application.applicant_user_id)),
        requested_scope: application.requested_scope.clone(),
        requested_organization: organization_output(
            application.requested_organization_id,
            &context.organizations,
        ),
        requested_course: course_output(application.requested_course_id, &context.courses),
        sponsor_organization: organization_output(
            application.organization_sponsor_id,
            &context.organizations,
        ),
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

fn unknown_user(id: i32) -> TeacherApplicationPlatformReviewUserOutput {
    TeacherApplicationPlatformReviewUserOutput {
        id,
        name: "Unknown applicant".to_string(),
        email: "unknown@example.invalid".to_string(),
    }
}

fn organization_output(
    id: Option<i32>,
    names: &std::collections::BTreeMap<i32, String>,
) -> Option<TeacherApplicationPlatformReviewOrganizationOutput> {
    id.map(|id| TeacherApplicationPlatformReviewOrganizationOutput {
        id,
        name: names
            .get(&id)
            .cloned()
            .unwrap_or_else(|| format!("Organization {id}")),
    })
}

fn course_output(
    id: Option<i32>,
    titles: &std::collections::BTreeMap<i32, String>,
) -> Option<TeacherApplicationPlatformReviewCourseOutput> {
    id.map(|id| TeacherApplicationPlatformReviewCourseOutput {
        id,
        title: titles
            .get(&id)
            .cloned()
            .unwrap_or_else(|| format!("Course {id}")),
    })
}

fn portfolio_links_from_json(portfolio_links: &Value) -> Vec<String> {
    portfolio_links
        .as_array()
        .map(|links| {
            links
                .iter()
                .filter_map(|link| link.as_str().map(str::trim))
                .filter(|link| !link.is_empty())
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default()
}
