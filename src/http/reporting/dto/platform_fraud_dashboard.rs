use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::reporting::platform_fraud_dashboard::{
    FraudBlockDashboardRowOutput, FraudBlockScopeSummaryOutput, PlatformFraudDashboardOutput,
};

#[derive(Debug, Clone, Serialize)]
pub struct PlatformFraudDashboardResponse {
    pub active_total: i64,
    pub active_by_scope: FraudBlockScopeSummaryResponse,
    pub active_blocks: Vec<FraudBlockDashboardRowResponse>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct FraudBlockScopeSummaryResponse {
    pub teacher: i64,
    pub organization: i64,
    pub course: i64,
    pub reward_policy: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct FraudBlockDashboardRowResponse {
    pub id: i64,
    pub scope_type: String,
    pub teacher_user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub reward_policy_id: Option<i64>,
    pub reason: String,
    pub evidence_reference: Option<String>,
    pub created_by_user_id: i32,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<PlatformFraudDashboardOutput> for PlatformFraudDashboardResponse {
    fn from(output: PlatformFraudDashboardOutput) -> Self {
        Self {
            active_total: output.active_total,
            active_by_scope: output.active_by_scope.into(),
            active_blocks: output.active_blocks.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<FraudBlockScopeSummaryOutput> for FraudBlockScopeSummaryResponse {
    fn from(summary: FraudBlockScopeSummaryOutput) -> Self {
        Self {
            teacher: summary.teacher,
            organization: summary.organization,
            course: summary.course,
            reward_policy: summary.reward_policy,
        }
    }
}

impl From<FraudBlockDashboardRowOutput> for FraudBlockDashboardRowResponse {
    fn from(row: FraudBlockDashboardRowOutput) -> Self {
        Self {
            id: row.id,
            scope_type: row.scope_type,
            teacher_user_id: row.teacher_user_id,
            organization_id: row.organization_id,
            course_id: row.course_id,
            reward_policy_id: row.reward_policy_id,
            reason: row.reason,
            evidence_reference: row.evidence_reference,
            created_by_user_id: row.created_by_user_id,
            expires_at: row.expires_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

pub fn platform_fraud_dashboard_csv(dashboard: &PlatformFraudDashboardResponse) -> String {
    let mut csv = String::from("section,metric,value\n");
    csv.push_str(&format!(
        "fraud_blocks,active_total,{}\n",
        dashboard.active_total
    ));
    csv.push_str(&format!(
        "fraud_blocks,teacher,{}\n",
        dashboard.active_by_scope.teacher
    ));
    csv.push_str(&format!(
        "fraud_blocks,organization,{}\n",
        dashboard.active_by_scope.organization
    ));
    csv.push_str(&format!(
        "fraud_blocks,course,{}\n",
        dashboard.active_by_scope.course
    ));
    csv.push_str(&format!(
        "fraud_blocks,reward_policy,{}\n",
        dashboard.active_by_scope.reward_policy
    ));
    csv.push_str("\nactive_fraud_blocks,id,scope_type,teacher_user_id,organization_id,course_id,reward_policy_id,reason,evidence_reference,created_by_user_id,expires_at,created_at\n");
    for row in &dashboard.active_blocks {
        csv.push_str(&format!(
            "active_fraud_blocks,{},{},{},{},{},{},{},{},{},{},{}\n",
            row.id,
            csv_value(&row.scope_type),
            csv_optional(row.teacher_user_id),
            csv_optional(row.organization_id),
            csv_optional(row.course_id),
            csv_optional(row.reward_policy_id),
            csv_value(&row.reason),
            csv_value(row.evidence_reference.as_deref().unwrap_or("")),
            row.created_by_user_id,
            csv_optional(row.expires_at),
            row.created_at
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

fn csv_optional(value: Option<impl ToString>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_legacy_platform_fraud_dashboard_csv_shape() {
        let now = Utc::now();
        let csv = platform_fraud_dashboard_csv(&PlatformFraudDashboardResponse {
            active_total: 1,
            active_by_scope: FraudBlockScopeSummaryResponse {
                teacher: 1,
                ..Default::default()
            },
            active_blocks: vec![FraudBlockDashboardRowResponse {
                id: 7,
                scope_type: "teacher".to_string(),
                teacher_user_id: Some(42),
                organization_id: None,
                course_id: None,
                reward_policy_id: None,
                reason: "needs, review".to_string(),
                evidence_reference: Some("case\"7".to_string()),
                created_by_user_id: 1,
                expires_at: None,
                created_at: now,
                updated_at: now,
            }],
        });

        assert!(csv.starts_with("section,metric,value\n"));
        assert!(csv.contains("fraud_blocks,active_total,1\n"));
        assert!(csv.contains("active_fraud_blocks,7,teacher,42"));
        assert!(csv.contains("\"needs, review\""));
        assert!(csv.contains("\"case\"\"7\""));
    }
}
