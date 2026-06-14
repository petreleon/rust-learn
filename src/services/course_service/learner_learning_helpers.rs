use crate::db::schema::{chapters, contents, upload_jobs};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use super::errors::LearnerCourseCatalogError;
use super::learner_course_types::{LearnerCourseLearningChapter, LearnerCourseLearningContent};

pub(super) async fn load_learner_course_learning_chapters(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<LearnerCourseLearningChapter>, LearnerCourseCatalogError> {
    let chapter_rows = chapters::table
        .filter(chapters::course_id.eq(course_id))
        .order(chapters::order.asc())
        .then_order_by(chapters::id.asc())
        .select((chapters::id, chapters::title, chapters::order))
        .load::<(i32, String, i32)>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    let mut result = Vec::with_capacity(chapter_rows.len());
    for (id, title, order) in chapter_rows {
        let content_rows = contents::table
            .filter(contents::chapter_id.eq(id))
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
            .map_err(LearnerCourseCatalogError::from)?;

        let mut learning_contents = Vec::with_capacity(content_rows.len());
        for (id, chapter_id, order, content_type, data) in content_rows {
            let processing = load_latest_content_processing(conn, data.as_deref()).await?;
            let display_state = content_display_state(&content_type, data.as_deref(), &processing);
            learning_contents.push(LearnerCourseLearningContent {
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

        result.push(LearnerCourseLearningChapter {
            id,
            title,
            order,
            contents: learning_contents,
        });
    }

    Ok(result)
}

pub(super) async fn load_latest_content_processing(
    conn: &mut AsyncPgConnection,
    object_key: Option<&str>,
) -> Result<Option<(String, Option<String>)>, LearnerCourseCatalogError> {
    let Some(object_key) = object_key.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(None);
    };

    upload_jobs::table
        .filter(upload_jobs::object.eq(object_key))
        .order(upload_jobs::created_at.desc())
        .select((upload_jobs::status, upload_jobs::last_error))
        .first::<(String, Option<String>)>(conn)
        .await
        .optional()
        .map_err(LearnerCourseCatalogError::from)
}

pub(super) fn content_display_state(
    content_type: &str,
    data: Option<&str>,
    processing: &Option<(String, Option<String>)>,
) -> String {
    let normalized_type = content_type.trim().to_ascii_lowercase();
    let has_data = data.map(str::trim).is_some_and(|value| !value.is_empty());

    if !has_data {
        if is_media_content_type(&normalized_type) {
            return "unprocessed_upload".to_string();
        }
        return "unavailable".to_string();
    }

    if let Some((status, _)) = processing {
        return match status.as_str() {
            "queued" | "processing" => "processing".to_string(),
            "failed" => "failed_processing".to_string(),
            "done" => "ready".to_string(),
            _ => "uploaded".to_string(),
        };
    }

    if is_media_content_type(&normalized_type) {
        return "uploaded".to_string();
    }

    "ready".to_string()
}

pub(super) fn is_media_content_type(content_type: &str) -> bool {
    content_type == "video"
        || content_type.starts_with("video/")
        || content_type == "document"
        || content_type == "pdf"
        || content_type.starts_with("application/pdf")
}
