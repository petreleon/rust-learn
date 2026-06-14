use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::discover_courses::{
    discover_courses, CourseDiscoveryCourseOutput, CourseDiscoveryError, CourseDiscoveryOutput,
    CourseDiscoveryQuery, CourseDiscoveryStore,
};

struct FakeCourseDiscoveryStore {
    output: CourseDiscoveryOutput,
    requested_query: Option<CourseDiscoveryQuery>,
}

impl CourseDiscoveryStore for FakeCourseDiscoveryStore {
    fn discover(
        &mut self,
        query: CourseDiscoveryQuery,
    ) -> BoxFuture<'_, Result<CourseDiscoveryOutput, CourseDiscoveryError>> {
        self.requested_query = Some(query);
        let output = self.output.clone();
        async move { Ok(output) }.boxed()
    }
}

#[tokio::test]
async fn discover_courses_normalizes_query_and_uses_store_port() {
    let expected = CourseDiscoveryOutput {
        courses: vec![CourseDiscoveryCourseOutput {
            id: 17,
            title: "Rust patterns".to_string(),
            lifecycle_status: "draft".to_string(),
            description: None,
            topics: Some("ownership".to_string()),
            prerequisites: None,
        }],
        total: 1,
        limit: 100,
        offset: 0,
        search: Some("Rust".to_string()),
        organization_id: Some(9),
    };
    let mut store = FakeCourseDiscoveryStore {
        output: expected.clone(),
        requested_query: None,
    };

    let query = CourseDiscoveryQuery::new(Some(" Rust ".to_string()), Some(9), Some(500), Some(-3));
    let result = discover_courses(&mut store, query)
        .await
        .expect("courses should load");

    assert_eq!(result, expected);
    assert_eq!(
        store.requested_query,
        Some(CourseDiscoveryQuery {
            search: Some("Rust".to_string()),
            organization_id: Some(9),
            limit: 100,
            offset: 0,
        })
    );
}
