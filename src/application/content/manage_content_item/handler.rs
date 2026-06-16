use crate::application::content::manage_content_item::{
    ContentItemError, ContentItemOutput, CreateContentItemCommand, CreateContentItemOutput,
    UpdateContentItemCommand,
};
use crate::application::content::ports::ContentItemStore;
use crate::domain::content::content_item::ContentPublicationStatus;

pub async fn list_content_items(
    store: &mut impl ContentItemStore,
    course_id: i32,
    chapter_id: i32,
) -> Result<Vec<ContentItemOutput>, ContentItemError> {
    store.list_by_chapter(course_id, chapter_id).await
}

pub async fn create_content_item(
    store: &mut impl ContentItemStore,
    course_id: i32,
    command: CreateContentItemCommand,
) -> Result<CreateContentItemOutput, ContentItemError> {
    let content = store.create(course_id, command).await?;
    let (notification_recipient_ids, notification_recipient_lookup_error) =
        match store.list_course_content_recipients(course_id).await {
            Ok(recipients) => (recipients, None),
            Err(ContentItemError::Database(message)) => (Vec::new(), Some(message)),
            Err(error) => (Vec::new(), Some(format!("{:?}", error))),
        };

    Ok(CreateContentItemOutput {
        content,
        notification_recipient_ids,
        notification_recipient_lookup_error,
    })
}

pub async fn update_content_item(
    store: &mut impl ContentItemStore,
    course_id: i32,
    chapter_id: i32,
    content_id: i32,
    command: UpdateContentItemCommand,
) -> Result<ContentItemOutput, ContentItemError> {
    let command = normalize_update_command(command)?;
    store
        .update(course_id, chapter_id, content_id, command)
        .await
}

pub async fn delete_content_item(
    store: &mut impl ContentItemStore,
    course_id: i32,
    chapter_id: i32,
    content_id: i32,
) -> Result<bool, ContentItemError> {
    store.delete(course_id, chapter_id, content_id).await
}

fn normalize_update_command(
    command: UpdateContentItemCommand,
) -> Result<UpdateContentItemCommand, ContentItemError> {
    let publication_status = command
        .publication_status
        .as_deref()
        .map(ContentPublicationStatus::normalize)
        .transpose()
        .map_err(|error| ContentItemError::InvalidPublicationStatus(error.value))?
        .map(|status| status.as_str().to_string());

    Ok(UpdateContentItemCommand {
        publication_status,
        ..command
    })
}
