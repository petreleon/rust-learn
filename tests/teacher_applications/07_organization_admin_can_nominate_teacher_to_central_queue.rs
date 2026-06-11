#[actix_web::test]
async fn organization_admin_can_nominate_teacher_to_central_queue() {
    let mut conn = setup_conn().await;
    let organization = create_organization(&mut conn, &unique_string("teacher_sponsor")).await;
    let nominator = create_user_helper(&mut conn, "teacher_nominator").await;
    let applicant = create_user_helper(&mut conn, "teacher_nominee").await;
    let platform_admin = create_user_helper(&mut conn, "teacher_nomination_reviewer").await;
    force_assign_organization_role(&mut conn, nominator.id(), organization.id, "ADMIN").await;
    assign_role_to_user(&mut conn, platform_admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");

    let application = nominate_application(
        &mut conn,
        nominator.id(),
        organization.id,
        OrganizationTeacherNominationRequest {
            applicant_user_id: applicant.id(),
            requested_scope: None,
            requested_course_id: None,
            experience_summary: "Organization-sponsored instructor candidate.".to_string(),
            portfolio_links: None,
            idempotency_key: None,
        },
    )
    .await
    .expect("organization admin should nominate teacher");

    assert_eq!(application.applicant_user_id, applicant.id());
    assert_eq!(application.organization_sponsor_id, Some(organization.id));
    assert_eq!(application.requested_organization_id, Some(organization.id));

    let audit = list_audit_events(&mut conn, application.id)
        .await
        .expect("audit events should load");
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].event_type, "organization_nominated");

    decide_application(
        &mut conn,
        platform_admin.id(),
        application.id,
        TeacherApplicationDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("approved for sponsored organization".to_string()),
        },
    )
    .await
    .expect("platform admin should approve organization nomination");

    let applicant_has_org_teacher_permission = user_permission_organization_request(
        &mut conn,
        applicant.id(),
        organization.id,
        &Permissions::GENERATE_REPORT.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(
        applicant_has_org_teacher_permission,
        "approved organization-scope applicant should receive the organization teaching bundle"
    );
}
