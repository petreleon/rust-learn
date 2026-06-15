use std::collections::BTreeSet;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::application::learning::teacher_course_enrollment::{
    TeacherCourseRosterLearnerOutput, TeacherCourseRosterPageOutput,
    TeacherEnrollmentUserSummaryOutput,
};
use crate::infra::postgres::learning::{
    teacher_course_reward_eligibility_queries, teacher_course_workspace_queries,
};
use crate::infra::postgres::schema::{course_join_requests, course_roles, user_role_course, users};

pub async fn load_teacher_course_roster_page(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    can_manage_enrollments: bool,
) -> Result<TeacherCourseRosterPageOutput, TeacherCourseDashboardError> {
    let rows = load_student_rows(conn, course_id).await?;
    let mut learners = Vec::with_capacity(rows.len());
    let mut seen_user_ids = BTreeSet::new();
    let course_eligibility =
        teacher_course_reward_eligibility_queries::load_teacher_course_reward_eligibility_summary(
            conn, course_id,
        )
        .await?;

    for (id, name, email, email_verified, kyc_verified) in rows {
        if !seen_user_ids.insert(id) {
            continue;
        }
        let reward_eligibility =
            teacher_course_reward_eligibility_queries::load_teacher_student_reward_eligibility(
                conn,
                course_id,
                id,
                &course_eligibility,
            )
            .await?;

        learners.push(TeacherCourseRosterLearnerOutput {
            user: TeacherEnrollmentUserSummaryOutput {
                id,
                name,
                email,
                email_verified,
                kyc_verified,
            },
            roles: teacher_course_workspace_queries::load_actor_course_roles(conn, id, course_id)
                .await?,
            latest_join_request_status: load_latest_join_request_status(conn, course_id, id)
                .await?,
            access_state: "enrolled".to_string(),
            can_remove: can_manage_enrollments,
            progress_supported: true,
            reward_eligibility_supported: reward_eligibility.supported,
            reward_eligibility,
        });
    }

    let total = learners.len() as i64;
    Ok(TeacherCourseRosterPageOutput { learners, total })
}

pub async fn load_teacher_enrollment_user_summary(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<Option<TeacherEnrollmentUserSummaryOutput>, TeacherCourseDashboardError> {
    users::table
        .find(user_id)
        .select((
            users::id,
            users::name,
            users::email,
            users::email_verified,
            users::kyc_verified,
        ))
        .first::<(i32, String, String, bool, bool)>(conn)
        .await
        .optional()
        .map(|row| {
            row.map(|(id, name, email, email_verified, kyc_verified)| {
                TeacherEnrollmentUserSummaryOutput {
                    id,
                    name,
                    email,
                    email_verified,
                    kyc_verified,
                }
            })
        })
        .map_err(map_dashboard_error)
}

async fn load_student_rows(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<(i32, String, String, bool, bool)>, TeacherCourseDashboardError> {
    user_role_course::table
        .inner_join(
            course_roles::table
                .on(user_role_course::course_role_id.eq(course_roles::id.nullable())),
        )
        .inner_join(users::table.on(user_role_course::user_id.eq(users::id.nullable())))
        .filter(user_role_course::course_id.eq(course_id))
        .filter(course_roles::name.eq("STUDENT"))
        .order(users::name.asc())
        .then_order_by(users::id.asc())
        .select((
            users::id,
            users::name,
            users::email,
            users::email_verified,
            users::kyc_verified,
        ))
        .load::<(i32, String, String, bool, bool)>(conn)
        .await
        .map_err(map_dashboard_error)
}

async fn load_latest_join_request_status(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    user_id: i32,
) -> Result<Option<String>, TeacherCourseDashboardError> {
    course_join_requests::table
        .filter(course_join_requests::course_id.eq(course_id))
        .filter(course_join_requests::requester_user_id.eq(user_id))
        .order(course_join_requests::updated_at.desc())
        .select(course_join_requests::status)
        .first::<String>(conn)
        .await
        .optional()
        .map_err(map_dashboard_error)
}

fn map_dashboard_error(error: diesel::result::Error) -> TeacherCourseDashboardError {
    match error {
        diesel::result::Error::NotFound => TeacherCourseDashboardError::NotFound,
        other => TeacherCourseDashboardError::Database(other.to_string()),
    }
}
