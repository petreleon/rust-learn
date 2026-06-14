use crate::db::schema::delegated_permissions;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = delegated_permissions)]
pub struct DelegatedPermission {
    pub id: i64,
    pub grantor_user_id: i32,
    pub grantee_user_id: i32,
    pub permission: String,
    pub scope_type: String,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reason: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_by_user_id: Option<i32>,
    pub revoke_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = delegated_permissions)]
pub struct NewDelegatedPermission {
    pub grantor_user_id: i32,
    pub grantee_user_id: i32,
    pub permission: String,
    pub scope_type: String,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reason: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}
