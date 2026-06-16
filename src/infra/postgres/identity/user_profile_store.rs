use diesel::prelude::*;
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
use crate::infra::postgres::schema::{
    platform_roles, role_permission_platform, user_role_platform, users,
};

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

            let users = result.map_err(map_user_profile_error)?;
            let mut outputs = Vec::with_capacity(users.len());
            for user in users {
                outputs.push(user_profile_output(self.conn, user).await?);
            }
            Ok(outputs)
        }
        .boxed()
    }

    fn find_user(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<UserProfileOutput, UserProfileError>> {
        async move {
            let user = users::table
                .find(user_id)
                .first::<User>(self.conn)
                .await
                .map_err(map_user_profile_error)?;
            user_profile_output(self.conn, user).await
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
            platform_roles: Vec::new(),
            platform_permissions: Vec::new(),
        }
    }
}

async fn user_profile_output(
    conn: &mut AsyncPgConnection,
    user: User,
) -> Result<UserProfileOutput, UserProfileError> {
    let mut output = UserProfileOutput::from(user);
    output.platform_roles = load_platform_roles(conn, output.id).await?;
    output.platform_permissions = load_platform_permissions(conn, output.id).await?;
    Ok(output)
}

async fn load_platform_roles(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<Vec<String>, UserProfileError> {
    let mut roles = user_role_platform::table
        .inner_join(
            platform_roles::table
                .on(user_role_platform::platform_role_id.eq(platform_roles::id.nullable())),
        )
        .filter(user_role_platform::user_id.eq(user_id))
        .order(platform_roles::name.asc())
        .select(platform_roles::name)
        .load::<String>(conn)
        .await
        .map_err(map_user_profile_error)?;
    roles.dedup();
    Ok(roles)
}

async fn load_platform_permissions(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<Vec<String>, UserProfileError> {
    let mut permissions = user_role_platform::table
        .inner_join(role_permission_platform::table.on(
            user_role_platform::platform_role_id.eq(role_permission_platform::platform_role_id),
        ))
        .filter(user_role_platform::user_id.eq(user_id))
        .order(role_permission_platform::permission.asc())
        .select(role_permission_platform::permission)
        .load::<String>(conn)
        .await
        .map_err(map_user_profile_error)?;
    permissions.dedup();
    Ok(permissions)
}

fn map_user_profile_error(error: diesel::result::Error) -> UserProfileError {
    match error {
        diesel::result::Error::NotFound => UserProfileError::NotFound,
        other => UserProfileError::Database(other.to_string()),
    }
}
