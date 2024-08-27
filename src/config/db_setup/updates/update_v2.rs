// src/config/db_setup/updates/update_v2.rs

use diesel::pg::PgConnection;
use diesel::QueryResult;
use std::env;
use crate::db_utils::user_registration_connection::create_user;
use chrono::NaiveDate;

pub fn apply_update_v2(conn: &mut PgConnection) -> QueryResult<()> {
    println!("Applying update v2...");

    // Fetch admin details from environment variables (dotenvy::dotenv() already called elsewhere)
    let admin_name = env::var("ADMIN_NAME").expect("ADMIN_NAME must be set in .env");
    let admin_email = env::var("ADMIN_EMAIL").expect("ADMIN_EMAIL must be set in .env");
    let admin_password = env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set in .env");
    let admin_dob = env::var("ADMIN_DATE_OF_BIRTH").ok().map(|dob| {
        NaiveDate::parse_from_str(&dob, "%Y-%m-%d")
            .expect("ADMIN_DATE_OF_BIRTH must be in the format YYYY-MM-DD")
    });

    // Create the admin user
    let result = create_user(conn, &admin_name, &admin_email, admin_dob, &admin_password);

    match result {
        Ok(user) => println!("Admin user '{}' created successfully with ID {}", user.name, user.id),
        Err(e) => println!("Failed to create admin user: {:?}", e),
    }

    Ok(())
}
