use crate::db::schema::role_platform_hierarchy;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::models::role::PlatformRole;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(PlatformRole))]
#[diesel(table_name = role_platform_hierarchy)]
pub struct RolePlatformHierarchy {
    pub id: i32,
    pub platform_role_id: Option<i32>, // updated field name
    pub hierarchy_level: i32,
}

impl RolePlatformHierarchy {
    pub async fn get_min_level(conn: &mut AsyncPgConnection, p_user_id: i32) -> QueryResult<Option<i32>> {
        use crate::db::schema::{role_platform_hierarchy, user_role_platform};
        use diesel::dsl::min;

        role_platform_hierarchy::table
            .inner_join(user_role_platform::table.on(
                role_platform_hierarchy::platform_role_id.eq(user_role_platform::platform_role_id)
            ))
            .filter(user_role_platform::user_id.eq(p_user_id))
            .select(min(role_platform_hierarchy::hierarchy_level))
            .first::<Option<i32>>(conn)
            .await
    }
}
