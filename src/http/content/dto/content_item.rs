use serde::{Deserialize, Serialize};

use crate::application::content::manage_content_item::{
    ContentItemOutput, CreateContentItemCommand, UpdateContentItemCommand,
};

#[derive(Debug, Clone, Serialize)]
pub struct ContentItemResponse {
    pub id: i32,
    pub chapter_id: i32,
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
    pub publication_status: String,
}

impl From<ContentItemOutput> for ContentItemResponse {
    fn from(content: ContentItemOutput) -> Self {
        Self {
            id: content.id,
            chapter_id: content.chapter_id,
            order: content.order,
            content_type: content.content_type,
            data: content.data,
            publication_status: content.publication_status,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateContentItemRequest {
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
}

impl CreateContentItemRequest {
    pub fn into_command(self, chapter_id: i32) -> CreateContentItemCommand {
        CreateContentItemCommand {
            chapter_id,
            order: self.order,
            content_type: self.content_type,
            data: self.data,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateContentItemRequest {
    pub order: Option<i32>,
    pub content_type: Option<String>,
    pub data: Option<String>,
    pub publication_status: Option<String>,
}

impl From<UpdateContentItemRequest> for UpdateContentItemCommand {
    fn from(request: UpdateContentItemRequest) -> Self {
        Self {
            order: request.order,
            content_type: request.content_type,
            data: request.data,
            publication_status: request.publication_status,
        }
    }
}
