use futures::future::BoxFuture;

use crate::application::content::request_media_url::{
    ContentMediaUrlError, MediaUrlOutput, RequestMediaUrlCommand,
};

pub trait ContentMediaUrlUseCase: Send + Sync {
    fn request_media_url(
        &self,
        command: RequestMediaUrlCommand,
    ) -> BoxFuture<'_, Result<MediaUrlOutput, ContentMediaUrlError>>;
}
