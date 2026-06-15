use crate::application::identity::current_session::{CurrentSessionError, CurrentSessionOutput};
use crate::application::identity::ports::CurrentSessionStore;

pub async fn get_current_session(
    store: &mut impl CurrentSessionStore,
    user_id: i32,
) -> Result<CurrentSessionOutput, CurrentSessionError> {
    store.load_current_session(user_id).await
}

#[cfg(test)]
mod tests {
    use super::get_current_session;
    use crate::application::identity::current_session::{
        CurrentSessionAccess, CurrentSessionError, CurrentSessionOutput, CurrentSessionUser,
        PlatformSessionScope,
    };
    use crate::application::identity::ports::CurrentSessionStore;
    use futures::future::{ready, BoxFuture, FutureExt};

    struct RecordingSessionStore {
        requested_user_id: Option<i32>,
        result: Result<CurrentSessionOutput, CurrentSessionError>,
    }

    impl RecordingSessionStore {
        fn new(result: Result<CurrentSessionOutput, CurrentSessionError>) -> Self {
            Self {
                requested_user_id: None,
                result,
            }
        }
    }

    impl CurrentSessionStore for RecordingSessionStore {
        fn load_current_session(
            &mut self,
            user_id: i32,
        ) -> BoxFuture<'_, Result<CurrentSessionOutput, CurrentSessionError>> {
            self.requested_user_id = Some(user_id);
            ready(self.result.clone()).boxed()
        }
    }

    #[test]
    fn delegates_to_store_for_actor() {
        futures::executor::block_on(async {
            let expected = CurrentSessionOutput {
                access: CurrentSessionAccess {
                    learner: true,
                    teacher: false,
                    teacher_application: false,
                    organization: false,
                    platform_admin: false,
                },
                user: CurrentSessionUser {
                    id: 7,
                    name: "Ada".to_string(),
                    email: "ada@example.com".to_string(),
                    email_verified: true,
                    kyc_verified: false,
                },
                platform: PlatformSessionScope {
                    roles: Vec::new(),
                    direct_permissions: Vec::new(),
                    delegated_permissions: Vec::new(),
                    effective_permissions: Vec::new(),
                    capabilities: Vec::new(),
                },
                organizations: Vec::new(),
                courses: Vec::new(),
                delegated_permissions: Vec::new(),
            };
            let mut store = RecordingSessionStore::new(Ok(expected.clone()));

            let session = get_current_session(&mut store, 7).await.unwrap();

            assert_eq!(session, expected);
            assert_eq!(store.requested_user_id, Some(7));
        });
    }

    #[test]
    fn returns_store_error() {
        futures::executor::block_on(async {
            let mut store = RecordingSessionStore::new(Err(CurrentSessionError::EmailUnverified));

            let error = get_current_session(&mut store, 7).await.unwrap_err();

            assert_eq!(error, CurrentSessionError::EmailUnverified);
        });
    }
}
