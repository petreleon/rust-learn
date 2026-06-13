use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_csv_exports::{
    PlatformCsvExportError, PlatformTeacherApplicationExportRowOutput,
};
use crate::db::schema::teacher_applications;
use crate::infra::postgres::reporting::platform_csv_export_store::map_diesel_error;
use crate::models::teacher_application::TeacherApplication;

pub(super) async fn load_teacher_application_export_rows(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<PlatformTeacherApplicationExportRowOutput>, PlatformCsvExportError> {
    teacher_applications::table
        .order(teacher_applications::created_at.desc())
        .limit(1000)
        .load::<TeacherApplication>(conn)
        .await
        .map(|rows| rows.into_iter().map(map_teacher_application).collect())
        .map_err(map_diesel_error)
}

fn map_teacher_application(
    application: TeacherApplication,
) -> PlatformTeacherApplicationExportRowOutput {
    PlatformTeacherApplicationExportRowOutput {
        application_id: application.id,
        applicant_user_id: application.applicant_user_id,
        requested_scope: application.requested_scope,
        requested_organization_id: application.requested_organization_id,
        requested_course_id: application.requested_course_id,
        organization_sponsor_id: application.organization_sponsor_id,
        status: application.status,
        reviewer_id: application.reviewer_id,
        decision_reason: application.decision_reason.unwrap_or_default(),
        portfolio_links: application.portfolio_links.to_string(),
        created_at: application.created_at,
        updated_at: application.updated_at,
        decided_at: application.decided_at,
    }
}
