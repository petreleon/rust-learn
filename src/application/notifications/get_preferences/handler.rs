use crate::application::notifications::ports::NotificationPreferenceStore;
use crate::application::notifications::preferences::{
    NotificationPreferenceOutput, NotificationPreferencesError,
};

pub async fn get_preferences(
    store: &mut impl NotificationPreferenceStore,
    user_id: i32,
) -> Result<NotificationPreferenceOutput, NotificationPreferencesError> {
    Ok(store
        .find_by_user(user_id)
        .await?
        .unwrap_or_else(|| NotificationPreferenceOutput::default_for_user(user_id)))
}

#[cfg(test)]
mod tests {
    use super::get_preferences;
    use crate::application::notifications::ports::NotificationPreferenceStore;
    use crate::application::notifications::preferences::{
        NotificationPreferenceOutput, NotificationPreferencesError,
    };
    use crate::application::notifications::save_preferences::SaveNotificationPreferencesCommand;
    use futures::future::{ready, BoxFuture, FutureExt};

    struct MissingPreferenceStore;

    impl NotificationPreferenceStore for MissingPreferenceStore {
        fn find_by_user(
            &mut self,
            _user_id: i32,
        ) -> BoxFuture<'_, Result<Option<NotificationPreferenceOutput>, NotificationPreferencesError>>
        {
            ready(Ok(None)).boxed()
        }

        fn save(
            &mut self,
            _command: SaveNotificationPreferencesCommand,
        ) -> BoxFuture<'_, Result<NotificationPreferenceOutput, NotificationPreferencesError>>
        {
            unreachable!("save should not be called by get_preferences")
        }
    }

    #[test]
    fn returns_default_preferences_when_user_has_no_row() {
        futures::executor::block_on(async {
            let mut store = MissingPreferenceStore;

            let preferences = get_preferences(&mut store, 7).await.unwrap();

            assert_eq!(preferences.user_id, 7);
            assert!(preferences.email_enabled);
            assert!(!preferences.push_enabled);
            assert_eq!(preferences.updated_at, None);
        });
    }
}
