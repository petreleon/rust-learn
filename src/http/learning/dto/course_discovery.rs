use serde::Serialize;

use crate::application::learning::discover_courses::{
    CourseDiscoveryCourseOutput, CourseDiscoveryOutput,
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CourseDiscoveryCourseResponse {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub description: Option<String>,
    pub topics: Option<String>,
    pub prerequisites: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CourseDiscoveryResponse {
    pub courses: Vec<CourseDiscoveryCourseResponse>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub organization_id: Option<i32>,
}

impl From<CourseDiscoveryOutput> for CourseDiscoveryResponse {
    fn from(output: CourseDiscoveryOutput) -> Self {
        Self {
            courses: output
                .courses
                .into_iter()
                .map(CourseDiscoveryCourseResponse::from)
                .collect(),
            total: output.total,
            limit: output.limit,
            offset: output.offset,
            search: output.search,
            organization_id: output.organization_id,
        }
    }
}

impl From<CourseDiscoveryCourseOutput> for CourseDiscoveryCourseResponse {
    fn from(course: CourseDiscoveryCourseOutput) -> Self {
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
