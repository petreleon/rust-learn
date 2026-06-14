use serde::Serialize;

use crate::application::content::request_media_url::MediaUrlOutput;

#[derive(Debug, Clone, Serialize)]
pub struct MediaUrlResponse {
    pub url: String,
}

impl From<MediaUrlOutput> for MediaUrlResponse {
    fn from(output: MediaUrlOutput) -> Self {
        Self { url: output.url }
    }
}
