use futures::future::BoxFuture;

use crate::application::notifications::preferences::{
    NotificationPreferenceOutput, NotificationPreferencesError,
};
use crate::application::notifications::save_preferences::SaveNotificationPreferencesCommand;

pub trait NotificationPreferenceStore {
    fn find_by_user(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Option<NotificationPreferenceOutput>, NotificationPreferencesError>>;

    fn save(
        &mut self,
        command: SaveNotificationPreferencesCommand,
    ) -> BoxFuture<'_, Result<NotificationPreferenceOutput, NotificationPreferencesError>>;
}
