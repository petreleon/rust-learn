use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::learner_progress::ProgressCourse;
use crate::db::schema::{
    chapters, contents, course_join_requests, course_progress, course_roles, courses,
    courses_organizations, user_role_course,
};
use crate::infra::postgres::models::course_progress::{CourseProgress, NewCourseProgress};

pub async fn load_course(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> diesel::QueryResult<ProgressCourse> {
    courses::table
        .find(course_id)
        .select((courses::id, courses::lifecycle_status))
        .first::<(i32, String)>(conn)
        .await
        .map(|(id, lifecycle_status)| ProgressCourse {
            id,
            lifecycle_status,
        })
}

pub async fn course_organization_ids(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> diesel::QueryResult<Vec<i32>> {
    courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await
}

pub async fn actor_course_roles(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> diesel::QueryResult<Vec<String>> {
    let mut roles = user_role_course::table
        .inner_join(
            course_roles::table
                .on(user_role_course::course_role_id.eq(course_roles::id.nullable())),
        )
        .filter(user_role_course::user_id.eq(actor_user_id))
        .filter(user_role_course::course_id.eq(course_id))
        .order(course_roles::name.asc())
        .select(course_roles::name)
        .load::<String>(conn)
        .await?;
    roles.dedup();
    Ok(roles)
}

pub async fn has_join_request_status(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    status: String,
) -> diesel::QueryResult<bool> {
    course_join_requests::table
        .filter(course_join_requests::course_id.eq(course_id))
        .filter(course_join_requests::requester_user_id.eq(actor_user_id))
        .filter(course_join_requests::status.eq(status))
        .select(course_join_requests::id)
        .first::<i64>(conn)
        .await
        .optional()
        .map(|request_id| request_id.is_some())
}

pub async fn content_belongs_to_course(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    content_id: i32,
) -> diesel::QueryResult<bool> {
    contents::table
        .inner_join(chapters::table.on(contents::chapter_id.eq(chapters::id)))
        .filter(contents::id.eq(content_id))
        .select(chapters::course_id)
        .first::<i32>(conn)
        .await
        .optional()
        .map(|content_course_id| content_course_id == Some(course_id))
}

pub async fn save_progress(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    content_id: i32,
) -> diesel::QueryResult<CourseProgress> {
    diesel::insert_into(course_progress::table)
        .values(NewCourseProgress {
            user_id: actor_user_id,
            course_id,
            content_id,
        })
        .on_conflict((course_progress::user_id, course_progress::course_id))
        .do_update()
        .set((
            course_progress::content_id.eq(content_id),
            course_progress::viewed_at.eq(diesel::dsl::now),
        ))
        .get_result::<CourseProgress>(conn)
        .await
}

pub async fn get_progress(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> diesel::QueryResult<Option<CourseProgress>> {
    course_progress::table
        .filter(course_progress::user_id.eq(actor_user_id))
        .filter(course_progress::course_id.eq(course_id))
        .first::<CourseProgress>(conn)
        .await
        .optional()
}
