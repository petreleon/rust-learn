#[actix_web::test]
async fn platform_admin_and_moderator_have_amount_approval_but_not_candidate_approval() {
    let mut conn = setup_conn().await;

    let admin = create_user(
        &mut conn,
        "Reward Admin Test",
        &unique_email("reward-admin"),
        Some(NaiveDate::from_ymd_opt(1994, 4, 4).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create admin user");
    let moderator = create_user(
        &mut conn,
        "Reward Moderator Test",
        &unique_email("reward-moderator"),
        Some(NaiveDate::from_ymd_opt(1995, 5, 5).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create moderator user");

    assign_platform_role_to_user(&mut conn, admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");
    assign_platform_role_to_user(&mut conn, moderator.id(), Roles::MODERATOR)
        .await
        .expect("failed to assign MODERATOR role");

    for user_id in [admin.id(), moderator.id()] {
        let can_approve_amount = has_platform_permission(
            &mut conn,
            user_id,
            &Permissions::APPROVE_REWARD_AMOUNT.to_string(),
        )
        .await
        .expect("permission query failed");
        assert!(
            can_approve_amount,
            "platform reviewer should approve amount"
        );

        let can_approve_candidate = has_platform_permission(
            &mut conn,
            user_id,
            &Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string(),
        )
        .await
        .expect("permission query failed");
        assert!(
            !can_approve_candidate,
            "platform amount approval must not approve course reward candidates"
        );
    }

    let admin_can_block_teacher = has_platform_permission(
        &mut conn,
        admin.id(),
        &Permissions::BLOCK_REWARD_TEACHER.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(
        admin_can_block_teacher,
        "ADMIN should be able to block reward fraud"
    );

    let moderator_can_block_teacher = has_platform_permission(
        &mut conn,
        moderator.id(),
        &Permissions::BLOCK_REWARD_TEACHER.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(
        !moderator_can_block_teacher,
        "MODERATOR should not get fraud-block permission by default"
    );
}
