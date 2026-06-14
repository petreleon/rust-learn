use crate::application::notifications::ports::NotificationPreferenceStore;
use crate::application::notifications::preferences::{
    NotificationPreferenceOutput, NotificationPreferencesError,
};
use crate::application::notifications::save_preferences::SaveNotificationPreferencesCommand;

pub async fn save_preferences(
    store: &mut impl NotificationPreferenceStore,
    command: SaveNotificationPreferencesCommand,
) -> Result<NotificationPreferenceOutput, NotificationPreferencesError> {
    store.save(command).await
}
