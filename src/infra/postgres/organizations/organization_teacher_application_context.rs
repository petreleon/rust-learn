use std::collections::{BTreeMap, BTreeSet};

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::organizations::list_organization_teacher_applications::{
    OrganizationTeacherApplicationAuditSummaryOutput, OrganizationTeacherApplicationListError,
    TeacherApplicationUserSummaryOutput,
};
use crate::infra::postgres::models::teacher_application::TeacherApplication;
use crate::infra::postgres::organizations::organization_teacher_application_audit_context::load_audits;
use crate::infra::postgres::schema::{courses, organizations, users};

pub struct OrganizationTeacherApplicationContext {
    pub users: BTreeMap<i32, TeacherApplicationUserSummaryOutput>,
    pub organizations: BTreeMap<i32, String>,
    pub courses: BTreeMap<i32, String>,
    pub audits: BTreeMap<i64, OrganizationTeacherApplicationAuditSummaryOutput>,
}

pub async fn build_context(
    conn: &mut AsyncPgConnection,
    applications: &[TeacherApplication],
) -> Result<OrganizationTeacherApplicationContext, OrganizationTeacherApplicationListError> {
    Ok(OrganizationTeacherApplicationContext {
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
    BTreeMap<i32, TeacherApplicationUserSummaryOutput>,
    OrganizationTeacherApplicationListError,
> {
    let user_ids = applications
        .iter()
        .flat_map(|application| [Some(application.applicant_user_id), application.reviewer_id])
        .flatten()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if user_ids.is_empty() {
        return Ok(BTreeMap::new());
    }

    users::table
        .filter(users::id.eq_any(&user_ids))
        .select((users::id, users::name, users::email))
        .load::<(i32, String, String)>(conn)
        .await
        .map_err(map_error)
        .map(|rows| {
            rows.into_iter()
                .map(|(id, name, email)| {
                    (id, TeacherApplicationUserSummaryOutput { id, name, email })
                })
                .collect()
        })
}

async fn load_organizations(
    conn: &mut AsyncPgConnection,
    applications: &[TeacherApplication],
) -> Result<BTreeMap<i32, String>, OrganizationTeacherApplicationListError> {
    let organization_ids = applications
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
    if organization_ids.is_empty() {
        return Ok(BTreeMap::new());
    }

    organizations::table
        .filter(organizations::id.eq_any(&organization_ids))
        .select((organizations::id, organizations::name))
        .load::<(i32, String)>(conn)
        .await
        .map_err(map_error)
        .map(|rows| rows.into_iter().collect())
}

async fn load_courses(
    conn: &mut AsyncPgConnection,
    applications: &[TeacherApplication],
) -> Result<BTreeMap<i32, String>, OrganizationTeacherApplicationListError> {
    let course_ids = applications
        .iter()
        .filter_map(|application| application.requested_course_id)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if course_ids.is_empty() {
        return Ok(BTreeMap::new());
    }

    courses::table
        .filter(courses::id.eq_any(&course_ids))
        .select((courses::id, courses::title))
        .load::<(i32, String)>(conn)
        .await
        .map_err(map_error)
        .map(|rows| rows.into_iter().collect())
}

fn map_error(error: diesel::result::Error) -> OrganizationTeacherApplicationListError {
    OrganizationTeacherApplicationListError::Database(error.to_string())
}
