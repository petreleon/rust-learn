impl From<RewardExecutionJob> for RewardExecutionFailureRow {
    fn from(job: RewardExecutionJob) -> Self {
        RewardExecutionFailureRow {
            reward_execution_job_id: job.id,
            reward_candidate_id: job.reward_candidate_id,
            status: job.status,
            attempts: job.attempts,
            last_error: job.last_error,
            updated_at: job.updated_at,
        }
    }
}

pub async fn organization_report_summary(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> QueryResult<OrganizationReportSummary> {
    let organization_name = organizations::table
        .find(organization_id)
        .select(organizations::name)
        .first::<String>(conn)
        .await?;

    let course_ids = courses_organizations::table
        .filter(courses_organizations::organization_id.eq(organization_id))
        .select(courses_organizations::course_id)
        .load::<i32>(conn)
        .await?;
    let course_count = course_ids.len() as i64;

    let member_ids = user_role_organization::table
        .filter(user_role_organization::organization_id.eq(organization_id))
        .select(user_role_organization::user_id)
        .distinct()
        .load::<Option<i32>>(conn)
        .await?;
    let member_count = member_ids.into_iter().flatten().count() as i64;

    let wallet_count = wallets::table
        .filter(wallets::organization_id.eq(organization_id))
        .count()
        .get_result(conn)
        .await?;

    let course_role_assignment_count = if course_ids.is_empty() {
        0
    } else {
        user_role_course::table
            .filter(user_role_course::course_id.eq_any(course_ids))
            .count()
            .get_result(conn)
            .await?
    };

    Ok(OrganizationReportSummary {
        organization_id,
        organization_name,
        course_count,
        member_count,
        wallet_count,
        course_role_assignment_count,
    })
}

pub async fn organization_reward_dashboard(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    from: Option<chrono::NaiveDate>,
    to: Option<chrono::NaiveDate>,
) -> QueryResult<OrganizationRewardDashboard> {
    let organization_name = organizations::table
        .find(organization_id)
        .select(organizations::name)
        .first::<String>(conn)
        .await?;

    let sponsored_teacher_applications =
        sponsored_teacher_application_summary(conn, organization_id).await?;
    let course_data = organization_course_reward_rows(conn, organization_id, from, to).await?;

    let course_reward_count = course_data
        .iter()
        .map(|data| data.row.reward_candidate_count)
        .sum::<i64>();
    let approved_reward_count = course_data
        .iter()
        .map(|data| data.row.approved_reward_count)
        .sum::<i64>();
    let approved_amount_total = course_data.iter().fold(BigDecimal::from(0), |total, data| {
        total + data.approved_amount_total.clone()
    });
    let courses = course_data.into_iter().map(|data| data.row).collect();

    let wallet_data = organization_wallet_balance_rows(conn, organization_id).await?;
    let wallet_balance_total = wallet_data.iter().fold(BigDecimal::from(0), |total, data| {
        total + data.balance.clone()
    });
    let wallets = wallet_data.into_iter().map(|data| data.row).collect();

    Ok(OrganizationRewardDashboard {
        organization_id,
        organization_name,
        sponsored_teacher_applications,
        course_reward_count,
        approved_reward_count,
        approved_amount_total: approved_amount_total.to_string(),
        courses,
        wallets,
        wallet_balance_total: wallet_balance_total.to_string(),
    })
}

async fn sponsored_teacher_application_summary(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> QueryResult<TeacherApplicationDashboardSummary> {
    let statuses = teacher_applications::table
        .filter(
            teacher_applications::organization_sponsor_id
                .eq(Some(organization_id))
                .or(teacher_applications::requested_organization_id.eq(Some(organization_id))),
        )
        .select(teacher_applications::status)
        .load::<String>(conn)
        .await?;

    let mut summary = TeacherApplicationDashboardSummary::default();
    for status in statuses {
        summary.total += 1;
        match status.as_str() {
            TEACHER_APPLICATION_STATUS_SUBMITTED => summary.submitted += 1,
            TEACHER_APPLICATION_STATUS_NEEDS_CHANGES => summary.needs_changes += 1,
            TEACHER_APPLICATION_STATUS_APPROVED => summary.approved += 1,
            TEACHER_APPLICATION_STATUS_REJECTED => summary.rejected += 1,
            _ => {}
        }
    }

    Ok(summary)
}
