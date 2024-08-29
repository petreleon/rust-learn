// src/config/db_setup/updates/update_v2.rs

use diesel::pg::PgConnection;
use diesel::QueryResult;
use std::env;
use crate::config::constants::roles::Roles;
use crate::utils::db_utils::authentication_registration::create_user;
use crate::utils::db_utils::platform::assign_role_to_user;
use chrono::NaiveDate;

pub fn apply_update_v2(conn: &mut PgConnection) -> QueryResult<()> {
    println!("Applying update v2...");

    // Fetch admin details from environment variables
    let admin_name = env::var("ADMIN_NAME").expect("ADMIN_NAME must be set in .env");
    let admin_email = env::var("ADMIN_EMAIL").expect("ADMIN_EMAIL must be set in .env");
    let admin_password = env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set in .env");
    let admin_dob = env::var("ADMIN_DATE_OF_BIRTH").ok().map(|dob| {
        NaiveDate::parse_from_str(&dob, "%Y-%m-%d")
            .expect("ADMIN_DATE_OF_BIRTH must be in the format YYYY-MM-DD")
    });

    // Attempt to create the admin user
    let user = match create_user(conn, &admin_name, &admin_email, admin_dob, &admin_password) {
        Ok(user) => {
            println!("Admin user '{}' created successfully with ID {}", user.name, user.id);
            user
        },
        Err(e) => {
            eprintln!("Failed to create admin user: {:?}", e);
            return Err(e);
        }
    };

    // Attempt to assign the SUPER_ADMIN role to the newly created admin user
    if let Err(e) = assign_role_to_user(conn, user.id, Roles::SUPER_ADMIN) {
        eprintln!("Failed to assign SUPER_ADMIN role to user '{}': {:?}", user.name, e);
        return Err(e);
    }

    println!("SUPER_ADMIN role assigned to user '{}' successfully", user.name);

    Ok(())
}


