fn current_session_test_app(
    pool: DbPool,
) -> App<
    impl actix_service::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(pool.clone()))
        .app_data(current_session_use_case_data(&pool))
        .app_data(notification_inbox_use_case_data(&pool))
        .app_data(notification_preferences_use_case_data(&pool))
        .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
        .service(
            web::scope("/api")
                .configure(rust_learn::http::identity::configure_routes)
                .configure(rust_learn::http::notifications::configure_routes),
        )
}

fn array_contains(value: &Value, expected: &str) -> bool {
    value
        .as_array()
        .map(|items| items.iter().any(|item| item.as_str() == Some(expected)))
        .unwrap_or(false)
}
