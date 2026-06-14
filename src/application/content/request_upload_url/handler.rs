use crate::application::content::ports::{ContentUploadScopeStore, ContentUploadUrlProvider};
use crate::application::content::request_upload_url::{
    ContentUploadUrlError, RequestUploadUrlCommand, UploadUrlOutput,
};

const COURSE_MATERIALS_BUCKET: &str = "course-materials";
const UPLOAD_URL_EXPIRES_SECONDS: u64 = 3600;

pub async fn request_upload_url(
    scope_store: &mut impl ContentUploadScopeStore,
    upload_url_provider: &mut impl ContentUploadUrlProvider,
    command: RequestUploadUrlCommand,
) -> Result<UploadUrlOutput, ContentUploadUrlError> {
    scope_store
        .ensure_chapter_belongs_to_course(command.course_id, command.chapter_id)
        .await?;

    if command.content_type.trim().is_empty() {
        return Err(ContentUploadUrlError::MissingContentType);
    }

    let object_key = format!(
        "courses/{}/chapters/{}/{}",
        command.course_id, command.chapter_id, command.filename
    );
    let upload_url = upload_url_provider
        .prepare_upload_url(
            COURSE_MATERIALS_BUCKET,
            object_key.clone(),
            UPLOAD_URL_EXPIRES_SECONDS,
        )
        .await?;

    Ok(UploadUrlOutput {
        upload_url,
        object_key,
    })
}

#[cfg(test)]
mod tests {
    use futures::future::{BoxFuture, FutureExt};

    use super::*;

    #[derive(Default)]
    struct FakeScopeStore {
        result: Option<Result<(), ContentUploadUrlError>>,
    }

    impl ContentUploadScopeStore for FakeScopeStore {
        fn ensure_chapter_belongs_to_course(
            &mut self,
            _course_id: i32,
            _chapter_id: i32,
        ) -> BoxFuture<'_, Result<(), ContentUploadUrlError>> {
            let result = self.result.take().unwrap_or(Ok(()));
            async move { result }.boxed()
        }
    }

    #[derive(Default)]
    struct FakeUploadUrlProvider {
        requested_object_key: Option<String>,
    }

    impl ContentUploadUrlProvider for FakeUploadUrlProvider {
        fn prepare_upload_url(
            &mut self,
            _bucket: &'static str,
            object_key: String,
            _expires_seconds: u64,
        ) -> BoxFuture<'_, Result<String, ContentUploadUrlError>> {
            self.requested_object_key = Some(object_key);
            async move { Ok("https://upload.example.test".to_string()) }.boxed()
        }
    }

    #[tokio::test]
    async fn request_upload_url_builds_course_chapter_object_key() {
        let mut scope_store = FakeScopeStore::default();
        let mut upload_provider = FakeUploadUrlProvider::default();

        let output = request_upload_url(
            &mut scope_store,
            &mut upload_provider,
            RequestUploadUrlCommand {
                course_id: 7,
                chapter_id: 9,
                filename: "intro.mp4".to_string(),
                content_type: "video/mp4".to_string(),
            },
        )
        .await
        .expect("upload URL should be generated");

        assert_eq!(output.upload_url, "https://upload.example.test");
        assert_eq!(output.object_key, "courses/7/chapters/9/intro.mp4");
        assert_eq!(
            upload_provider.requested_object_key.as_deref(),
            Some("courses/7/chapters/9/intro.mp4")
        );
    }

    #[tokio::test]
    async fn request_upload_url_validates_chapter_before_content_type() {
        let mut scope_store = FakeScopeStore {
            result: Some(Err(ContentUploadUrlError::ChapterNotFound)),
        };
        let mut upload_provider = FakeUploadUrlProvider::default();

        let error = request_upload_url(
            &mut scope_store,
            &mut upload_provider,
            RequestUploadUrlCommand {
                course_id: 7,
                chapter_id: 9,
                filename: "intro.mp4".to_string(),
                content_type: " ".to_string(),
            },
        )
        .await
        .expect_err("chapter should fail before content type validation");

        assert_eq!(error, ContentUploadUrlError::ChapterNotFound);
        assert!(upload_provider.requested_object_key.is_none());
    }
}
