use crate::db::schema::role_permission_platform;
use diesel::prelude::*;
use crate::models::role::PlatformRole;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(PlatformRole))]
#[diesel(table_name = role_permission_platform)]
pub struct RolePermissionPlatform {
    pub id: i32,
    pub platform_role_id: Option<i32>, // changed here!
    pub permission: String,
}

impl RolePermissionPlatform {
    pub fn assign(conn: &mut PgConnection, p_platform_role_id: i32, p_permission: &str) -> QueryResult<usize> {
        use crate::db::schema::role_permission_platform;
        use diesel::dsl::{exists, select};
        use diesel::prelude::*;

        // Check if the permission is already assigned to this role.
        let permission_exists = select(exists(
            role_permission_platform::table
                .filter(role_permission_platform::platform_role_id.nullable().eq(Some(p_platform_role_id)))
                .filter(role_permission_platform::permission.eq(p_permission))
        ))
        .get_result(conn)?;

        if permission_exists {
            return Ok(0);
        }

        #[derive(Insertable)]
        #[diesel(table_name = role_permission_platform)]
        struct NewPermission<'a> {
            platform_role_id: Option<i32>,
            permission: &'a str,
        }

        let new_permission = NewPermission {
            platform_role_id: Some(p_platform_role_id),
            permission: p_permission,
        };

        diesel::insert_into(role_permission_platform::table)
            .values(&new_permission)
            .execute(conn)
    }
}
