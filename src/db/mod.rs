pub mod schema;

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub type DbPool = Pool<AsyncPgConnection>;

pub fn establish_connection() -> DbPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    
    Pool::builder(config)
        .build()
        .expect("Failed to create pool.")
}