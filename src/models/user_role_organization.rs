use crate::db::schema::user_role_organization;
use diesel::prelude::*;
use crate::models::user::User;
use crate::models::role::OrganizationRole;
use crate::models::organization::Organization;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(OrganizationRole))]
#[diesel(belongs_to(Organization))]
#[diesel(table_name = user_role_organization)]
pub struct UserRoleOrganization {
    pub id: i32,
    pub user_id: Option<i32>,
    pub organization_role_id: Option<i32>, // updated field name
    pub organization_id: Option<i32>,
}

impl UserRoleOrganization {
    pub fn has_permission(conn: &mut PgConnection, p_user_id: i32, p_org_id: i32, p_permission: &str) -> QueryResult<bool> {
        use crate::db::schema::{organization_roles, role_permission_organization, user_role_organization};
        
        let has_permission = diesel::select(diesel::dsl::exists(
            user_role_organization::table
                .inner_join(organization_roles::table.on(
                    user_role_organization::organization_role_id.eq(organization_roles::id.nullable())
                ))
                .inner_join(role_permission_organization::table.on(
                    organization_roles::id.nullable().eq(role_permission_organization::organization_role_id)
                ))
                .filter(user_role_organization::user_id.eq(p_user_id))
                .filter(user_role_organization::organization_id.eq(p_org_id))
                .filter(role_permission_organization::permission.eq(p_permission))
        ))
        .get_result(conn)?;

        Ok(has_permission)
    }
}
