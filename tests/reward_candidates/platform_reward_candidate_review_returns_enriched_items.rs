use crate::{
    force_assign_organization_role::*, link_course_to_organization::*, submission_helper::*,
    support::*, teacher_decision_helper::*,
};

#[actix_web::test]
async fn platform_reward_candidate_review_returns_enriched_items() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("PlatformReviewCourse")).await;
    let teacher = create_user_helper(&mut conn, "platform_review_teacher").await;
    let student = create_user_helper(&mut conn, "platform_review_student").await;
    let reviewer = create_user_helper(&mut conn, "platform_review_reviewer").await;
    let admin = create_user_helper(&mut conn, "platform_review_admin").await;

    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    let reviewer_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("PLATFORM_REVIEWER"),
        &[
            Permissions::VIEW_REWARD_AUDIT,
            Permissions::APPROVE_REWARD_AMOUNT,
        ],
    )
    .await;
    assign_platform_role_id(&mut conn, reviewer.id(), reviewer_role_id).await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_COURSE_COMPLETION).await;

    let candidate = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(student.id(), &unique_string("platform_review_candidate")),
    )
    .await
    .expect("teacher should submit reward candidate");
    decide_reward_candidate_by_teacher(
        &mut conn,
        teacher.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("teacher approved for platform review".to_string()),
        },
    )
    .await
    .expect("teacher should approve candidate");

    drop(conn);

    let pool = establish_connection();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(platform_reward_candidates_use_case(&pool)))
            .app_data(rust_learn::bootstrap::auth_token_verifier_app_data())
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::rewards::platform_reward_candidates_resource()),
    )
    .await;

    let review_req = test::TestRequest::get()
        .uri(&format!(
            "/reward-candidates/review?status={}&limit=10&offset=0",
            REWARD_STATUS_TEACHER_APPROVED
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(reviewer.id())),
        ))
        .to_request();
    let review_resp = test::call_service(&app, review_req).await;
    assert_eq!(review_resp.status(), StatusCode::OK);
    let review: Value = test::read_body_json(review_resp).await;
    let candidates = review["candidates"]
        .as_array()
        .expect("platform review candidates");
    assert!(candidates
        .iter()
        .any(|candidate_json| candidate_json["id"].as_i64() == Some(candidate.id)));
    let found = review["candidates"]
        .as_array()
        .unwrap()
        .iter()
        .find(|candidate_json| candidate_json["id"].as_i64() == Some(candidate.id))
        .unwrap();
    assert_eq!(
        found["student"]["id"].as_i64(),
        Some(i64::from(student.id()))
    );
    assert!(!found["student"]["name"].as_str().unwrap().is_empty());
    assert!(!found["student"]["email"].as_str().unwrap().is_empty());
    assert_eq!(found["course"]["id"].as_i64(), Some(i64::from(course.id)));
    assert!(!found["course"]["title"].as_str().unwrap().is_empty());
    assert_eq!(found["status"], REWARD_STATUS_TEACHER_APPROVED);
    assert_eq!(
        found["teacher_approver"]["id"].as_i64(),
        Some(i64::from(teacher.id()))
    );
    assert_eq!(
        found["teacher_decision_reason"].as_str(),
        Some("teacher approved for platform review")
    );
    assert_eq!(
        found["submitter"]["id"].as_i64(),
        Some(i64::from(teacher.id()))
    );
    assert_eq!(
        review["operator_permissions"]["can_view_candidates"].as_bool(),
        Some(true)
    );
    assert_eq!(
        review["operator_permissions"]["can_approve_amount"].as_bool(),
        Some(true)
    );

    let denied_req = test::TestRequest::get()
        .uri("/reward-candidates/review")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(student.id())),
        ))
        .to_request();
    let denied_resp = test::call_service(&app, denied_req).await;
    assert_eq!(denied_resp.status(), StatusCode::FORBIDDEN);
    let body: Value = serde_json::from_slice(&to_bytes(denied_resp.into_body()).await.unwrap())
        .expect("permission error should be json");
    assert_eq!(body["error"]["code"], "permission_denied");
    assert_eq!(
        body["error"]["message"],
        "User does not have reward candidate permission"
    );
}
