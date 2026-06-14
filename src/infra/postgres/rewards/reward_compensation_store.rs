use diesel_async::{AsyncConnection, AsyncPgConnection};
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::record_compensation::{
    RewardCompensation, RewardCompensationError, RewardCompensationOutput, RewardCompensationStore,
};
use crate::infra::postgres::rewards::reward_authorization_access;

pub struct PostgresRewardCompensationStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresRewardCompensationStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl RewardCompensationStore for PostgresRewardCompensationStore<'_> {
    fn can_record_reward_compensation(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardCompensationError>> {
        async move {
            reward_authorization_access::can_record_reward_compensation(self.conn, actor_user_id)
                .await
                .map_err(|error| RewardCompensationError::Database(error.to_string()))
        }
        .boxed()
    }

    fn record_reward_compensation(
        &mut self,
        compensation: RewardCompensation,
    ) -> BoxFuture<'_, Result<RewardCompensationOutput, RewardCompensationError>> {
        async move {
            self.conn
                .transaction::<_, RewardCompensationTransactionError, _>(|conn| {
                    Box::pin(async move {
                        super::reward_compensation_transaction::record_reward_compensation(
                            conn,
                            compensation,
                        )
                        .await
                        .map_err(RewardCompensationTransactionError::Application)
                    })
                })
                .await
                .map_err(RewardCompensationError::from)
        }
        .boxed()
    }
}

enum RewardCompensationTransactionError {
    Application(RewardCompensationError),
    Diesel(diesel::result::Error),
}

impl From<diesel::result::Error> for RewardCompensationTransactionError {
    fn from(error: diesel::result::Error) -> Self {
        Self::Diesel(error)
    }
}

impl From<RewardCompensationTransactionError> for RewardCompensationError {
    fn from(error: RewardCompensationTransactionError) -> Self {
        match error {
            RewardCompensationTransactionError::Application(error) => error,
            RewardCompensationTransactionError::Diesel(error) => {
                super::reward_compensation_mappers::map_reward_compensation_error(error)
            }
        }
    }
}
