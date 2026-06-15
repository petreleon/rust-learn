use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_csv_exports::{
    platform_teacher_application_export_row, PlatformCsvExportError,
    PlatformTeacherApplicationExportFact, PlatformTeacherApplicationExportRowOutput,
};
use crate::db::schema::teacher_applications;
use crate::domain::teacher_applications::scope::TeacherApplicationScope;
use crate::domain::teacher_applications::status::TeacherApplicationStatus;
use crate::infra::postgres::reporting::platform_csv_export_mappers::map_diesel_error;
use crate::models::teacher_application::TeacherApplication;

pub(super) async fn load_teacher_application_export_rows(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<PlatformTeacherApplicationExportRowOutput>, PlatformCsvExportError> {
    teacher_applications::table
        .order(teacher_applications::created_at.desc())
        .limit(1000)
        .load::<TeacherApplication>(conn)
        .await
        .map_err(map_diesel_error)?
        .into_iter()
        .map(teacher_application_fact)
        .map(|fact| fact.map(platform_teacher_application_export_row))
        .collect()
}

fn teacher_application_fact(
    application: TeacherApplication,
) -> Result<PlatformTeacherApplicationExportFact, PlatformCsvExportError> {
    let requested_scope = TeacherApplicationScope::parse(&application.requested_scope)
        .map_err(|error| PlatformCsvExportError::Database(error.to_string()))?;
    let status = TeacherApplicationStatus::parse(&application.status)
        .map_err(|error| PlatformCsvExportError::Database(error.to_string()))?;

    Ok(PlatformTeacherApplicationExportFact {
        application_id: application.id,
        applicant_user_id: application.applicant_user_id,
        requested_scope,
        requested_organization_id: application.requested_organization_id,
        requested_course_id: application.requested_course_id,
        organization_sponsor_id: application.organization_sponsor_id,
        status,
        reviewer_id: application.reviewer_id,
        decision_reason: application.decision_reason,
        portfolio_links: application.portfolio_links,
        created_at: application.created_at,
        updated_at: application.updated_at,
        decided_at: application.decided_at,
    })
}
