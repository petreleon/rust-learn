async fn get_learner_course_learning_route(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match get_learner_course_learning(&mut conn, requester.user_id, path.into_inner()).await {
        Ok(learning) => HttpResponse::Ok().json(learning),
        Err(error) => learner_course_catalog_error_response(error),
    }
}

async fn get_learner_course_catalog_detail(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match get_learner_course_detail(&mut conn, requester.user_id, path.into_inner()).await {
        Ok(detail) => HttpResponse::Ok().json(detail),
        Err(error) => learner_course_catalog_error_response(error),
    }
}

async fn list_courses(
    pool: web::Data<db::DbPool>,
    query: web::Query<CourseDiscoveryParams>,
) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let discovery = CourseDiscoveryQuery::new(
        query.search.clone(),
        query.organization_id,
        query.limit,
        query.offset,
    );
    let result = discover_courses(&mut conn, discovery).await;

    match result {
        Ok(course_list) => HttpResponse::Ok().json(course_list),
        Err(e) => {
            log::error!(
                "event=course_list_failed search={:?} organization_id={:?} limit={:?} offset={:?} error={}",
                query.search,
                query.organization_id,
                query.limit,
                query.offset,
                e
            );
            HttpResponse::InternalServerError().body("Failed to load courses")
        }
    }
}

async fn get_course(path: web::Path<i32>, pool: web::Data<db::DbPool>) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = courses::table
        .find(course_id)
        .first::<Course>(&mut conn)
        .await;

    match result {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().body("Course not found"),
        Err(e) => {
            log::error!(
                "event=course_fetch_failed course_id={} error={}",
                course_id,
                e
            );
            HttpResponse::InternalServerError().body("Failed to fetch course")
        }
    }
}

use crate::db::schema::courses_organizations;
#[derive(Deserialize)]
pub struct CreateCourseRequest {
    pub title: String,
    pub organization_ids: Vec<i32>,
}

async fn create_course(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    body: web::Json<CreateCourseRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result = create_course_with_invites_for_actor(
        &mut conn,
        requester.user_id,
        body.title.clone(),
        body.organization_ids.clone(),
    )
    .await;

    match result {
        Ok(course) => HttpResponse::Created().json(course),
        Err(error) => course_creation_error_response(error),
    }
}

async fn update_course(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    body: web::Json<UpdateCourse>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let result =
        update_course_for_actor(&mut conn, requester.user_id, course_id, body.into_inner()).await;

    match result {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(error) => course_update_error_response(error),
    }
}
