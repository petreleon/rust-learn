use serde::Serialize;

use crate::application::reporting::organization_summary::OrganizationSummaryOutput;

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationSummaryResponse {
    pub organization_id: i32,
    pub organization_name: String,
    pub course_count: i64,
    pub member_count: i64,
    pub wallet_count: i64,
    pub course_role_assignment_count: i64,
}

impl From<OrganizationSummaryOutput> for OrganizationSummaryResponse {
    fn from(output: OrganizationSummaryOutput) -> Self {
        Self {
            organization_id: output.organization_id,
            organization_name: output.organization_name,
            course_count: output.course_count,
            member_count: output.member_count,
            wallet_count: output.wallet_count,
            course_role_assignment_count: output.course_role_assignment_count,
        }
    }
}

pub fn organization_summary_csv(summary: &OrganizationSummaryResponse) -> String {
    format!(
        "metric,value\norganization_id,{}\norganization_name,{}\ncourses,{}\nmembers,{}\nwallets,{}\ncourse_role_assignments,{}\n",
        summary.organization_id,
        csv_value(&summary.organization_name),
        summary.course_count,
        summary.member_count,
        summary.wallet_count,
        summary.course_role_assignment_count
    )
}

fn csv_value(value: impl AsRef<str>) -> String {
    let value = value.as_ref();
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{organization_summary_csv, OrganizationSummaryResponse};

    #[test]
    fn keeps_legacy_organization_summary_csv_shape() {
        let csv = organization_summary_csv(&OrganizationSummaryResponse {
            organization_id: 1,
            organization_name: "Test, Org".to_string(),
            course_count: 3,
            member_count: 12,
            wallet_count: 1,
            course_role_assignment_count: 5,
        });

        assert!(csv.starts_with("metric,value\n"));
        assert!(csv.contains("organization_id,1\n"));
        assert!(csv.contains("organization_name,\"Test, Org\"\n"));
        assert!(csv.contains("courses,3\n"));
        assert!(csv.contains("wallets,1\n"));
    }
}
