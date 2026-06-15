use crate::support::*;

pub(crate) fn current_session_test_app(
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
        .configure(|cfg| rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool))
        .app_data(current_session_use_case_data(&pool))
        .app_data(notification_inbox_use_case_data(&pool))
        .app_data(notification_preferences_use_case_data(&pool))
        .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
        .service(
            web::scope("/api")
                .configure(rust_learn::http::identity::configure_routes)
                .configure(rust_learn::http::notifications::configure_routes),
        )
}

pub(crate) fn array_contains(value: &Value, expected: &str) -> bool {
    value
        .as_array()
        .map(|items| items.iter().any(|item| item.as_str() == Some(expected)))
        .unwrap_or(false)
}

pub(crate) fn capability_contains_permission(scope: &Value, key: &str, permission: &str) -> bool {
    scope["capabilities"]
        .as_array()
        .map(|capabilities| {
            capabilities.iter().any(|capability| {
                capability["key"].as_str() == Some(key)
                    && array_contains(&capability["permissions"], permission)
            })
        })
        .unwrap_or(false)
}
