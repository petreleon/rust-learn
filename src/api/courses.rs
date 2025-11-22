use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;
use crate::db;
use crate::models::course::{Course, NewCourse, UpdateCourse};
use crate::db::schema::courses;

#[get("")]
async fn list_courses(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = courses::table.load::<Course>(&mut conn);

    match result {
        Ok(course_list) => HttpResponse::Ok().json(course_list),
        Err(e) => {
            eprintln!("DB error listing courses: {}", e);
            HttpResponse::InternalServerError().body("Failed to load courses")
        }
    }
}

#[get("/{id}")]
async fn get_course(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = courses::table.find(course_id).first::<Course>(&mut conn);

    match result {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("Course not found"),
        Err(e) => {
            eprintln!("DB error fetching course {}: {}", course_id, e);
            HttpResponse::InternalServerError().body("Failed to fetch course")
        }
    }
}

use crate::models::courses_organizations::NewCourseOrganization;
use crate::db::schema::courses_organizations;

#[derive(Deserialize)]
pub struct CreateCourseRequest {
    pub title: String,
    pub organization_ids: Vec<i32>,
}

#[post("")]
async fn create_course(pool: web::Data<db::DbPool>, req: web::Json<CreateCourseRequest>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
        let new_course = NewCourse {
            title: req.title.clone(),
        };

        let course = diesel::insert_into(courses::table)
            .values(&new_course)
            .get_result::<Course>(conn)?;

        for (index, org_id) in req.organization_ids.iter().enumerate() {
            let new_link = NewCourseOrganization {
                course_id: course.id,
                organization_id: *org_id,
                order: index as i32,
            };
            diesel::insert_into(courses_organizations::table)
                .values(&new_link)
                .execute(conn)?;
        }

        Ok(course)
    });

    match result {
        Ok(course) => HttpResponse::Created().json(course),
        Err(e) => {
            eprintln!("DB error creating course: {}", e);
            HttpResponse::InternalServerError().body("Failed to create course")
        }
    }
}

#[put("/{id}")]
async fn update_course(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    req: web::Json<UpdateCourse>,
) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::update(courses::table.find(course_id))
        .set(&*req)
        .get_result::<Course>(&mut conn);

    match result {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("Course not found"),
        Err(e) => {
            eprintln!("DB error updating course {}: {}", course_id, e);
            HttpResponse::InternalServerError().body("Failed to update course")
        }
    }
}

#[delete("/{id}")]
async fn delete_course(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = diesel::delete(courses::table.find(course_id))
        .execute(&mut conn);

    match result {
        Ok(count) => {
            if count > 0 {
                HttpResponse::Ok().body("Course deleted")
            } else {
                HttpResponse::NotFound().body("Course not found")
            }
        }
        Err(e) => {
            eprintln!("DB error deleting course {}: {}", course_id, e);
            HttpResponse::InternalServerError().body("Failed to delete course")
        }
    }
}

#[get("/{id}/organizations")]
async fn get_course_organizations(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    use crate::models::organization::Organization;
    
    let result = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .inner_join(crate::db::schema::organizations::table)
        .select(crate::db::schema::organizations::all_columns)
        .load::<Organization>(&mut conn);

    match result {
        Ok(orgs) => HttpResponse::Ok().json(orgs),
        Err(e) => {
            eprintln!("DB error fetching course organizations: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch course organizations")
        }
    }
}

pub fn course_scope() -> actix_web::Scope {
    web::scope("/courses")
        .service(list_courses)
        .service(get_course)
        .service(create_course)
        .service(update_course)
        .service(delete_course)
        .service(get_course_organizations)
}
