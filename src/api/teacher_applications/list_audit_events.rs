async fn list_audit_events(
    req: HttpRequest,
    path: web::Path<i64>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match teacher_application_service::list_audit_events(
        &mut conn,
        requester.user_id,
        path.into_inner(),
    )
    .await
    {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(error) => service_error_response(error),
    }
}

pub fn teacher_application_scope() -> actix_web::Scope {
    web::scope("/teacher-applications")
        .service(
            web::resource("")
                .route(web::post().to(submit_application))
                .route(web::get().to(list_applications)),
        )
        .service(web::resource("/review").route(web::get().to(list_platform_review_applications)))
        .service(web::resource("/me").route(web::get().to(get_my_application)))
        .service(web::resource("/{id}/decision").route(web::put().to(decide_application)))
        .service(web::resource("/{id}/audit").route(web::get().to(list_audit_events)))
}
