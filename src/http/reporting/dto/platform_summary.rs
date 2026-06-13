use serde::Serialize;

use crate::application::reporting::platform_summary::PlatformSummaryOutput;

#[derive(Debug, Clone, Serialize)]
pub struct PlatformSummaryResponse {
    pub total_users: i64,
    pub total_organizations: i64,
    pub total_courses: i64,
    pub total_wallets: i64,
    pub total_notifications: i64,
}

impl From<PlatformSummaryOutput> for PlatformSummaryResponse {
    fn from(output: PlatformSummaryOutput) -> Self {
        Self {
            total_users: output.total_users,
            total_organizations: output.total_organizations,
            total_courses: output.total_courses,
            total_wallets: output.total_wallets,
            total_notifications: output.total_notifications,
        }
    }
}

pub fn platform_summary_csv(summary: &PlatformSummaryResponse) -> String {
    format!(
        "metric,value\nusers,{}\norganizations,{}\ncourses,{}\nwallets,{}\nnotifications,{}\n",
        summary.total_users,
        summary.total_organizations,
        summary.total_courses,
        summary.total_wallets,
        summary.total_notifications
    )
}

#[cfg(test)]
mod tests {
    use super::{platform_summary_csv, PlatformSummaryResponse};

    #[test]
    fn keeps_legacy_platform_summary_csv_shape() {
        let csv = platform_summary_csv(&PlatformSummaryResponse {
            total_users: 10,
            total_organizations: 3,
            total_courses: 5,
            total_wallets: 8,
            total_notifications: 25,
        });

        assert!(csv.starts_with("metric,value\n"));
        assert!(csv.contains("users,10\n"));
        assert!(csv.contains("notifications,25\n"));
    }
}
