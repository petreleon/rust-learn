use futures::future::BoxFuture;

use crate::application::learning::discover_courses::{
    CourseDiscoveryError, CourseDiscoveryOutput, CourseDiscoveryQuery,
};

pub trait CourseDiscoveryStore {
    fn discover(
        &mut self,
        query: CourseDiscoveryQuery,
    ) -> BoxFuture<'_, Result<CourseDiscoveryOutput, CourseDiscoveryError>>;
}
