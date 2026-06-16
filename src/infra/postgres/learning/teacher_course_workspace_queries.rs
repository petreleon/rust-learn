use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::get_teacher_course_workspace::{
    TeacherCourseWorkspaceChapterOutput, TeacherCourseWorkspaceContentOutput,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::infra::postgres::learning::content_processing_queries;
use crate::infra::postgres::schema::{chapters, contents, course_roles, user_role_course};

pub async fn load_teacher_course_workspace_chapters(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    _course_lifecycle_status: &str,
) -> Result<Vec<TeacherCourseWorkspaceChapterOutput>, TeacherCourseDashboardError> {
    let rows = chapters::table
        .filter(chapters::course_id.eq(course_id))
        .order(chapters::order.asc())
        .then_order_by(chapters::id.asc())
        .select((chapters::id, chapters::title, chapters::order))
        .load::<(i32, String, i32)>(conn)
        .await
        .map_err(map_dashboard_error)?;

    let mut chapters = Vec::with_capacity(rows.len());
    for (id, title, order) in rows {
        chapters.push(TeacherCourseWorkspaceChapterOutput {
            id,
            title,
            order,
            contents: load_workspace_contents(conn, id).await?,
        });
    }

    Ok(chapters)
}

pub async fn load_actor_course_roles(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<Vec<String>, TeacherCourseDashboardError> {
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
        .await
        .map_err(map_dashboard_error)?;

    roles.dedup();
    Ok(roles)
}

async fn load_workspace_contents(
    conn: &mut AsyncPgConnection,
    chapter_id: i32,
) -> Result<Vec<TeacherCourseWorkspaceContentOutput>, TeacherCourseDashboardError> {
    let rows = contents::table
        .filter(contents::chapter_id.eq(chapter_id))
        .order(contents::order.asc())
        .then_order_by(contents::id.asc())
        .select((
            contents::id,
            contents::order,
            contents::content_type,
            contents::data,
            contents::publication_status,
        ))
        .load::<(i32, i32, String, Option<String>, String)>(conn)
        .await
        .map_err(map_dashboard_error)?;

    let mut contents = Vec::with_capacity(rows.len());
    for (id, order, content_type, data, publication_status) in rows {
        let processing =
            content_processing_queries::load_latest_content_processing(conn, data.as_deref())
                .await
                .map_err(map_dashboard_error)?;
        contents.push(TeacherCourseWorkspaceContentOutput {
            id,
            order,
            content_type: content_type.clone(),
            data: data.clone(),
            data_present: data
                .as_deref()
                .map(str::trim)
                .is_some_and(|value| !value.is_empty()),
            publication_status,
            display_state: content_processing_queries::content_display_state(
                &content_type,
                data.as_deref(),
                &processing,
            ),
            processing_status: processing.as_ref().map(|(status, _)| status.clone()),
            processing_error: processing.and_then(|(_, error)| error),
        });
    }

    Ok(contents)
}

fn map_dashboard_error(error: diesel::result::Error) -> TeacherCourseDashboardError {
    match error {
        diesel::result::Error::NotFound => TeacherCourseDashboardError::NotFound,
        other => TeacherCourseDashboardError::Database(other.to_string()),
    }
}
