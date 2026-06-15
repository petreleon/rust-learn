use diesel::{BoolExpressionMethods, PgTextExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::identity::list_users::ListUsersQuery;
use crate::application::identity::ports::UserProfileStore;
use crate::application::identity::user_profile::{UserProfileError, UserProfileOutput};
use crate::infra::postgres::access_control::permission_checks;
use crate::infra::postgres::models::user::User;
use crate::infra::postgres::schema::users;

pub struct PostgresUserProfileStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresUserProfileStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl UserProfileStore for PostgresUserProfileStore<'_> {
    fn list_users(
        &mut self,
        query: ListUsersQuery,
    ) -> BoxFuture<'_, Result<Vec<UserProfileOutput>, UserProfileError>> {
        async move {
            let result = if let Some(search) = query.search_term() {
                let pattern = format!("%{}%", search);
                users::table
                    .filter(users::name.ilike(&pattern).or(users::email.ilike(&pattern)))
                    .load::<User>(self.conn)
                    .await
            } else {
                users::table.load::<User>(self.conn).await
            };

            result
                .map(|users| users.into_iter().map(UserProfileOutput::from).collect())
                .map_err(map_user_profile_error)
        }
        .boxed()
    }

    fn find_user(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<UserProfileOutput, UserProfileError>> {
        async move {
            users::table
                .find(user_id)
                .first::<User>(self.conn)
                .await
                .map(UserProfileOutput::from)
                .map_err(map_user_profile_error)
        }
        .boxed()
    }
}

impl AccessDecisionStore for PostgresUserProfileStore<'_> {
    type Error = UserProfileError;

    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, UserProfileError>> {
        async move {
            permission_checks::can(self.conn, actor, action, scope)
                .await
                .map_err(map_user_profile_error)
        }
        .boxed()
    }
}

impl From<User> for UserProfileOutput {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            date_of_birth: user.date_of_birth,
            created_at: user.created_at,
            kyc_verified: user.kyc_verified,
            email_verified: user.email_verified,
        }
    }
}

fn map_user_profile_error(error: diesel::result::Error) -> UserProfileError {
    match error {
        diesel::result::Error::NotFound => UserProfileError::NotFound,
        other => UserProfileError::Database(other.to_string()),
    }
}
