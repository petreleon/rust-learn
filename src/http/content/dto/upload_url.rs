use serde::{Deserialize, Serialize};

use crate::application::content::request_upload_url::{RequestUploadUrlCommand, UploadUrlOutput};

#[derive(Debug, Clone, Deserialize)]
pub struct RequestUploadUrlRequest {
    pub filename: String,
    pub content_type: String,
}

impl RequestUploadUrlRequest {
    pub fn into_command(self, course_id: i32, chapter_id: i32) -> RequestUploadUrlCommand {
        RequestUploadUrlCommand {
            course_id,
            chapter_id,
            filename: self.filename,
            content_type: self.content_type,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UploadUrlResponse {
    pub upload_url: String,
    pub object_key: String,
}

impl From<UploadUrlOutput> for UploadUrlResponse {
    fn from(output: UploadUrlOutput) -> Self {
        Self {
            upload_url: output.upload_url,
            object_key: output.object_key,
        }
    }
}
