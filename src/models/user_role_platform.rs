use crate::db::schema::user_role_platform;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::models::user::User;
use crate::models::role::PlatformRole;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(PlatformRole))]
#[diesel(table_name = user_role_platform)]
pub struct UserRolePlatform {
    pub id: i32,
    pub user_id: i32,
    pub platform_role_id: i32,
}

impl UserRolePlatform {
    pub async fn has_permission(conn: &mut AsyncPgConnection, p_user_id: i32, p_permission: &str) -> QueryResult<bool> {
        use crate::db::schema::{platform_roles, role_permission_platform, user_role_platform};
        
        let has_permission = diesel::select(diesel::dsl::exists(
            user_role_platform::table
                .inner_join(role_permission_platform::table.on(
                    user_role_platform::platform_role_id.eq(role_permission_platform::platform_role_id)
                ))
                .filter(user_role_platform::user_id.eq(p_user_id))
                .filter(role_permission_platform::permission.eq(p_permission))
        ))
        .get_result(conn)
        .await?;

        Ok(has_permission)
    }

    pub async fn assign(conn: &mut AsyncPgConnection, p_user_id: i32, p_platform_role_id: i32) -> QueryResult<usize> {
        use crate::db::schema::user_role_platform::dsl::*;
        
        let new_user_role = (
            user_id.eq(p_user_id),
            platform_role_id.eq(p_platform_role_id),
        );

        diesel::insert_into(user_role_platform)
            .values(&new_user_role)
            .execute(conn)
            .await
    }
}
