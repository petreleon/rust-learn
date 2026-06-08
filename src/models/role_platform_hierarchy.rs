use crate::db::schema::role_platform_hierarchy;
use crate::models::role::PlatformRole;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(PlatformRole))]
#[diesel(table_name = role_platform_hierarchy)]
pub struct RolePlatformHierarchy {
    pub id: i32,
    pub platform_role_id: Option<i32>, // updated field name
    pub hierarchy_level: i32,
}

impl RolePlatformHierarchy {
    pub async fn get_min_level(
        conn: &mut AsyncPgConnection,
        p_user_id: i32,
    ) -> QueryResult<Option<i32>> {
        use crate::db::schema::{role_platform_hierarchy, user_role_platform};

        role_platform_hierarchy::table
            .inner_join(user_role_platform::table.on(
                role_platform_hierarchy::platform_role_id.eq(user_role_platform::platform_role_id),
            ))
            .filter(user_role_platform::user_id.eq(p_user_id))
            .select(diesel::dsl::min(role_platform_hierarchy::hierarchy_level))
            .first::<Option<i32>>(conn)
            .await
    }

    pub async fn get_role_level(conn: &mut AsyncPgConnection, p_role_id: i32) -> QueryResult<i32> {
        use crate::db::schema::role_platform_hierarchy::dsl::*;

        role_platform_hierarchy
            .filter(platform_role_id.eq(p_role_id))
            .select(hierarchy_level)
            .first::<i32>(conn)
            .await
    }
}
