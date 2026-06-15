use std::collections::HashSet;

use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::access_control::reward_fraud_block_notifications::{
    organization_reward_fraud_block_notification_permissions,
    platform_reward_fraud_block_notification_permissions,
};
use crate::application::rewards::manage_fraud_block::{
    RewardFraudBlockError, RewardFraudBlockOutput,
};
use crate::domain::access_control::delegation::{
    DELEGATED_SCOPE_ORGANIZATION, DELEGATED_SCOPE_PLATFORM,
};
use crate::domain::access_control::permission::Permission;
use crate::infra::postgres::rewards::reward_fraud_block_mappers::map_reward_fraud_block_error;
use crate::infra::postgres::schema::{
    courses_organizations, delegated_permissions, reward_policies, role_permission_organization,
    role_permission_platform, user_role_organization, user_role_platform,
};

pub(super) async fn reward_fraud_block_notification_recipients(
    conn: &mut AsyncPgConnection,
    block: &RewardFraudBlockOutput,
) -> Result<HashSet<i32>, RewardFraudBlockError> {
    let mut recipients = HashSet::new();

    if let Some(teacher_user_id) = block.teacher_user_id {
        recipients.insert(teacher_user_id);
    }
    if let Some(organization_id) = block.organization_id {
        recipients.extend(organization_reward_operator_user_ids(conn, organization_id).await?);
    }
    if let Some(course_id) = block.course_id {
        for organization_id in course_organization_ids(conn, course_id).await? {
            recipients.extend(organization_reward_operator_user_ids(conn, organization_id).await?);
        }
    }
    if let Some(reward_policy_id) = block.reward_policy_id {
        recipients.extend(reward_policy_operator_user_ids(conn, reward_policy_id).await?);
    }

    recipients.extend(platform_reward_reviewer_user_ids(conn).await?);
    Ok(recipients)
}

async fn platform_reward_reviewer_user_ids(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<i32>, RewardFraudBlockError> {
    let now = Utc::now();
    let permissions = permission_keys(platform_reward_fraud_block_notification_permissions());
    let mut reviewers = user_role_platform::table
        .inner_join(role_permission_platform::table.on(
            user_role_platform::platform_role_id.eq(role_permission_platform::platform_role_id),
        ))
        .select(user_role_platform::user_id)
        .filter(role_permission_platform::permission.eq_any(permissions.as_slice()))
        .distinct()
        .load::<Option<i32>>(conn)
        .await
        .map_err(map_reward_fraud_block_error)?
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
            .await
            .map_err(map_reward_fraud_block_error)?,
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
    let permissions = permission_keys(organization_reward_fraud_block_notification_permissions());
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
        .await
        .map_err(map_reward_fraud_block_error)?
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
            .await
            .map_err(map_reward_fraud_block_error)?,
    );

    operators.sort_unstable();
    operators.dedup();
    Ok(operators)
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
        .map_err(map_reward_fraud_block_error)
}

async fn reward_policy_operator_user_ids(
    conn: &mut AsyncPgConnection,
    reward_policy_id: i64,
) -> Result<Vec<i32>, RewardFraudBlockError> {
    let (organization_id, course_id) = reward_policies::table
        .find(reward_policy_id)
        .select((reward_policies::organization_id, reward_policies::course_id))
        .first::<(Option<i32>, Option<i32>)>(conn)
        .await
        .map_err(map_reward_fraud_block_error)?;

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

fn permission_keys<const N: usize>(permissions: [Permission; N]) -> [&'static str; N] {
    permissions.map(Permission::as_str)
}
