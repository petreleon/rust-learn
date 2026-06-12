use futures::future::BoxFuture;

use crate::application::content::request_upload_url::{
    ContentUploadUrlError, RequestUploadUrlCommand, UploadUrlOutput,
};

pub trait ContentUploadUrlUseCase: Send + Sync {
    fn request_upload_url(
        &self,
        command: RequestUploadUrlCommand,
    ) -> BoxFuture<'_, Result<UploadUrlOutput, ContentUploadUrlError>>;
}
