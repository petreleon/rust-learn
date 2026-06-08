pub mod schema;

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub type DbPool = Pool<AsyncPgConnection>;

pub fn try_establish_connection() -> Result<DbPool, String> {
    let database_url = database_url_from_env()?;

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);

    Pool::builder(config)
        .build()
        .map_err(|error| format!("Failed to create pool: {error}"))
}

fn database_url_from_env() -> Result<String, String> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be set in .env file".to_string())?;

    Ok(normalize_database_url_for_debug_host(
        database_url,
        cfg!(debug_assertions),
        host_port_resolves,
    ))
}

fn normalize_database_url_for_debug_host<F>(
    database_url: String,
    is_debug_build: bool,
    host_resolves: F,
) -> String
where
    F: Fn(&str, u16) -> bool,
{
    if should_use_host_published_postgres(&database_url, is_debug_build, host_resolves) {
        return database_url.replacen("@db:5432/", "@localhost:5433/", 1);
    }

    database_url
}

fn should_use_host_published_postgres<F>(
    database_url: &str,
    is_debug_build: bool,
    host_resolves: F,
) -> bool
where
    F: Fn(&str, u16) -> bool,
{
    is_debug_build && database_url.contains("@db:5432/") && !host_resolves("db", 5432)
}

fn host_port_resolves(host: &str, port: u16) -> bool {
    use std::net::ToSocketAddrs;

    (host, port)
        .to_socket_addrs()
        .map(|mut addresses| addresses.next().is_some())
        .unwrap_or(false)
}

pub fn establish_connection() -> DbPool {
    try_establish_connection().expect("Failed to establish database connection")
}

#[cfg(test)]
mod tests {
    use super::normalize_database_url_for_debug_host;

    fn compose_database_url() -> String {
        "postgres://user:password@db:5432/rust_learn".to_string()
    }

    #[test]
    fn leaves_non_compose_database_urls_unchanged() {
        let database_url = "postgres://user:password@localhost:5433/rust_learn".to_string();

        assert_eq!(
            normalize_database_url_for_debug_host(database_url.clone(), true, |_, _| false),
            database_url
        );
    }

    #[test]
    fn rewrites_compose_database_url_when_service_dns_is_unavailable() {
        let database_url =
            normalize_database_url_for_debug_host(compose_database_url(), true, |host, port| {
                host != "db" || port != 5432
            });

        assert_eq!(
            database_url,
            "postgres://user:password@localhost:5433/rust_learn"
        );
    }

    #[test]
    fn leaves_compose_database_url_when_service_dns_is_available() {
        let database_url = compose_database_url();

        assert_eq!(
            normalize_database_url_for_debug_host(database_url.clone(), true, |_, _| true),
            database_url
        );
    }

    #[test]
    fn leaves_compose_database_url_in_release_builds() {
        let database_url = compose_database_url();

        assert_eq!(
            normalize_database_url_for_debug_host(database_url.clone(), false, |_, _| false),
            database_url
        );
    }
}
