use crate::support::*;

#[actix_web::test]
async fn org_admin_has_permissions() {
    let mut conn = setup_conn().await;
    let org_name = unique_string("OrgAdminTest");
    let org = create_organization(&mut conn, &org_name).await;

    let subject_user = create_user_helper(&mut conn, "subject_admin").await;
    force_assign_role(&mut conn, subject_user.id(), org.id, "ADMIN").await;

    let allowed_permissions = [
        Permissions::MANAGE_ORG_SETTINGS,
        Permissions::MANAGE_ORG_MEMBERS,
    ];

    for permission in allowed_permissions {
        let has_permission = has_organization_permission(
            &mut conn,
            subject_user.id(),
            org.id,
            &permission.to_string(),
        )
        .await
        .expect("permission query failed");
        assert!(
            has_permission,
            "ADMIN should have permission: {:?}",
            permission
        );
    }
}
