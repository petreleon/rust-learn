use crate::http::reporting::dto::OrganizationRewardDashboardResponse;

pub fn organization_reward_dashboard_csv(
    dashboard: &OrganizationRewardDashboardResponse,
) -> String {
    let mut csv = String::from("section,metric,value\n");
    csv.push_str(&format!(
        "organization,organization_id,{}\n",
        dashboard.organization_id
    ));
    csv.push_str(&format!(
        "organization,organization_name,{}\n",
        csv_value(&dashboard.organization_name)
    ));
    csv.push_str(&format!(
        "teacher_applications,total,{}\n",
        dashboard.sponsored_teacher_applications.total
    ));
    csv.push_str(&format!(
        "teacher_applications,submitted,{}\n",
        dashboard.sponsored_teacher_applications.submitted
    ));
    csv.push_str(&format!(
        "teacher_applications,approved,{}\n",
        dashboard.sponsored_teacher_applications.approved
    ));
    csv.push_str(&format!(
        "reward_candidates,total,{}\n",
        dashboard.course_reward_count
    ));
    csv.push_str(&format!(
        "reward_candidates,approved,{}\n",
        dashboard.approved_reward_count
    ));
    csv.push_str(&format!(
        "reward_candidates,approved_amount_total,{}\n",
        csv_value(&dashboard.approved_amount_total)
    ));
    csv.push_str(&format!(
        "wallets,balance_total,{}\n",
        csv_value(&dashboard.wallet_balance_total)
    ));

    csv.push_str("\ncourses,course_id,course_title,reward_candidate_count,approved_reward_count,approved_amount_total\n");
    for row in &dashboard.courses {
        csv.push_str(&format!(
            "courses,{},{},{},{},{}\n",
            row.course_id,
            csv_value(&row.course_title),
            row.reward_candidate_count,
            row.approved_reward_count,
            csv_value(&row.approved_amount_total)
        ));
    }

    csv.push_str("\nwallets,wallet_id,balance\n");
    for row in &dashboard.wallets {
        csv.push_str(&format!(
            "wallets,{},{}\n",
            row.wallet_id,
            csv_value(&row.balance)
        ));
    }
    csv
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
    use super::*;
    use crate::http::reporting::dto::{
        OrganizationCourseRewardDashboardRowResponse, OrganizationWalletBalanceRowResponse,
        TeacherApplicationDashboardSummaryResponse,
    };

    #[test]
    fn keeps_legacy_organization_reward_dashboard_csv_shape() {
        let csv = organization_reward_dashboard_csv(&OrganizationRewardDashboardResponse {
            organization_id: 42,
            organization_name: "My Org".to_string(),
            sponsored_teacher_applications: TeacherApplicationDashboardSummaryResponse {
                total: 3,
                submitted: 1,
                approved: 1,
                rejected: 1,
                ..Default::default()
            },
            course_reward_count: 10,
            approved_reward_count: 5,
            approved_amount_total: "500".to_string(),
            courses: vec![OrganizationCourseRewardDashboardRowResponse {
                course_id: 1,
                course_title: "Rust 101".to_string(),
                reward_candidate_count: 3,
                approved_reward_count: 2,
                approved_amount_total: "200".to_string(),
            }],
            wallets: vec![OrganizationWalletBalanceRowResponse {
                wallet_id: 1,
                balance: "300".to_string(),
            }],
            wallet_balance_total: "300".to_string(),
        });

        assert!(csv.contains("organization,organization_id,42"));
        assert!(csv.contains("organization,organization_name,My Org"));
        assert!(csv.contains("teacher_applications,total,3"));
        assert!(csv.contains("reward_candidates,total,10"));
        assert!(csv.contains("courses,course_id,"));
        assert!(csv.contains("courses,1,Rust 101"));
        assert!(csv.contains("wallets,1,300"));
    }
}
