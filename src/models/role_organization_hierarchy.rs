use crate::db::schema::role_organization_hierarchy;
use diesel::prelude::*;
use crate::models::role::OrganizationRole;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(OrganizationRole))]
#[diesel(table_name = role_organization_hierarchy)]
pub struct RoleOrganizationHierarchy {
    pub id: i32,
    pub organization_role_id: Option<i32>,
    pub hierarchy_level: i32,
}

impl RoleOrganizationHierarchy {
    pub fn get_min_level(conn: &mut PgConnection, p_user_id: i32, p_org_id: i32) -> QueryResult<Option<i32>> {
        use crate::db::schema::{role_organization_hierarchy, user_role_organization};
        use diesel::dsl::min;

        role_organization_hierarchy::table
            .inner_join(user_role_organization::table.on(
                role_organization_hierarchy::organization_role_id.eq(user_role_organization::organization_role_id)
            ))
            .filter(user_role_organization::user_id.eq(p_user_id))
            .filter(user_role_organization::organization_id.eq(p_org_id))
            .select(min(role_organization_hierarchy::hierarchy_level))
            .first::<Option<i32>>(conn)
    }
}
