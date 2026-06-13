use crate::application::notifications::notification_inbox::{
    NotificationInboxError, NotificationOutput,
};
use crate::application::notifications::ports::NotificationInboxStore;

pub async fn list_notifications(
    store: &mut impl NotificationInboxStore,
    user_id: i32,
) -> Result<Vec<NotificationOutput>, NotificationInboxError> {
    store.list(user_id).await
}

pub async fn mark_notification_read(
    store: &mut impl NotificationInboxStore,
    user_id: i32,
    notification_id: i64,
) -> Result<(), NotificationInboxError> {
    store.mark_read(user_id, notification_id).await
}

pub async fn clear_notifications(
    store: &mut impl NotificationInboxStore,
    user_id: i32,
) -> Result<(), NotificationInboxError> {
    store.clear(user_id).await
}

#[cfg(test)]
mod tests {
    use super::{clear_notifications, list_notifications, mark_notification_read};
    use crate::application::notifications::notification_inbox::{
        NotificationInboxError, NotificationOutput,
    };
    use crate::application::notifications::ports::NotificationInboxStore;
    use chrono::Utc;
    use futures::future::{ready, BoxFuture, FutureExt};

    #[derive(Debug, PartialEq, Eq)]
    enum InboxCall {
        List { user_id: i32 },
        MarkRead { user_id: i32, notification_id: i64 },
        Clear { user_id: i32 },
    }

    struct RecordingInboxStore {
        calls: Vec<InboxCall>,
        list_result: Vec<NotificationOutput>,
    }

    impl RecordingInboxStore {
        fn new(list_result: Vec<NotificationOutput>) -> Self {
            Self {
                calls: Vec::new(),
                list_result,
            }
        }
    }

    impl NotificationInboxStore for RecordingInboxStore {
        fn list(
            &mut self,
            user_id: i32,
        ) -> BoxFuture<'_, Result<Vec<NotificationOutput>, NotificationInboxError>> {
            self.calls.push(InboxCall::List { user_id });
            ready(Ok(self.list_result.clone())).boxed()
        }

        fn mark_read(
            &mut self,
            user_id: i32,
            notification_id: i64,
        ) -> BoxFuture<'_, Result<(), NotificationInboxError>> {
            self.calls.push(InboxCall::MarkRead {
                user_id,
                notification_id,
            });
            ready(Ok(())).boxed()
        }

        fn clear(&mut self, user_id: i32) -> BoxFuture<'_, Result<(), NotificationInboxError>> {
            self.calls.push(InboxCall::Clear { user_id });
            ready(Ok(())).boxed()
        }
    }

    #[test]
    fn list_delegates_to_store_for_actor() {
        futures::executor::block_on(async {
            let expected = NotificationOutput {
                id: 11,
                user_id: Some(7),
                title: "welcome".to_string(),
                body: "Hello".to_string(),
                created_at: Utc::now(),
                read: false,
            };
            let mut store = RecordingInboxStore::new(vec![expected.clone()]);

            let rows = list_notifications(&mut store, 7).await.unwrap();

            assert_eq!(rows, vec![expected]);
            assert_eq!(store.calls, vec![InboxCall::List { user_id: 7 }]);
        });
    }

    #[test]
    fn read_mutations_delegate_to_store_for_actor() {
        futures::executor::block_on(async {
            let mut store = RecordingInboxStore::new(Vec::new());

            mark_notification_read(&mut store, 7, 11).await.unwrap();
            clear_notifications(&mut store, 7).await.unwrap();

            assert_eq!(
                store.calls,
                vec![
                    InboxCall::MarkRead {
                        user_id: 7,
                        notification_id: 11,
                    },
                    InboxCall::Clear { user_id: 7 },
                ]
            );
        });
    }
}
