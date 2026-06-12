use futures::future::{BoxFuture, FutureExt};

use crate::application::content::ports::ContentProcessingJobStore;
use crate::application::content::process_upload_job::{
    process_upload_job, ProcessUploadJobCommand, ProcessUploadJobError, ProcessableContent,
};

#[derive(Default)]
struct FakeProcessingStore {
    chapter_result: Option<Result<(), ProcessUploadJobError>>,
    content_result: Option<Result<ProcessableContent, ProcessUploadJobError>>,
    queued_job: Option<(String, i32)>,
    content_lookup_count: usize,
}

impl ContentProcessingJobStore for FakeProcessingStore {
    fn ensure_chapter_belongs_to_course(
        &mut self,
        _course_id: i32,
        _chapter_id: i32,
    ) -> BoxFuture<'_, Result<(), ProcessUploadJobError>> {
        let result = self.chapter_result.take().unwrap_or(Ok(()));
        async move { result }.boxed()
    }

    fn content_for_processing(
        &mut self,
        _chapter_id: i32,
        _content_id: i32,
    ) -> BoxFuture<'_, Result<ProcessableContent, ProcessUploadJobError>> {
        self.content_lookup_count += 1;
        let result = self.content_result.take().unwrap_or_else(|| {
            Ok(ProcessableContent {
                content_type: "video".to_string(),
                object_key: Some("courses/7/chapters/9/intro.mp4".to_string()),
            })
        });
        async move { result }.boxed()
    }

    fn enqueue_processing_job(
        &mut self,
        _bucket: &'static str,
        object_key: String,
        user_id: i32,
    ) -> BoxFuture<'_, Result<(), ProcessUploadJobError>> {
        self.queued_job = Some((object_key, user_id));
        async move { Ok(()) }.boxed()
    }
}

fn command() -> ProcessUploadJobCommand {
    ProcessUploadJobCommand {
        course_id: 7,
        chapter_id: 9,
        content_id: 11,
        user_id: 13,
    }
}

#[tokio::test]
async fn process_upload_job_queues_video_object_for_user() {
    let mut store = FakeProcessingStore {
        content_result: Some(Ok(ProcessableContent {
            content_type: " video/mp4 ".to_string(),
            object_key: Some(" courses/7/chapters/9/intro.mp4 ".to_string()),
        })),
        ..FakeProcessingStore::default()
    };

    let output = process_upload_job(&mut store, command())
        .await
        .expect("processing job should be queued");

    assert_eq!(output.object_key, "courses/7/chapters/9/intro.mp4");
    assert_eq!(
        store.queued_job,
        Some(("courses/7/chapters/9/intro.mp4".to_string(), 13))
    );
}

#[tokio::test]
async fn process_upload_job_validates_chapter_before_content_lookup() {
    let mut store = FakeProcessingStore {
        chapter_result: Some(Err(ProcessUploadJobError::ChapterNotFound)),
        ..FakeProcessingStore::default()
    };

    let error = process_upload_job(&mut store, command())
        .await
        .expect_err("chapter should fail before content lookup");

    assert_eq!(error, ProcessUploadJobError::ChapterNotFound);
    assert_eq!(store.content_lookup_count, 0);
    assert!(store.queued_job.is_none());
}

#[tokio::test]
async fn process_upload_job_rejects_non_video_content() {
    let mut store = FakeProcessingStore {
        content_result: Some(Ok(ProcessableContent {
            content_type: "application/pdf".to_string(),
            object_key: Some("courses/7/chapters/9/notes.pdf".to_string()),
        })),
        ..FakeProcessingStore::default()
    };

    let error = process_upload_job(&mut store, command())
        .await
        .expect_err("non-video content should not be queued");

    assert_eq!(error, ProcessUploadJobError::NonVideoContent);
    assert!(store.queued_job.is_none());
}

#[tokio::test]
async fn process_upload_job_rejects_out_of_scope_object_key() {
    let mut store = FakeProcessingStore {
        content_result: Some(Ok(ProcessableContent {
            content_type: "video".to_string(),
            object_key: Some("courses/7/chapters/10/intro.mp4".to_string()),
        })),
        ..FakeProcessingStore::default()
    };

    let error = process_upload_job(&mut store, command())
        .await
        .expect_err("object key should be scoped to the requested chapter");

    assert_eq!(error, ProcessUploadJobError::InvalidObjectKey);
    assert!(store.queued_job.is_none());
}
