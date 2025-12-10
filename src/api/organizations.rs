use actix_web::{get, post, put, delete, web, HttpResponse, Responder, HttpRequest};
use diesel::prelude::*;
use serde::Deserialize;
use crate::db;
use crate::models::organization::{Organization, NewOrganization, UpdateOrganization};
use crate::db::schema::organizations;
use crate::utils::jwt_utils::decode_jwt;
use crate::models::user_role_organization::UserRoleOrganization;
use crate::utils::db_utils::organization::assign_role_to_user_in_organization;

#[derive(Deserialize)]
pub struct AssignRoleRequest {
    pub role_name: String,
}

#[get("")]
async fn list_organizations(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = organizations::table.load::<Organization>(&mut conn);

    match result {
        Ok(org_list) => HttpResponse::Ok().json(org_list),
        Err(e) => {
            eprintln!("DB error listing organizations: {}", e);
            HttpResponse::InternalServerError().body("Failed to load organizations")
        }
    }
}

#[get("/{id}")]
async fn get_organization(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let org_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = organizations::table.find(org_id).first::<Organization>(&mut conn);

    match result {
        Ok(org) => HttpResponse::Ok().json(org),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("Organization not found"),
        Err(e) => {
            eprintln!("DB error fetching organization {}: {}", org_id, e);
            HttpResponse::InternalServerError().body("Failed to fetch organization")
        }
    }
}

use crate::models::courses_organizations::NewCourseOrganization;
use crate::db::schema::courses_organizations;

#[derive(Deserialize)]
pub struct CreateOrganizationRequest {
    pub name: String,
    pub website_link: Option<String>,
    pub profile_url: Option<String>,
    pub course_ids: Option<Vec<i32>>,
}

#[post("")]
async fn create_organization(pool: web::Data<db::DbPool>, req: web::Json<CreateOrganizationRequest>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
        let new_org = NewOrganization {
            name: req.name.clone(),
            website_link: req.website_link.clone(),
            profile_url: req.profile_url.clone(),
        };

        let org = diesel::insert_into(organizations::table)
            .values(&new_org)
            .get_result::<Organization>(conn)?;

        if let Some(course_ids) = &req.course_ids {
            for (index, course_id) in course_ids.iter().enumerate() {
                let new_link = NewCourseOrganization {
                    course_id: *course_id,
                    organization_id: org.id,
                    order: index as i32,
                };
                diesel::insert_into(courses_organizations::table)
                    .values(&new_link)
                    .execute(conn)?;
            }
        }

        Ok(org)
    });

    match result {
        Ok(org) => HttpResponse::Created().json(org),
        Err(e) => {
            eprintln!("DB error creating organization: {}", e);
            HttpResponse::InternalServerError().body("Failed to create organization")
        }
    }
}

#[put("/{id}")]
async fn update_organization(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    req: web::Json<UpdateOrganization>,
) -> impl Responder {
    let org_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::update(organizations::table.find(org_id))
        .set(&*req)
        .get_result::<Organization>(&mut conn);

    match result {
        Ok(org) => HttpResponse::Ok().json(org),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("Organization not found"),
        Err(e) => {
            eprintln!("DB error updating organization {}: {}", org_id, e);
            HttpResponse::InternalServerError().body("Failed to update organization")
        }
    }
}

#[delete("/{id}")]
async fn delete_organization(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let org_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::delete(organizations::table.find(org_id))
        .execute(&mut conn);

    match result {
        Ok(count) => {
            if count > 0 {
                HttpResponse::Ok().body("Organization deleted")
            } else {
                HttpResponse::NotFound().body("Organization not found")
            }
        }
        Err(e) => {
            eprintln!("DB error deleting organization {}: {}", org_id, e);
            HttpResponse::InternalServerError().body("Failed to delete organization")
        }
    }
}

#[get("/{id}/courses")]
async fn get_organization_courses(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let org_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    use crate::models::course::Course;
    
    let result = courses_organizations::table
        .filter(courses_organizations::organization_id.eq(org_id))
        .inner_join(crate::db::schema::courses::table)
        .select(crate::db::schema::courses::all_columns)
        .load::<Course>(&mut conn);

    match result {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(e) => {
            eprintln!("DB error fetching organization courses: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch organization courses")
        }
    }
}

pub fn organization_scope() -> actix_web::Scope {
    web::scope("/organizations")
        .service(list_organizations)
        .service(get_organization)
        .service(create_organization)
        .service(update_organization)
        .service(delete_organization)
        .service(get_organization_courses)
}
