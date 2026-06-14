use serde::{Deserialize, Serialize};

use crate::application::content::manage_chapter::{
    ChapterOutput, CreateChapterCommand, UpdateChapterCommand,
};

#[derive(Debug, Clone, Serialize)]
pub struct ChapterResponse {
    pub id: i32,
    pub course_id: i32,
    pub title: String,
    pub order: i32,
}

impl From<ChapterOutput> for ChapterResponse {
    fn from(chapter: ChapterOutput) -> Self {
        Self {
            id: chapter.id,
            course_id: chapter.course_id,
            title: chapter.title,
            order: chapter.order,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateChapterRequest {
    pub title: String,
    pub order: i32,
}

impl CreateChapterRequest {
    pub fn into_command(self, course_id: i32) -> CreateChapterCommand {
        CreateChapterCommand {
            course_id,
            title: self.title,
            order: self.order,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateChapterRequest {
    pub title: Option<String>,
    pub order: Option<i32>,
}

impl From<UpdateChapterRequest> for UpdateChapterCommand {
    fn from(request: UpdateChapterRequest) -> Self {
        Self {
            title: request.title,
            order: request.order,
        }
    }
}
