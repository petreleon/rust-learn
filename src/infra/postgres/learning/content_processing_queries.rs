use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::infra::postgres::schema::upload_jobs;

pub async fn load_latest_content_processing(
    conn: &mut AsyncPgConnection,
    object_key: Option<&str>,
) -> diesel::QueryResult<Option<(String, Option<String>)>> {
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
}

pub fn content_display_state(
    content_type: &str,
    data: Option<&str>,
    processing: &Option<(String, Option<String>)>,
) -> String {
    let normalized_type = content_type.trim().to_ascii_lowercase();
    let has_data = data.map(str::trim).is_some_and(|value| !value.is_empty());

    if !has_data {
        return if is_media_content_type(&normalized_type) {
            "unprocessed_upload".to_string()
        } else {
            "unavailable".to_string()
        };
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

fn is_media_content_type(content_type: &str) -> bool {
    content_type == "video"
        || content_type.starts_with("video/")
        || content_type == "document"
        || content_type == "pdf"
        || content_type.starts_with("application/pdf")
}
