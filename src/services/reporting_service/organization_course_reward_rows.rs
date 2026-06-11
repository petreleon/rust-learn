async fn organization_course_reward_rows(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    from: Option<chrono::NaiveDate>,
    to: Option<chrono::NaiveDate>,
) -> QueryResult<Vec<OrganizationCourseRewardDashboardData>> {
    use chrono::NaiveDateTime;

    let courses = courses_organizations::table
        .inner_join(courses::table.on(courses_organizations::course_id.eq(courses::id)))
        .filter(courses_organizations::organization_id.eq(organization_id))
        .select((courses::id, courses::title))
        .order(courses::title.asc())
        .load::<(i32, String)>(conn)
        .await?;

    let mut rows = Vec::new();
    for (course_id, course_title) in courses {
        let mut query = reward_candidates::table
            .filter(reward_candidates::course_id.eq(course_id))
            .into_boxed();

        if let Some(from_date) = from {
            query = query.filter(reward_candidates::created_at.ge(NaiveDateTime::new(
                from_date,
                chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            )));
        }
        if let Some(to_date) = to {
            query = query.filter(reward_candidates::created_at.le(NaiveDateTime::new(
                to_date,
                chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
            )));
        }

        let amounts = query
            .select(reward_candidates::approved_amount)
            .load::<Option<BigDecimal>>(conn)
            .await?;

        let reward_candidate_count = amounts.len() as i64;
        let approved_amounts = amounts.into_iter().flatten().collect::<Vec<_>>();
        let approved_reward_count = approved_amounts.len() as i64;
        let approved_amount_total = approved_amounts
            .into_iter()
            .fold(BigDecimal::from(0), |total, amount| total + amount);

        rows.push(OrganizationCourseRewardDashboardData {
            row: OrganizationCourseRewardDashboardRow {
                course_id,
                course_title,
                reward_candidate_count,
                approved_reward_count,
                approved_amount_total: approved_amount_total.to_string(),
            },
            approved_amount_total,
        });
    }

    Ok(rows)
}

async fn organization_wallet_balance_rows(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> QueryResult<Vec<OrganizationWalletBalanceData>> {
    wallets::table
        .filter(wallets::organization_id.eq(Some(organization_id)))
        .filter(wallets::user_id.is_null())
        .select((wallets::id, wallets::value))
        .order(wallets::id.asc())
        .load::<(i32, BigDecimal)>(conn)
        .await
        .map(|rows| {
            rows.into_iter()
                .map(|(wallet_id, balance)| OrganizationWalletBalanceData {
                    row: OrganizationWalletBalanceRow {
                        wallet_id,
                        balance: balance.to_string(),
                    },
                    balance,
                })
                .collect()
        })
}

pub fn platform_report_csv(summary: &PlatformReportSummary) -> String {
    format!(
        "metric,value\nusers,{}\norganizations,{}\ncourses,{}\nwallets,{}\nnotifications,{}\n",
        summary.total_users,
        summary.total_organizations,
        summary.total_courses,
        summary.total_wallets,
        summary.total_notifications
    )
}

pub async fn platform_teacher_applications_csv(
    conn: &mut AsyncPgConnection,
) -> QueryResult<String> {
    let applications = teacher_applications::table
        .order(teacher_applications::created_at.desc())
        .limit(1000)
        .load::<TeacherApplication>(conn)
        .await?;

    let mut csv = String::from(
        "application_id,applicant_user_id,requested_scope,requested_organization_id,requested_course_id,organization_sponsor_id,status,reviewer_id,decision_reason,portfolio_links,created_at,updated_at,decided_at\n",
    );
    for application in applications {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            application.id,
            application.applicant_user_id,
            csv_value(&application.requested_scope),
            csv_optional(application.requested_organization_id),
            csv_optional(application.requested_course_id),
            csv_optional(application.organization_sponsor_id),
            csv_value(&application.status),
            csv_optional(application.reviewer_id),
            csv_value(application.decision_reason.as_deref().unwrap_or("")),
            csv_value(application.portfolio_links.to_string()),
            application.created_at,
            application.updated_at,
            csv_optional(application.decided_at)
        ));
    }

    Ok(csv)
}
