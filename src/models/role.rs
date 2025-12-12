use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::db::schema::{platform_roles, organization_roles, course_roles};
use serde::Serialize;

#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = platform_roles)]
pub struct PlatformRole {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl PlatformRole {
    pub async fn find_by_name(role_name: &str, conn: &mut AsyncPgConnection) -> QueryResult<i32> {
        use crate::db::schema::platform_roles::dsl::*;
        platform_roles
            .filter(name.eq(role_name))
            .select(id)
            .first::<i32>(conn)
            .await
    }
}

#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = organization_roles)]
pub struct OrganizationRole {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl OrganizationRole {
    pub async fn find_by_name(role_name: &str, conn: &mut AsyncPgConnection) -> QueryResult<i32> {
        use crate::db::schema::organization_roles::dsl::*;
        organization_roles
            .filter(name.eq(role_name))
            .select(id)
            .first::<i32>(conn)
            .await
    }
}

#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = course_roles)]
pub struct CourseRole {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl CourseRole {
    pub async fn find_by_name(role_name: &str, conn: &mut AsyncPgConnection) -> QueryResult<i32> {
        use crate::db::schema::course_roles::dsl::*;
        course_roles
            .filter(name.eq(role_name))
            .select(id)
            .first::<i32>(conn)
            .await
    }
}