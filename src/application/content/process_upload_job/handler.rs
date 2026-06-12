use crate::application::content::ports::ContentProcessingJobStore;
use crate::application::content::process_upload_job::{
    ProcessUploadJobCommand, ProcessUploadJobError, ProcessUploadJobOutput,
};
use crate::domain::content::content_item::is_video_content_type;

const COURSE_MATERIALS_BUCKET: &str = "course-materials";

pub async fn process_upload_job(
    store: &mut impl ContentProcessingJobStore,
    command: ProcessUploadJobCommand,
) -> Result<ProcessUploadJobOutput, ProcessUploadJobError> {
    store
        .ensure_chapter_belongs_to_course(command.course_id, command.chapter_id)
        .await?;

    let content = store
        .content_for_processing(command.chapter_id, command.content_id)
        .await?;

    if !is_video_content_type(&content.content_type) {
        return Err(ProcessUploadJobError::NonVideoContent);
    }

    let object_key = content
        .object_key
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .ok_or(ProcessUploadJobError::MissingObjectKey)?;

    let expected_prefix = format!(
        "courses/{}/chapters/{}/",
        command.course_id, command.chapter_id
    );
    if !object_key.starts_with(&expected_prefix) {
        return Err(ProcessUploadJobError::InvalidObjectKey);
    }

    store
        .enqueue_processing_job(COURSE_MATERIALS_BUCKET, object_key.clone(), command.user_id)
        .await
        .map_err(|error| match error {
            ProcessUploadJobError::JobQueueFailed { message, .. } => {
                ProcessUploadJobError::JobQueueFailed {
                    object_key: object_key.clone(),
                    message,
                }
            }
            other => other,
        })?;

    Ok(ProcessUploadJobOutput { object_key })
}
