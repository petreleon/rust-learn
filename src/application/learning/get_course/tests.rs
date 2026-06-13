use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::get_course::{
    get_course, CourseOutput, CourseReadError, CourseReadStore,
};

struct FakeCourseReadStore {
    course: CourseOutput,
    requested_course_id: Option<i32>,
}

impl CourseReadStore for FakeCourseReadStore {
    fn get(&mut self, course_id: i32) -> BoxFuture<'_, Result<CourseOutput, CourseReadError>> {
        self.requested_course_id = Some(course_id);
        let course = self.course.clone();
        async move { Ok(course) }.boxed()
    }
}

#[tokio::test]
async fn get_course_uses_course_read_port() {
    let expected = CourseOutput {
        id: 17,
        title: "Rust patterns".to_string(),
        lifecycle_status: "draft".to_string(),
        description: Some("Modular Rust".to_string()),
        topics: None,
        prerequisites: None,
    };
    let mut store = FakeCourseReadStore {
        course: expected.clone(),
        requested_course_id: None,
    };

    let result = get_course(&mut store, 17)
        .await
        .expect("course should load");

    assert_eq!(result, expected);
    assert_eq!(store.requested_course_id, Some(17));
}
