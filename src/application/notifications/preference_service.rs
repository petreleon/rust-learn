use futures::future::BoxFuture;

use crate::application::notifications::preferences::{
    NotificationPreferenceOutput, NotificationPreferencesError,
};
use crate::application::notifications::save_preferences::SaveNotificationPreferencesCommand;

pub trait NotificationPreferencesUseCase: Send + Sync {
    fn get_preferences(
        &self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<NotificationPreferenceOutput, NotificationPreferencesError>>;

    fn save_preferences(
        &self,
        command: SaveNotificationPreferencesCommand,
    ) -> BoxFuture<'_, Result<NotificationPreferenceOutput, NotificationPreferencesError>>;
}
