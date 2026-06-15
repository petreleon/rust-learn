use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::get_teacher_course_students::TeacherStudentProgressSummaryOutput;
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::infra::postgres::schema::{chapters, contents, course_progress};

pub async fn load_course_content_count(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<usize, TeacherCourseDashboardError> {
    let count = contents::table
        .inner_join(chapters::table.on(contents::chapter_id.eq(chapters::id)))
        .filter(chapters::course_id.eq(course_id))
        .count()
        .get_result::<i64>(conn)
        .await
        .map_err(map_dashboard_error)?;

    Ok(count as usize)
}

pub async fn load_teacher_student_lesson_progress(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
    total_content_count: usize,
) -> Result<TeacherStudentProgressSummaryOutput, TeacherCourseDashboardError> {
    let row = course_progress::table
        .inner_join(contents::table.on(course_progress::content_id.eq(contents::id)))
        .inner_join(chapters::table.on(contents::chapter_id.eq(chapters::id)))
        .filter(course_progress::course_id.eq(course_id))
        .filter(course_progress::user_id.eq(student_user_id))
        .select((
            course_progress::content_id,
            contents::content_type,
            chapters::order,
            contents::order,
            course_progress::viewed_at,
        ))
        .first::<(i32, String, i32, i32, DateTime<Utc>)>(conn)
        .await
        .optional()
        .map_err(map_dashboard_error)?;

    let Some((content_id, content_type, chapter_order, content_order, viewed_at)) = row else {
        return Ok(TeacherStudentProgressSummaryOutput {
            supported: true,
            completed_content_count: Some(0),
            total_content_count,
            completion_percentage: Some(0.0),
            current_content_id: None,
            current_content_label: None,
            last_activity_at: None,
            note: "No lesson progress has been saved yet.".to_string(),
        });
    };

    let completed = if total_content_count == 0 { 0 } else { 1 };
    let percentage = if total_content_count == 0 {
        0.0
    } else {
        (completed as f64 / total_content_count as f64 * 100.0).round()
    };

    Ok(TeacherStudentProgressSummaryOutput {
        supported: true,
        completed_content_count: Some(completed),
        total_content_count,
        completion_percentage: Some(percentage),
        current_content_id: Some(content_id),
        current_content_label: Some(format!(
            "Module {}, lesson {}: {}",
            chapter_order + 1,
            content_order + 1,
            content_type
        )),
        last_activity_at: Some(viewed_at),
        note: "Latest viewed lesson is saved from the learner course route.".to_string(),
    })
}

fn map_dashboard_error(error: diesel::result::Error) -> TeacherCourseDashboardError {
    match error {
        diesel::result::Error::NotFound => TeacherCourseDashboardError::NotFound,
        other => TeacherCourseDashboardError::Database(other.to_string()),
    }
}
