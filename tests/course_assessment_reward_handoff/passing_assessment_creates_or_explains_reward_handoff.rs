use crate::support::*;

#[actix_web::test]
async fn passing_assessment_creates_or_explains_reward_handoff() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let learner = create_user(&mut conn, "assessment_reward").await;
    let rewarded = create_course(&mut conn, &unique_string("RewardedCourse")).await;
    let missing = create_course(&mut conn, &unique_string("MissingPolicyCourse")).await;
    assign_student(&mut conn, learner.id(), rewarded.id).await;
    assign_student(&mut conn, learner.id(), missing.id).await;
    let policy_id = create_reward_policy(&mut conn, rewarded.id).await;
    let (rewarded_assessment, rewarded_question) = create_assessment(&mut conn, rewarded.id).await;
    let (missing_assessment, missing_question) = create_assessment(&mut conn, missing.id).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(assessment_use_case(&pool))
            .app_data(rust_learn::bootstrap::auth_token_verifier_app_data())
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;
    let token = token_for(learner.id());

    let created_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(&format!(
                "/courses/{}/assessments/{}/submit",
                rewarded.id, rewarded_assessment
            ))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(body(rewarded_question))
            .to_request(),
    )
    .await;
    assert_eq!(created_response.status(), StatusCode::OK);
    let created_json: Value = test::read_body_json(created_response).await;
    assert_eq!(created_json["reward_handoff"]["status"], "created");
    assert_eq!(created_json["reward_handoff"]["policy_id"], policy_id);
    let candidate_id = created_json["reward_handoff"]["candidate_id"]
        .as_i64()
        .expect("candidate id should be returned");

    let missing_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(&format!(
                "/courses/{}/assessments/{}/submit",
                missing.id, missing_assessment
            ))
            .insert_header((
                "Authorization",
                format!("Bearer {}", token_for(learner.id())),
            ))
            .set_json(body(missing_question))
            .to_request(),
    )
    .await;
    assert_eq!(missing_response.status(), StatusCode::OK);
    let missing_json: Value = test::read_body_json(missing_response).await;
    assert_eq!(missing_json["reward_handoff"]["status"], "missing_policy");
    assert!(missing_json["reward_handoff"]["policy_id"].is_null());

    let mut conn = setup_conn(&pool).await;
    let candidate = reward_candidates::table
        .find(candidate_id)
        .first::<RewardCandidate>(&mut conn)
        .await
        .expect("candidate should exist");
    assert_eq!(candidate.event_type, REWARD_EVENT_ASSESSMENT_COMPLETION);
    assert_eq!(candidate.course_id, rewarded.id);
    assert_eq!(candidate.student_user_id, learner.id());
    assert_eq!(candidate.evidence["assessment_id"], rewarded_assessment);
    assert_eq!(candidate.evidence["passing_score"], 100);
}
