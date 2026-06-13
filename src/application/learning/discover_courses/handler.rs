use crate::application::learning::discover_courses::{
    CourseDiscoveryError, CourseDiscoveryOutput, CourseDiscoveryQuery, CourseDiscoveryStore,
};

pub async fn discover_courses(
    store: &mut impl CourseDiscoveryStore,
    query: CourseDiscoveryQuery,
) -> Result<CourseDiscoveryOutput, CourseDiscoveryError> {
    store.discover(query).await
}
