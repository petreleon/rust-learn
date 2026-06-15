#[actix_web::test]
async fn test_list_by_grantee() {
    let mut conn = setup_conn().await;
    let grantor = create_user_helper(&mut conn, "d5g").await;
    let grantee = create_user_helper(&mut conn, "d5e").await;

    for i in 0..2 {
        delegated_permissions::create_delegated_permission(
            &mut conn,
            new_del(
                grantor.id(),
                grantee.id(),
                "VIEW_COURSE_REWARD_STATUS",
                i + 10,
            ),
        )
        .await
        .unwrap();
    }

    let list = delegated_permissions::list_delegated_permissions(
        &mut conn,
        DelegatedPermissionFilter {
            grantee_user_id: Some(grantee.id()),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    assert!(list.len() >= 2);
}

#[actix_web::test]
async fn test_active_filter_excludes_revoked() {
    let mut conn = setup_conn().await;
    let grantor = create_user_helper(&mut conn, "d6g").await;
    let grantee = create_user_helper(&mut conn, "d6e").await;

    let d = delegated_permissions::create_delegated_permission(
        &mut conn,
        new_del(grantor.id(), grantee.id(), "VIEW_COURSE_REWARD_STATUS", 100),
    )
    .await
    .unwrap();

    let active_before = delegated_permissions::list_delegated_permissions(
        &mut conn,
        DelegatedPermissionFilter {
            grantee_user_id: Some(grantee.id()),
            active: Some(true),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert!(active_before.iter().any(|d2| d2.id == d.id));

    delegated_permissions::revoke_delegated_permission(
        &mut conn,
        d.id,
        grantor.id(),
        None,
    )
    .await
    .unwrap();

    let active_after = delegated_permissions::list_delegated_permissions(
        &mut conn,
        DelegatedPermissionFilter {
            grantee_user_id: Some(grantee.id()),
            active: Some(true),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert!(!active_after.iter().any(|d2| d2.id == d.id));
}

#[actix_web::test]
async fn test_has_active_platform_delegation() {
    let mut conn = setup_conn().await;
    let grantor = create_user_helper(&mut conn, "d7g").await;
    let grantee = create_user_helper(&mut conn, "d7e").await;

    delegated_permissions::create_delegated_permission(
        &mut conn,
        NewDelegatedPermission {
            grantor_user_id: grantor.id(),
            grantee_user_id: grantee.id(),
            permission: "VIEW_REWARD_AUDIT".to_string(),
            scope_type: "platform".to_string(),
            organization_id: None,
            course_id: None,
            reason: None,
            expires_at: None,
        },
    )
    .await
    .unwrap();

    let has = delegated_permissions::has_active_platform_delegation(
        &mut conn,
        grantee.id(),
        "VIEW_REWARD_AUDIT",
    )
    .await
    .unwrap();
    assert!(has);
}
