pub mod schema;

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub type DbPool = Pool<AsyncPgConnection>;

pub fn try_establish_connection() -> Result<DbPool, String> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be set in .env file".to_string())?;

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);

    Pool::builder(config)
        .build()
        .map_err(|error| format!("Failed to create pool: {error}"))
}

pub fn establish_connection() -> DbPool {
    try_establish_connection().expect("Failed to establish database connection")
}
