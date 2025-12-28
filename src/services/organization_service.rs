use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncConnection};
use crate::db::DbPool;
use crate::models::organization::{Organization, NewOrganization, UpdateOrganization};
use crate::db::schema::{organizations, courses_organizations};
use crate::models::courses_organizations::NewCourseOrganization;
use crate::models::course::Course;
use crate::utils::db_utils::organization::assign_role_to_user_in_organization;

pub async fn list_organizations(pool: &DbPool) -> Result<Vec<Organization>, String> {
    let mut conn = pool.get().await.map_err(|_| "Failed to get DB connection")?;
    organizations::table.load::<Organization>(&mut conn).await.map_err(|e| format!("DB error: {}", e))
}

pub async fn get_organization(pool: &DbPool, org_id: i32) -> Result<Organization, diesel::result::Error> {
    let mut conn = pool.get().await.map_err(|_| diesel::result::Error::NotFound)?; // Simplified error mapping
    organizations::table.find(org_id).first::<Organization>(&mut conn).await
}

pub struct CreateOrganizationDto {
    pub name: String,
    pub website_link: Option<String>,
    pub profile_url: Option<String>,
    pub course_ids: Option<Vec<i32>>,
}

pub async fn create_organization(pool: &DbPool, req: CreateOrganizationDto) -> Result<Organization, String> {
    let mut conn = pool.get().await.map_err(|_| "Failed to get DB connection".to_string())?;

    conn.transaction::<_, diesel::result::Error, _>(|conn| Box::pin(async move {
        let new_org = NewOrganization {
            name: req.name,
            website_link: req.website_link,
            profile_url: req.profile_url,
        };

        let org = diesel::insert_into(organizations::table)
            .values(&new_org)
            .get_result::<Organization>(conn)
            .await?;

        if let Some(course_ids) = &req.course_ids {
            for (index, course_id) in course_ids.iter().enumerate() {
                let new_link = NewCourseOrganization {
                    course_id: *course_id,
                    organization_id: org.id,
                    order: index as i32,
                };
                diesel::insert_into(courses_organizations::table)
                    .values(&new_link)
                    .execute(conn)
                    .await?;
            }
        }

        Ok(org)
    })).await.map_err(|e| format!("DB error: {}", e))
}

pub async fn update_organization(pool: &DbPool, org_id: i32, req: UpdateOrganization) -> Result<Organization, diesel::result::Error> {
    let mut conn = pool.get().await.map_err(|_| diesel::result::Error::NotFound)?; // Simplified
    diesel::update(organizations::table.find(org_id))
        .set(&req)
        .get_result::<Organization>(&mut conn)
        .await
}

pub async fn delete_organization(pool: &DbPool, org_id: i32) -> Result<usize, String> {
    let mut conn = pool.get().await.map_err(|_| "Failed to get DB connection".to_string())?;
    diesel::delete(organizations::table.find(org_id))
        .execute(&mut conn)
        .await
        .map_err(|e| format!("DB error: {}", e))
}

pub async fn get_organization_courses(pool: &DbPool, org_id: i32) -> Result<Vec<Course>, String> {
    let mut conn = pool.get().await.map_err(|_| "Failed to get DB connection".to_string())?;
    
    courses_organizations::table
        .filter(courses_organizations::organization_id.eq(org_id))
        .inner_join(crate::db::schema::courses::table)
        .select(crate::db::schema::courses::all_columns)
        .load::<Course>(&mut conn)
        .await
        .map_err(|e| format!("DB error: {}", e))
}

pub async fn assign_role(pool: &DbPool, requester_id: i32, target_user_id: i32, org_id: i32, role_name: &str) -> Result<(), String> {
    let mut conn = pool.get().await.map_err(|_| "Failed to get DB connection".to_string())?;
    match assign_role_to_user_in_organization(&mut conn, requester_id, target_user_id, org_id, role_name).await {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::RollbackTransaction) => Err("Hierarchy check failed".to_string()),
        Err(diesel::result::Error::NotFound) => Err("Role or User not found".to_string()),
        Err(e) => Err(format!("Error assigning role: {}", e)),
    }
}
