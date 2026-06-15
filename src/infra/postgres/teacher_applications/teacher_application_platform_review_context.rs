use std::collections::{BTreeMap, BTreeSet};

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::teacher_applications::list_platform_review::{
    TeacherApplicationPlatformReviewAuditSummaryOutput, TeacherApplicationPlatformReviewError,
    TeacherApplicationPlatformReviewUserOutput,
};
use crate::db::schema::{courses, organizations, teacher_application_audit_events, users};
use crate::infra::postgres::teacher_applications::teacher_application_platform_review_audit::audit_summaries;
use crate::models::teacher_application::{TeacherApplication, TeacherApplicationAuditEvent};

pub struct TeacherApplicationPlatformReviewContext {
    pub users: BTreeMap<i32, TeacherApplicationPlatformReviewUserOutput>,
    pub organizations: BTreeMap<i32, String>,
    pub courses: BTreeMap<i32, String>,
    pub audits: BTreeMap<i64, TeacherApplicationPlatformReviewAuditSummaryOutput>,
}

pub async fn build_context(
    conn: &mut AsyncPgConnection,
    applications: &[TeacherApplication],
) -> Result<TeacherApplicationPlatformReviewContext, TeacherApplicationPlatformReviewError> {
    Ok(TeacherApplicationPlatformReviewContext {
        users: load_users(conn, applications).await?,
        organizations: load_organizations(conn, applications).await?,
        courses: load_courses(conn, applications).await?,
        audits: load_audits(conn, applications).await?,
    })
}

async fn load_users(
    conn: &mut AsyncPgConnection,
    applications: &[TeacherApplication],
) -> Result<
    BTreeMap<i32, TeacherApplicationPlatformReviewUserOutput>,
    TeacherApplicationPlatformReviewError,
> {
    let ids = applications
        .iter()
        .flat_map(|application| [Some(application.applicant_user_id), application.reviewer_id])
        .flatten()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if ids.is_empty() {
        return Ok(BTreeMap::new());
    }

    users::table
        .filter(users::id.eq_any(&ids))
        .select((users::id, users::name, users::email))
        .load::<(i32, String, String)>(conn)
        .await
        .map(|users| {
            users
                .into_iter()
                .map(|(id, name, email)| {
                    (
                        id,
                        TeacherApplicationPlatformReviewUserOutput { id, name, email },
                    )
                })
                .collect()
        })
        .map_err(map_error)
}

async fn load_organizations(
    conn: &mut AsyncPgConnection,
    applications: &[TeacherApplication],
) -> Result<BTreeMap<i32, String>, TeacherApplicationPlatformReviewError> {
    let ids = applications
        .iter()
        .flat_map(|application| {
            [
                application.requested_organization_id,
                application.organization_sponsor_id,
            ]
        })
        .flatten()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if ids.is_empty() {
        return Ok(BTreeMap::new());
    }

    organizations::table
        .filter(organizations::id.eq_any(&ids))
        .select((organizations::id, organizations::name))
        .load::<(i32, String)>(conn)
        .await
        .map(|organizations| organizations.into_iter().collect())
        .map_err(map_error)
}

async fn load_courses(
    conn: &mut AsyncPgConnection,
    applications: &[TeacherApplication],
) -> Result<BTreeMap<i32, String>, TeacherApplicationPlatformReviewError> {
    let ids = applications
        .iter()
        .filter_map(|application| application.requested_course_id)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if ids.is_empty() {
        return Ok(BTreeMap::new());
    }

    courses::table
        .filter(courses::id.eq_any(&ids))
        .select((courses::id, courses::title))
        .load::<(i32, String)>(conn)
        .await
        .map(|courses| courses.into_iter().collect())
        .map_err(map_error)
}

async fn load_audits(
    conn: &mut AsyncPgConnection,
    applications: &[TeacherApplication],
) -> Result<
    BTreeMap<i64, TeacherApplicationPlatformReviewAuditSummaryOutput>,
    TeacherApplicationPlatformReviewError,
> {
    let ids = applications
        .iter()
        .map(|application| application.id)
        .collect::<Vec<_>>();
    if ids.is_empty() {
        return Ok(BTreeMap::new());
    }

    teacher_application_audit_events::table
        .filter(teacher_application_audit_events::application_id.eq_any(&ids))
        .order(teacher_application_audit_events::created_at.asc())
        .then_order_by(teacher_application_audit_events::id.asc())
        .load::<TeacherApplicationAuditEvent>(conn)
        .await
        .map_err(map_error)
        .and_then(audit_summaries)
}

fn map_error(error: diesel::result::Error) -> TeacherApplicationPlatformReviewError {
    TeacherApplicationPlatformReviewError::Database(error.to_string())
}
