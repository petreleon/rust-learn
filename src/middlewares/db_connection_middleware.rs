use once_cell::sync::Lazy;
use actix_web::HttpMessage;
use actix_web::{web, dev::ServiceRequest, Error, HttpResponse};
use std::sync::Arc;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use crate::middlewares::conditional_access_middleware::ConditionalAccessMiddleware;
use crate::db::DbPool;

// Assuming `ConditionalAccessMiddleware` is defined and imported correctly
pub static DB_CONNECTION_MIDDLEWARE_CONDITION: &'static (dyn Fn(&ServiceRequest) -> bool + Sync) = &move |req: &ServiceRequest| -> bool {
    if let Some(pool) = req.app_data::<web::Data<DbPool>>() {
        match pool.get() {
            Ok(mut conn) => {
                // Successfully obtained a database connection; store it in the request's extensions
                req.extensions_mut().insert::<PooledConnection<ConnectionManager<PgConnection>>>(conn);
                true // Permit the request to continue
            },
            Err(_) => {
                // Failed to obtain a database connection
                false // Deny the request
            }
        }
    } else {
        // The pool is not found in the app data
        false // Deny the request
    }
};

pub static DB_CONNECTION_MIDDLEWARE_ERROR: fn() -> actix_web::Error = || {
    // Construct an Error to return when the database connection fails
    actix_web::error::ErrorServiceUnavailable("It can't connect to the database")
};
