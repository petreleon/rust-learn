// src/config/db_setup/updates/update_v2.rs

use crate::application::identity::password_policy::validate_password_strength;
use crate::config::constants::roles::Roles;
use crate::repositories::platform_repository::assign_role_to_user;
use crate::repositories::user_repository::create_user;
use anyhow::{anyhow, Context, Result};
use chrono::NaiveDate;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};
use std::env;

pub fn apply_update_v2(conn: &mut AsyncPgConnection) -> BoxFuture<'_, Result<()>> {
    async move {
        log::info!("event=db_version_update_apply_started version=2");

        // Fetch admin details from environment variables
        let admin_name = required_env("ADMIN_NAME")?;
        let admin_email = required_env("ADMIN_EMAIL")?;
        let admin_password = required_env("ADMIN_PASSWORD")?;
        validate_admin_password(&admin_password)?;
        let admin_dob = optional_admin_date_of_birth()?;

        // Attempt to create the admin user
        let user = create_user(conn, &admin_name, &admin_email, admin_dob, &admin_password)
            .await
            .context("Failed to create admin user")?;
        log::info!(
            "event=bootstrap_admin_created version=2 user_id={} user_name={}",
            user.id,
            user.name
        );

        // Attempt to assign the SUPER_ADMIN role to the newly created admin user
        assign_role_to_user(conn, user.id, Roles::SUPER_ADMIN)
            .await
            .with_context(|| {
                format!("Failed to assign SUPER_ADMIN role to user '{}'", user.name)
            })?;

        log::info!(
            "event=bootstrap_admin_role_assigned version=2 user_id={} role={}",
            user.id,
            Roles::SUPER_ADMIN
        );

        Ok(())
    }
    .boxed()
}

fn required_env(name: &str) -> Result<String> {
    env::var(name).with_context(|| format!("{name} must be set in .env"))
}

fn validate_admin_password(password: &str) -> Result<()> {
    const DISALLOWED_DEFAULTS: &[&str] = &[
        "supersecretpassword",
        "rustlearn-admin",
        "replace-with-strong-admin-password",
    ];

    if DISALLOWED_DEFAULTS.contains(&password) {
        return Err(anyhow!(
            "ADMIN_PASSWORD must not use a default or placeholder value"
        ));
    }

    validate_password_strength(password)
        .map_err(|message| anyhow!("ADMIN_PASSWORD does not meet password policy: {message}"))
}

fn optional_admin_date_of_birth() -> Result<Option<NaiveDate>> {
    env::var("ADMIN_DATE_OF_BIRTH")
        .ok()
        .map(|dob| {
            NaiveDate::parse_from_str(&dob, "%Y-%m-%d")
                .with_context(|| "ADMIN_DATE_OF_BIRTH must be in the format YYYY-MM-DD")
        })
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::validate_admin_password;

    #[test]
    fn rejects_known_default_admin_passwords() {
        for password in [
            "supersecretpassword",
            "rustlearn-admin",
            "replace-with-strong-admin-password",
        ] {
            let error = validate_admin_password(password).unwrap_err();

            assert!(error.to_string().contains("default or placeholder"));
        }
    }

    #[test]
    fn rejects_weak_admin_passwords() {
        let error = validate_admin_password("alllowercasepassword").unwrap_err();

        assert!(error.to_string().contains("password policy"));
    }

    #[test]
    fn accepts_strong_admin_passwords() {
        assert!(validate_admin_password("UniqueAdminPass1!").is_ok());
    }
}
