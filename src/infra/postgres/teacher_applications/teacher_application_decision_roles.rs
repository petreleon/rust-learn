use diesel_async::AsyncPgConnection;

use crate::application::teacher_applications::TeacherApplicationOutput;
use crate::config::constants::roles::Roles;
use crate::domain::teacher_applications::scope::{
    TEACHER_APPLICATION_SCOPE_COURSE, TEACHER_APPLICATION_SCOPE_ORGANIZATION,
    TEACHER_APPLICATION_SCOPE_PLATFORM,
};
use crate::infra::postgres::access_control::course_role_records;
use crate::infra::postgres::access_control::organization_role_records;
use crate::infra::postgres::access_control::platform_role_records;
use crate::infra::postgres::access_control::role_catalog_store;

pub async fn assign_approved_teaching_bundle(
    conn: &mut AsyncPgConnection,
    application: &TeacherApplicationOutput,
) -> diesel::QueryResult<()> {
    match application.requested_scope.as_str() {
        TEACHER_APPLICATION_SCOPE_PLATFORM => {
            let role_id =
                role_catalog_store::platform_role_id_by_name(conn, &Roles::TEACHER.to_string())
                    .await?;
            platform_role_records::assign_platform_role_to_user_if_missing(
                conn,
                application.applicant_user_id,
                role_id,
            )
            .await
        }
        TEACHER_APPLICATION_SCOPE_ORGANIZATION => {
            let organization_id = application
                .requested_organization_id
                .or(application.organization_sponsor_id)
                .ok_or(diesel::result::Error::NotFound)?;
            let role_id =
                role_catalog_store::organization_role_id_by_name(conn, &Roles::TEACHER.to_string())
                    .await?;
            organization_role_records::assign_organization_role_to_user_if_missing(
                conn,
                application.applicant_user_id,
                organization_id,
                role_id,
            )
            .await
        }
        TEACHER_APPLICATION_SCOPE_COURSE => {
            let course_id = application
                .requested_course_id
                .ok_or(diesel::result::Error::NotFound)?;
            let role_id =
                role_catalog_store::course_role_id_by_name(conn, &Roles::TEACHER.to_string())
                    .await?;
            course_role_records::assign_course_role_to_user_if_missing(
                conn,
                application.applicant_user_id,
                course_id,
                role_id,
            )
            .await
        }
        _ => Err(diesel::result::Error::NotFound),
    }
}
