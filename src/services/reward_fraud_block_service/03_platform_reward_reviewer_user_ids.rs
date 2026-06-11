async fn platform_reward_reviewer_user_ids(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<i32>, RewardFraudBlockError> {
    let now = Utc::now();
    let permissions = platform_fraud_notification_permissions();
    let mut reviewers = user_role_platform::table
        .inner_join(role_permission_platform::table.on(
            user_role_platform::platform_role_id.eq(role_permission_platform::platform_role_id),
        ))
        .select(user_role_platform::user_id)
        .filter(role_permission_platform::permission.eq_any(permissions.as_slice()))
        .distinct()
        .load::<Option<i32>>(conn)
        .await?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    reviewers.extend(
        delegated_permissions::table
            .select(delegated_permissions::grantee_user_id)
            .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_PLATFORM))
            .filter(delegated_permissions::organization_id.is_null())
            .filter(delegated_permissions::course_id.is_null())
            .filter(delegated_permissions::permission.eq_any(permissions.as_slice()))
            .filter(delegated_permissions::revoked_at.is_null())
            .filter(
                delegated_permissions::expires_at
                    .is_null()
                    .or(delegated_permissions::expires_at.gt(now)),
            )
            .distinct()
            .load::<i32>(conn)
            .await?,
    );

    reviewers.sort_unstable();
    reviewers.dedup();
    Ok(reviewers)
}

async fn organization_reward_operator_user_ids(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<Vec<i32>, RewardFraudBlockError> {
    let now = Utc::now();
    let permissions = organization_fraud_notification_permissions();
    let mut operators = user_role_organization::table
        .inner_join(
            role_permission_organization::table.on(user_role_organization::organization_role_id
                .eq(role_permission_organization::organization_role_id)),
        )
        .filter(user_role_organization::organization_id.eq(Some(organization_id)))
        .filter(role_permission_organization::permission.eq_any(permissions.as_slice()))
        .select(user_role_organization::user_id)
        .distinct()
        .load::<Option<i32>>(conn)
        .await?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    operators.extend(
        delegated_permissions::table
            .select(delegated_permissions::grantee_user_id)
            .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_ORGANIZATION))
            .filter(delegated_permissions::organization_id.eq(Some(organization_id)))
            .filter(delegated_permissions::course_id.is_null())
            .filter(delegated_permissions::permission.eq_any(permissions.as_slice()))
            .filter(delegated_permissions::revoked_at.is_null())
            .filter(
                delegated_permissions::expires_at
                    .is_null()
                    .or(delegated_permissions::expires_at.gt(now)),
            )
            .distinct()
            .load::<i32>(conn)
            .await?,
    );

    operators.sort_unstable();
    operators.dedup();
    Ok(operators)
}

fn platform_fraud_notification_permissions() -> [String; 2] {
    [
        Permissions::VIEW_REWARD_AUDIT.to_string(),
        Permissions::MANAGE_REWARD_FRAUD_BLOCKS.to_string(),
    ]
}

fn organization_fraud_notification_permissions() -> [String; 2] {
    [
        Permissions::VIEW_ORG_REWARD_REPORTS.to_string(),
        Permissions::MANAGE_ORG_REWARD_BUDGET.to_string(),
    ]
}

async fn course_organization_ids(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<i32>, RewardFraudBlockError> {
    courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await
        .map_err(RewardFraudBlockError::from)
}

async fn reward_policy_operator_user_ids(
    conn: &mut AsyncPgConnection,
    reward_policy_id: i64,
) -> Result<Vec<i32>, RewardFraudBlockError> {
    let (organization_id, course_id) = reward_policies::table
        .find(reward_policy_id)
        .select((reward_policies::organization_id, reward_policies::course_id))
        .first::<(Option<i32>, Option<i32>)>(conn)
        .await?;

    let mut recipients = Vec::new();
    if let Some(organization_id) = organization_id {
        recipients.extend(organization_reward_operator_user_ids(conn, organization_id).await?);
    }
    if let Some(course_id) = course_id {
        for organization_id in course_organization_ids(conn, course_id).await? {
            recipients.extend(organization_reward_operator_user_ids(conn, organization_id).await?);
        }
    }
    Ok(recipients)
}

struct NormalizedRewardFraudBlock {
    scope_type: String,
    teacher_user_id: Option<i32>,
    organization_id: Option<i32>,
    course_id: Option<i32>,
    reward_policy_id: Option<i64>,
    reason: String,
    evidence_reference: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}
