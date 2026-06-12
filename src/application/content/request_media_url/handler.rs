use crate::application::content::ports::{ContentMediaStore, ContentMediaUrlProvider};
use crate::application::content::request_media_url::{
    ContentMediaUrlError, MediaUrlOutput, RequestMediaUrlCommand,
};

const COURSE_MATERIALS_BUCKET: &str = "course-materials";
const MEDIA_URL_EXPIRES_SECONDS: u64 = 3600;

pub async fn request_media_url(
    store: &mut impl ContentMediaStore,
    media_url_provider: &mut impl ContentMediaUrlProvider,
    command: RequestMediaUrlCommand,
) -> Result<MediaUrlOutput, ContentMediaUrlError> {
    store
        .ensure_chapter_belongs_to_course(command.course_id, command.chapter_id)
        .await?;

    let object_key = store
        .content_object_key(command.chapter_id, command.content_id)
        .await?
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .ok_or(ContentMediaUrlError::MissingObjectKey)?;

    let expected_prefix = format!(
        "courses/{}/chapters/{}/",
        command.course_id, command.chapter_id
    );
    if !object_key.starts_with(&expected_prefix) {
        return Err(ContentMediaUrlError::InvalidObjectKey);
    }

    let url = media_url_provider
        .media_url(
            COURSE_MATERIALS_BUCKET,
            object_key.clone(),
            MEDIA_URL_EXPIRES_SECONDS,
        )
        .await
        .map_err(|error| match error {
            ContentMediaUrlError::PresignFailed { message, .. } => {
                ContentMediaUrlError::PresignFailed {
                    object_key: object_key.clone(),
                    message,
                }
            }
            other => other,
        })?;

    Ok(MediaUrlOutput { url })
}

#[cfg(test)]
mod tests {
    use futures::future::{BoxFuture, FutureExt};

    use super::*;

    #[derive(Default)]
    struct FakeMediaStore {
        chapter_result: Option<Result<(), ContentMediaUrlError>>,
        object_key_result: Option<Result<Option<String>, ContentMediaUrlError>>,
        content_lookup_count: usize,
    }

    impl ContentMediaStore for FakeMediaStore {
        fn ensure_chapter_belongs_to_course(
            &mut self,
            _course_id: i32,
            _chapter_id: i32,
        ) -> BoxFuture<'_, Result<(), ContentMediaUrlError>> {
            let result = self.chapter_result.take().unwrap_or(Ok(()));
            async move { result }.boxed()
        }

        fn content_object_key(
            &mut self,
            _chapter_id: i32,
            _content_id: i32,
        ) -> BoxFuture<'_, Result<Option<String>, ContentMediaUrlError>> {
            self.content_lookup_count += 1;
            let result = self
                .object_key_result
                .take()
                .unwrap_or(Ok(Some("courses/7/chapters/9/intro.mp4".to_string())));
            async move { result }.boxed()
        }
    }

    #[derive(Default)]
    struct FakeMediaUrlProvider {
        requested_object_key: Option<String>,
    }

    impl ContentMediaUrlProvider for FakeMediaUrlProvider {
        fn media_url(
            &mut self,
            _bucket: &'static str,
            object_key: String,
            _expires_seconds: u64,
        ) -> BoxFuture<'_, Result<String, ContentMediaUrlError>> {
            self.requested_object_key = Some(object_key);
            async move { Ok("https://media.example.test".to_string()) }.boxed()
        }
    }

    fn command() -> RequestMediaUrlCommand {
        RequestMediaUrlCommand {
            course_id: 7,
            chapter_id: 9,
            content_id: 11,
        }
    }

    #[tokio::test]
    async fn request_media_url_generates_url_for_course_chapter_object_key() {
        let mut store = FakeMediaStore {
            object_key_result: Some(Ok(Some(" courses/7/chapters/9/intro.mp4 ".to_string()))),
            ..FakeMediaStore::default()
        };
        let mut provider = FakeMediaUrlProvider::default();

        let output = request_media_url(&mut store, &mut provider, command())
            .await
            .expect("media URL should be generated");

        assert_eq!(output.url, "https://media.example.test");
        assert_eq!(
            provider.requested_object_key.as_deref(),
            Some("courses/7/chapters/9/intro.mp4")
        );
    }

    #[tokio::test]
    async fn request_media_url_validates_chapter_before_content_lookup() {
        let mut store = FakeMediaStore {
            chapter_result: Some(Err(ContentMediaUrlError::ChapterNotFound)),
            ..FakeMediaStore::default()
        };
        let mut provider = FakeMediaUrlProvider::default();

        let error = request_media_url(&mut store, &mut provider, command())
            .await
            .expect_err("chapter should fail before content lookup");

        assert_eq!(error, ContentMediaUrlError::ChapterNotFound);
        assert_eq!(store.content_lookup_count, 0);
        assert!(provider.requested_object_key.is_none());
    }

    #[tokio::test]
    async fn request_media_url_rejects_object_keys_outside_course_chapter() {
        let mut store = FakeMediaStore {
            object_key_result: Some(Ok(Some("courses/7/chapters/10/intro.mp4".to_string()))),
            ..FakeMediaStore::default()
        };
        let mut provider = FakeMediaUrlProvider::default();

        let error = request_media_url(&mut store, &mut provider, command())
            .await
            .expect_err("object key should be scoped to the requested chapter");

        assert_eq!(error, ContentMediaUrlError::InvalidObjectKey);
        assert!(provider.requested_object_key.is_none());
    }
}
