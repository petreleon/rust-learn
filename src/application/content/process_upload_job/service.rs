use futures::future::BoxFuture;

use crate::application::content::process_upload_job::{
    ProcessUploadJobCommand, ProcessUploadJobError, ProcessUploadJobOutput,
};

pub trait ContentProcessingUseCase: Send + Sync {
    fn process_upload_job(
        &self,
        command: ProcessUploadJobCommand,
    ) -> BoxFuture<'_, Result<ProcessUploadJobOutput, ProcessUploadJobError>>;
}
