use actix_web::http::StatusCode;
use actix_web::test::TestRequest;
use actix_web::HttpMessage;

use rust_learn::middlewares::db_connection_middleware::{DB_CONNECTION_MIDDLEWARE_CONDITION, DB_CONNECTION_MIDDLEWARE_ERROR};
use rust_learn::db::establish_connection;
use actix_web::web;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

#[test]
fn no_pool_returns_false() {
	// Build a ServiceRequest without app data (no DbPool)
	let srv_req = TestRequest::default().to_srv_request();

	let allowed = (DB_CONNECTION_MIDDLEWARE_CONDITION)(&srv_req);
	assert!(!allowed, "Middleware condition should return false when no pool is set");
}

#[test]
fn error_returns_service_unavailable() {
	let err = (DB_CONNECTION_MIDDLEWARE_ERROR)();
	let resp = err.error_response();
	assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
}

// This test requires a running Postgres accessible via DATABASE_URL and is ignored by default.
// Run it explicitly when your dev DB is up: `cargo test --test db_connection_middleware_tests pool_present_returns_true -- --ignored`
#[test]
#[ignore]
fn pool_present_returns_true() {
	// Build a real pool (reads DATABASE_URL). If DATABASE_URL points to a running Postgres this will succeed.
	let pool = establish_connection();

	// Attach pool as app data and create request
	let srv_req = TestRequest::default()
		.app_data(web::Data::new(pool))
		.to_srv_request();

	// Condition should return true and store a PooledConnection in extensions
	let allowed = (DB_CONNECTION_MIDDLEWARE_CONDITION)(&srv_req);
	assert!(allowed, "Middleware condition should return true when pool is present and healthy");

	let found = srv_req.extensions().get::<PooledConnection<ConnectionManager<PgConnection>>>().is_some();
	assert!(found, "Request extensions should contain a PooledConnection after middleware runs");
}
