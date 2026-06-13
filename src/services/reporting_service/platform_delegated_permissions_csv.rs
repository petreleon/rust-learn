pub async fn platform_delegated_permissions_csv(
    conn: &mut AsyncPgConnection,
) -> QueryResult<String> {
    let now = chrono::Utc::now();
    let delegations = delegated_permissions::table
        .order(delegated_permissions::created_at.desc())
        .limit(1000)
        .load::<DelegatedPermission>(conn)
        .await?;

    let mut csv = String::from(
        "delegated_permission_id,grantor_user_id,grantee_user_id,permission,scope_type,organization_id,course_id,state,reason,expires_at,revoked_at,revoked_by_user_id,revoke_reason,created_at,updated_at\n",
    );
    for delegation in delegations {
        let state = if delegation.revoked_at.is_some() {
            "revoked"
        } else if delegation
            .expires_at
            .as_ref()
            .map(|expires_at| *expires_at <= now)
            .unwrap_or(false)
        {
            "expired"
        } else {
            "active"
        };

        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            delegation.id,
            delegation.grantor_user_id,
            delegation.grantee_user_id,
            csv_value(&delegation.permission),
            csv_value(&delegation.scope_type),
            csv_optional(delegation.organization_id),
            csv_optional(delegation.course_id),
            state,
            csv_value(delegation.reason.as_deref().unwrap_or("")),
            csv_optional(delegation.expires_at),
            csv_optional(delegation.revoked_at),
            csv_optional(delegation.revoked_by_user_id),
            csv_value(delegation.revoke_reason.as_deref().unwrap_or("")),
            delegation.created_at,
            delegation.updated_at
        ));
    }

    Ok(csv)
}
