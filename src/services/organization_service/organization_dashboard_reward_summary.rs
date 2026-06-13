async fn organization_dashboard_reward_summary(
    conn: &mut diesel_async::AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationDashboardRewardSummary, OrganizationDashboardError> {
    let mut store = PostgresOrganizationRewardDashboardStore::new(conn);
    let reward_dashboard =
        load_organization_reward_dashboard(&mut store, organization_id, None, None)
            .await
            .map_err(map_reward_dashboard_error)?;
    let course_ids = courses_organizations::table
        .filter(courses_organizations::organization_id.eq(organization_id))
        .select(courses_organizations::course_id)
        .load::<i32>(conn)
        .await?;

    let statuses = if course_ids.is_empty() {
        Vec::new()
    } else {
        reward_candidates::table
            .filter(reward_candidates::course_id.eq_any(&course_ids))
            .select(reward_candidates::status)
            .load::<String>(conn)
            .await?
    };
    let failed_count = statuses
        .iter()
        .filter(|status| status.as_str() == REWARD_STATUS_FAILED)
        .count() as i64;
    let needs_reconciliation_count = statuses
        .iter()
        .filter(|status| status.as_str() == REWARD_STATUS_NEEDS_RECONCILIATION)
        .count() as i64;

    Ok(OrganizationDashboardRewardSummary {
        available: true,
        missing_permissions: vec![],
        reward_candidate_count: reward_dashboard.course_reward_count,
        approved_reward_count: reward_dashboard.approved_reward_count,
        approved_amount_total: reward_dashboard.approved_amount_total,
        failed_count,
        needs_reconciliation_count,
    })
}

fn map_reward_dashboard_error(
    error: OrganizationRewardDashboardError,
) -> OrganizationDashboardError {
    match error {
        OrganizationRewardDashboardError::NotFound => OrganizationDashboardError::NotFound,
        OrganizationRewardDashboardError::Connection(message)
        | OrganizationRewardDashboardError::Database(message) => {
            OrganizationDashboardError::Reporting(message)
        }
    }
}

async fn organization_dashboard_wallet_summary(
    conn: &mut diesel_async::AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationDashboardWalletSummary, OrganizationDashboardError> {
    let balances = wallets::table
        .filter(wallets::organization_id.eq(Some(organization_id)))
        .filter(wallets::user_id.is_null())
        .select(wallets::value)
        .load::<BigDecimal>(conn)
        .await?;
    let balance_total = balances
        .iter()
        .cloned()
        .fold(BigDecimal::from(0), |total, balance| total + balance);

    Ok(OrganizationDashboardWalletSummary {
        available: true,
        missing_permissions: vec![],
        wallet_count: balances.len() as i64,
        balance_total: balance_total.to_string(),
    })
}

fn gated_member_summary() -> OrganizationDashboardMemberSummary {
    OrganizationDashboardMemberSummary {
        available: false,
        missing_permissions: vec![Permissions::VIEW_ORGANIZATION.to_string()],
        total: 0,
        verified_email_count: 0,
        kyc_ready_count: 0,
        delegated_permission_count: 0,
    }
}

fn gated_course_summary() -> OrganizationDashboardCourseSummary {
    OrganizationDashboardCourseSummary {
        available: false,
        missing_permissions: vec![Permissions::VIEW_ORGANIZATION.to_string()],
        total: 0,
        draft: 0,
        submitted: 0,
        needs_changes: 0,
        approved: 0,
        published: 0,
        suspended: 0,
        archived: 0,
    }
}

fn gated_teacher_application_summary() -> OrganizationDashboardTeacherApplicationSummary {
    OrganizationDashboardTeacherApplicationSummary {
        available: false,
        missing_permissions: vec![
            Permissions::VIEW_ORG_TEACHER_APPLICATIONS.to_string(),
            Permissions::NOMINATE_TEACHER_FOR_PLATFORM_REVIEW.to_string(),
        ],
        ..Default::default()
    }
}

fn gated_reward_summary() -> OrganizationDashboardRewardSummary {
    OrganizationDashboardRewardSummary {
        available: false,
        missing_permissions: vec![Permissions::VIEW_ORG_REWARD_REPORTS.to_string()],
        reward_candidate_count: 0,
        approved_reward_count: 0,
        approved_amount_total: "0".to_string(),
        failed_count: 0,
        needs_reconciliation_count: 0,
    }
}

fn gated_wallet_summary() -> OrganizationDashboardWalletSummary {
    OrganizationDashboardWalletSummary {
        available: false,
        missing_permissions: vec![
            Permissions::MANAGE_ORG_WALLETS.to_string(),
            Permissions::MANAGE_ORG_REWARD_BUDGET.to_string(),
            Permissions::VIEW_ORG_REWARD_REPORTS.to_string(),
        ],
        wallet_count: 0,
        balance_total: "0".to_string(),
    }
}
