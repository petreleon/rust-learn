use futures::future::BoxFuture;

use crate::application::learning::discover_courses::{
    CourseDiscoveryError, CourseDiscoveryOutput, CourseDiscoveryQuery,
};

pub trait CourseDiscoveryUseCase: Send + Sync {
    fn discover_courses(
        &self,
        query: CourseDiscoveryQuery,
    ) -> BoxFuture<'_, Result<CourseDiscoveryOutput, CourseDiscoveryError>>;
}
