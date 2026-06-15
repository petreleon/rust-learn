use std::sync::Arc;

use actix_web::web;

use crate::application::reporting::platform_csv_exports::PlatformCsvExportsUseCase;
use crate::http::errors::ApiError;
use crate::http::reporting::dto::{
    csv_download, platform_delegated_permissions_csv, platform_reward_approvals_csv,
    platform_teacher_applications_csv, platform_token_payouts_csv, platform_wallet_credits_csv,
    CsvDownload,
};
use crate::http::reporting::errors::platform_csv_export_error;

pub async fn export_platform_teacher_applications(
    exports: web::Data<Arc<dyn PlatformCsvExportsUseCase>>,
) -> Result<CsvDownload, ApiError> {
    exports
        .load_platform_teacher_application_exports()
        .await
        .map(|rows| {
            csv_download(
                "platform-teacher-applications.csv",
                platform_teacher_applications_csv(&rows),
            )
        })
        .map_err(|error| {
            platform_csv_export_error(error, "teacher_applications", "teacher applications")
        })
}

pub async fn export_platform_reward_approvals(
    exports: web::Data<Arc<dyn PlatformCsvExportsUseCase>>,
) -> Result<CsvDownload, ApiError> {
    exports
        .load_platform_reward_approval_exports()
        .await
        .map(|rows| {
            csv_download(
                "platform-reward-approvals.csv",
                platform_reward_approvals_csv(&rows),
            )
        })
        .map_err(|error| platform_csv_export_error(error, "reward_approvals", "reward approvals"))
}

pub async fn export_platform_token_payouts(
    exports: web::Data<Arc<dyn PlatformCsvExportsUseCase>>,
) -> Result<CsvDownload, ApiError> {
    exports
        .load_platform_token_payout_exports()
        .await
        .map(|rows| {
            csv_download(
                "platform-token-payouts.csv",
                platform_token_payouts_csv(&rows),
            )
        })
        .map_err(|error| platform_csv_export_error(error, "token_payouts", "token payouts"))
}

pub async fn export_platform_wallet_credits(
    exports: web::Data<Arc<dyn PlatformCsvExportsUseCase>>,
) -> Result<CsvDownload, ApiError> {
    exports
        .load_platform_wallet_credit_exports()
        .await
        .map(|rows| {
            csv_download(
                "platform-wallet-credits.csv",
                platform_wallet_credits_csv(&rows),
            )
        })
        .map_err(|error| platform_csv_export_error(error, "wallet_credits", "wallet credits"))
}

pub async fn export_platform_delegated_permissions(
    exports: web::Data<Arc<dyn PlatformCsvExportsUseCase>>,
) -> Result<CsvDownload, ApiError> {
    exports
        .load_platform_delegated_permission_exports()
        .await
        .map(|rows| {
            csv_download(
                "platform-delegated-permissions.csv",
                platform_delegated_permissions_csv(&rows),
            )
        })
        .map_err(|error| {
            platform_csv_export_error(error, "delegated_permissions", "delegated permissions")
        })
}
