use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::db;
use crate::db::schema::courses_organizations;
use crate::models::organization::Organization;

pub(super) async fn get_course_organizations(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .inner_join(crate::db::schema::organizations::table)
        .select(crate::db::schema::organizations::all_columns)
        .load::<Organization>(&mut conn)
        .await;

    match result {
        Ok(orgs) => HttpResponse::Ok().json(orgs),
        Err(e) => {
            log::error!(
                "event=course_organizations_fetch_failed course_id={} error={}",
                course_id,
                e
            );
            HttpResponse::InternalServerError().body("Failed to fetch course organizations")
        }
    }
}
