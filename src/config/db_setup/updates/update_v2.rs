// src/config/db_setup/updates/update_v2.rs

use crate::application::identity::password_policy::validate_password_strength;
use crate::config::constants::roles::Roles;
use crate::infra::postgres::access_control::{platform_role_records, role_catalog_store};
use crate::infra::postgres::identity::bootstrap_accounts::{
    create_verified_password_account, BootstrapPasswordAccount,
};
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

        let user = create_verified_password_account(
            conn,
            BootstrapPasswordAccount {
                name: admin_name,
                email: admin_email,
                date_of_birth: admin_dob,
                password: admin_password,
            },
        )
        .await
        .context("Failed to create admin user")?;
        log::info!(
            "event=bootstrap_admin_created version=2 user_id={} user_name={}",
            user.user_id,
            user.name
        );

        let role_id =
            role_catalog_store::platform_role_id_by_name(conn, &Roles::SUPER_ADMIN.to_string())
                .await
                .context("Failed to find SUPER_ADMIN platform role")?;

        platform_role_records::assign_platform_role_to_user(conn, user.user_id, role_id)
            .await
            .with_context(|| {
                format!("Failed to assign SUPER_ADMIN role to user '{}'", user.name)
            })?;

        log::info!(
            "event=bootstrap_admin_role_assigned version=2 user_id={} role={}",
            user.user_id,
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
