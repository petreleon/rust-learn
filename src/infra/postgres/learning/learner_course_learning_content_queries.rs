use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::get_learner_course_learning::{
    LearnerCourseLearningChapterOutput, LearnerCourseLearningContentOutput,
    LearnerCourseLearningError,
};
use crate::db::schema::{chapters, contents};
use crate::infra::postgres::learning::content_processing_queries;

pub async fn load_learner_course_learning_chapters(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<LearnerCourseLearningChapterOutput>, LearnerCourseLearningError> {
    let chapter_rows = chapters::table
        .filter(chapters::course_id.eq(course_id))
        .order(chapters::order.asc())
        .then_order_by(chapters::id.asc())
        .select((chapters::id, chapters::title, chapters::order))
        .load::<(i32, String, i32)>(conn)
        .await
        .map_err(map_learning_error)?;

    let mut result = Vec::with_capacity(chapter_rows.len());
    for (id, title, order) in chapter_rows {
        let contents = load_learning_contents(conn, id).await?;
        result.push(LearnerCourseLearningChapterOutput {
            id,
            title,
            order,
            contents,
        });
    }

    Ok(result)
}

async fn load_learning_contents(
    conn: &mut AsyncPgConnection,
    chapter_id: i32,
) -> Result<Vec<LearnerCourseLearningContentOutput>, LearnerCourseLearningError> {
    let rows = contents::table
        .filter(contents::chapter_id.eq(chapter_id))
        .order(contents::order.asc())
        .then_order_by(contents::id.asc())
        .select((
            contents::id,
            contents::chapter_id,
            contents::order,
            contents::content_type,
            contents::data,
        ))
        .load::<(i32, i32, i32, String, Option<String>)>(conn)
        .await
        .map_err(map_learning_error)?;

    let mut result = Vec::with_capacity(rows.len());
    for (id, chapter_id, order, content_type, data) in rows {
        let processing =
            content_processing_queries::load_latest_content_processing(conn, data.as_deref())
                .await
                .map_err(map_learning_error)?;
        let display_state = content_processing_queries::content_display_state(
            &content_type,
            data.as_deref(),
            &processing,
        );
        result.push(LearnerCourseLearningContentOutput {
            id,
            chapter_id,
            order,
            content_type,
            data,
            display_state,
            processing_status: processing.as_ref().map(|(status, _)| status.clone()),
            processing_error: processing.and_then(|(_, error)| error),
        });
    }

    Ok(result)
}

fn map_learning_error(error: diesel::result::Error) -> LearnerCourseLearningError {
    match error {
        diesel::result::Error::NotFound => LearnerCourseLearningError::NotFound,
        other => LearnerCourseLearningError::Database(other.to_string()),
    }
}
