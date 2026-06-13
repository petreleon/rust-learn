use serde::Serialize;

use crate::application::learning::get_course::CourseOutput;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CourseResponse {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub description: Option<String>,
    pub topics: Option<String>,
    pub prerequisites: Option<String>,
}

impl From<CourseOutput> for CourseResponse {
    fn from(course: CourseOutput) -> Self {
        Self {
            id: course.id,
            title: course.title,
            lifecycle_status: course.lifecycle_status,
            description: course.description,
            topics: course.topics,
            prerequisites: course.prerequisites,
        }
    }
}
