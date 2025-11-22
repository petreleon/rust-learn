use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::db::schema::pending_course_organization_invites;
use crate::models::course::Course;
use crate::models::organization::Organization;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(Course))]
#[diesel(belongs_to(Organization))]
#[diesel(table_name = pending_course_organization_invites)]
#[diesel(primary_key(id))]
pub struct PendingCourseOrganizationInvite {
    pub id: i32,
    pub course_id: i32,
    pub organization_id: i32,
    pub order: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = pending_course_organization_invites)]
pub struct NewPendingCourseOrganizationInvite {
    pub course_id: i32,
    pub organization_id: i32,
    pub order: i32,
}
