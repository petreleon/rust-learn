use actix_web::{http::StatusCode, test, web, App};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rust_learn::db::schema::{
    chapters, contents, course_join_requests, courses, courses_organizations, organizations,
    reward_policies, upload_jobs,
};
use rust_learn::db::{establish_connection, DbPool};
use rust_learn::models::chapter::NewChapter;
use rust_learn::models::content::NewContent;
use rust_learn::models::course::{Course, NewCourse, COURSE_STATUS_PUBLISHED};
use rust_learn::models::course_join_request::{NewCourseJoinRequest, COURSE_JOIN_STATUS_PENDING};
use rust_learn::models::courses_organizations::NewCourseOrganization;
use rust_learn::models::organization::{NewOrganization, Organization};
use rust_learn::models::reward_policy::{
    NewRewardPolicy, REWARD_PAYMENT_TREASURY_TRANSFER, REWARD_POLICY_SCOPE_COURSE,
};
use rust_learn::models::role::{CourseRole, PlatformRole};
use rust_learn::models::upload_job::NewUploadJob;
use rust_learn::models::user::User;
use rust_learn::models::user_role_course::UserRoleCourse;
use rust_learn::models::user_role_platform::UserRolePlatform;
use rust_learn::repositories::user_repository::create_user;
use rust_learn::utils::jwt_utils::create_jwt;
use serde_json::Value;
use std::sync::atomic::{AtomicU64, Ordering};

static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_string(prefix: &str) -> String {
    let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let counter = UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}_{}_{}", prefix, std::process::id(), ts, counter)
}

async fn setup_conn(
    pool: &DbPool,
) -> diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection> {
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

async fn create_test_user(conn: &mut AsyncPgConnection) -> User {
    let email = format!("{}@example.com", unique_string("course_discovery"));
    create_user(
        conn,
        "Course Discovery",
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "ValidPass123!",
    )
    .await
    .expect("failed to create user")
}

async fn assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = PlatformRole::find_by_name(role_name, conn)
        .await
        .expect("platform role should exist");
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = CourseRole::find_by_name(role_name, conn)
        .await
        .expect("course role should exist");
    UserRoleCourse::assign(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

async fn create_course(conn: &mut AsyncPgConnection, title: &str) -> Course {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title: title.to_string(),
        })
        .get_result(conn)
        .await
        .expect("failed to create course")
}

async fn publish_course(conn: &mut AsyncPgConnection, course_id: i32) {
    diesel::update(courses::table.find(course_id))
        .set(courses::lifecycle_status.eq(COURSE_STATUS_PUBLISHED))
        .execute(conn)
        .await
        .expect("failed to publish course");
}

async fn create_organization(conn: &mut AsyncPgConnection, name: &str) -> Organization {
    diesel::insert_into(organizations::table)
        .values(NewOrganization {
            name: name.to_string(),
            website_link: None,
            profile_url: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create organization")
}

async fn link_course_to_org(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    organization_id: i32,
    order: i32,
) {
    diesel::insert_into(courses_organizations::table)
        .values(NewCourseOrganization {
            course_id,
            organization_id,
            order,
        })
        .execute(conn)
        .await
        .expect("failed to link course and organization");
}

async fn create_chapter(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    title: &str,
    order: i32,
) -> i32 {
    diesel::insert_into(chapters::table)
        .values(NewChapter {
            course_id,
            title: title.to_string(),
            order,
        })
        .returning(chapters::id)
        .get_result(conn)
        .await
        .expect("failed to create chapter")
}

async fn create_content(
    conn: &mut AsyncPgConnection,
    chapter_id: i32,
    content_type: &str,
    order: i32,
) {
    create_content_with_data(conn, chapter_id, content_type, order, None).await;
}

async fn create_content_with_data(
    conn: &mut AsyncPgConnection,
    chapter_id: i32,
    content_type: &str,
    order: i32,
    data: Option<&str>,
) -> i32 {
    diesel::insert_into(contents::table)
        .values(NewContent {
            chapter_id,
            order,
            content_type: content_type.to_string(),
            data: data.map(ToString::to_string),
        })
        .returning(contents::id)
        .get_result(conn)
        .await
        .expect("failed to create content")
}

async fn create_upload_job(
    conn: &mut AsyncPgConnection,
    object_key: &str,
    status: &str,
    last_error: Option<&str>,
) {
    let job_id = diesel::insert_into(upload_jobs::table)
        .values(NewUploadJob {
            bucket: "course-materials",
            object: object_key,
            user_id: None,
        })
        .returning(upload_jobs::id)
        .get_result::<i64>(conn)
        .await
        .expect("failed to create upload job");

    diesel::update(upload_jobs::table.find(job_id))
        .set((
            upload_jobs::status.eq(status),
            upload_jobs::last_error.eq(last_error.map(ToString::to_string)),
        ))
        .execute(conn)
        .await
        .expect("failed to update upload job");
}

async fn create_course_reward_policy(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    event_type: &str,
) {
    diesel::insert_into(reward_policies::table)
        .values(NewRewardPolicy {
            scope_type: REWARD_POLICY_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course_id),
            event_type: event_type.to_string(),
            version: 1,
            token_amount: BigDecimal::from(25),
            multiplier: BigDecimal::from(1),
            max_payout: None,
            cooldown_seconds: 0,
            payment_strategy: REWARD_PAYMENT_TREASURY_TRANSFER.to_string(),
            active: true,
            created_by_user_id: None,
        })
        .execute(conn)
        .await
        .expect("failed to create reward policy");
}

async fn create_pending_join_request(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> i64 {
    diesel::insert_into(course_join_requests::table)
        .values(NewCourseJoinRequest {
            course_id,
            requester_user_id: user_id,
            status: COURSE_JOIN_STATUS_PENDING.to_string(),
        })
        .returning(course_join_requests::id)
        .get_result(conn)
        .await
        .expect("failed to create join request")
}

fn token_for(user_id: i32) -> String {
    create_jwt(user_id).expect("failed to create JWT")
}

#[actix_web::test]
async fn course_list_supports_search_org_filter_and_pagination() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let viewer = create_test_user(&mut conn).await;
    assign_platform_role(&mut conn, viewer.id(), "STUDENT").await;

    let org = create_organization(&mut conn, &unique_string("DiscoveryOrg")).await;
    let other_org = create_organization(&mut conn, &unique_string("OtherOrg")).await;
    let prefix = unique_string("DiscoveryCourse");
    let first = create_course(&mut conn, &format!("{} Rust Patterns", prefix)).await;
    let second = create_course(&mut conn, &format!("{} Rust Services", prefix)).await;
    let excluded = create_course(&mut conn, &format!("{} TypeScript", prefix)).await;
    let other_org_course = create_course(&mut conn, &format!("{} Rust External", prefix)).await;

    link_course_to_org(&mut conn, first.id, org.id, 0).await;
    link_course_to_org(&mut conn, second.id, org.id, 1).await;
    link_course_to_org(&mut conn, excluded.id, org.id, 2).await;
    link_course_to_org(&mut conn, other_org_course.id, other_org.id, 0).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/courses?search=rust&organization_id={}&limit=1&offset=1",
            org.id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(viewer.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["total"].as_i64(), Some(2));
    assert_eq!(body["limit"].as_i64(), Some(1));
    assert_eq!(body["offset"].as_i64(), Some(1));
    assert_eq!(body["search"].as_str(), Some("rust"));
    assert_eq!(body["organization_id"].as_i64(), Some(i64::from(org.id)));
    assert_eq!(body["courses"].as_array().expect("courses array").len(), 1);
    assert_eq!(
        body["courses"][0]["title"].as_str(),
        Some(second.title.as_str())
    );
}

#[actix_web::test]
async fn course_search_treats_like_wildcards_as_literal_text() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let viewer = create_test_user(&mut conn).await;
    assign_platform_role(&mut conn, viewer.id(), "STUDENT").await;

    let org = create_organization(&mut conn, &unique_string("WildcardOrg")).await;
    let prefix = unique_string("WildcardCourse").replace('_', "-");
    let percent_course = create_course(&mut conn, &format!("{} 100% Complete", prefix)).await;
    let underscore_course = create_course(&mut conn, &format!("{} data_set", prefix)).await;
    let plain_course = create_course(&mut conn, &format!("{} Plain Course", prefix)).await;

    link_course_to_org(&mut conn, percent_course.id, org.id, 0).await;
    link_course_to_org(&mut conn, underscore_course.id, org.id, 1).await;
    link_course_to_org(&mut conn, plain_course.id, org.id, 2).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope()),
    )
    .await;
    let token = token_for(viewer.id());

    let percent_req = test::TestRequest::get()
        .uri(&format!(
            "/courses?search=%25&organization_id={}&limit=10",
            org.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let percent_resp = test::call_service(&app, percent_req).await;
    assert_eq!(percent_resp.status(), StatusCode::OK);

    let percent_body: Value = test::read_body_json(percent_resp).await;
    assert_eq!(percent_body["total"].as_i64(), Some(1));
    assert_eq!(percent_body["search"].as_str(), Some("%"));
    assert_eq!(
        percent_body["courses"][0]["title"].as_str(),
        Some(percent_course.title.as_str())
    );

    let underscore_req = test::TestRequest::get()
        .uri(&format!(
            "/courses?search=_&organization_id={}&limit=10",
            org.id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let underscore_resp = test::call_service(&app, underscore_req).await;
    assert_eq!(underscore_resp.status(), StatusCode::OK);

    let underscore_body: Value = test::read_body_json(underscore_resp).await;
    assert_eq!(underscore_body["total"].as_i64(), Some(1));
    assert_eq!(underscore_body["search"].as_str(), Some("_"));
    assert_eq!(
        underscore_body["courses"][0]["title"].as_str(),
        Some(underscore_course.title.as_str())
    );
}

#[actix_web::test]
async fn learner_catalog_returns_published_course_summaries_and_pending_state() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let learner = create_test_user(&mut conn).await;
    let teacher = create_test_user(&mut conn).await;
    assign_platform_role(&mut conn, learner.id(), "USER").await;

    let org = create_organization(&mut conn, &unique_string("CatalogOrg")).await;
    let prefix = unique_string("CatalogCourse");
    let published = create_course(&mut conn, &format!("{} Published Rust", prefix)).await;
    let hidden_draft = create_course(&mut conn, &format!("{} Hidden Draft", prefix)).await;
    publish_course(&mut conn, published.id).await;
    link_course_to_org(&mut conn, published.id, org.id, 0).await;
    link_course_to_org(&mut conn, hidden_draft.id, org.id, 1).await;
    assign_course_role(&mut conn, teacher.id(), published.id, "TEACHER").await;
    let chapter_id = create_chapter(&mut conn, published.id, "Getting started", 0).await;
    create_content(&mut conn, chapter_id, "video", 0).await;
    create_course_reward_policy(&mut conn, published.id, "course_completion").await;
    let request_id = create_pending_join_request(&mut conn, learner.id(), published.id).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/courses/catalog?search={}&reward_available=true&limit=10",
            prefix
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["total"].as_i64(), Some(1));
    let course = &body["courses"][0];
    assert_eq!(course["id"].as_i64(), Some(i64::from(published.id)));
    assert_eq!(course["title"].as_str(), Some(published.title.as_str()));
    assert_eq!(
        course["organizations"][0]["name"].as_str(),
        Some(org.name.as_str())
    );
    assert_eq!(
        course["teachers"][0]["name"].as_str(),
        Some(teacher.name.as_str())
    );
    assert_eq!(course["content"]["chapter_count"].as_u64(), Some(1));
    assert_eq!(course["content"]["content_count"].as_u64(), Some(1));
    assert_eq!(
        course["content"]["content_types"][0].as_str(),
        Some("video")
    );
    assert_eq!(course["rewards"]["available"].as_bool(), Some(true));
    assert_eq!(
        course["rewards"]["event_types"][0].as_str(),
        Some("course_completion")
    );
    assert_eq!(course["enrollment"]["state"].as_str(), Some("pending"));
    assert_eq!(
        course["enrollment"]["request_id"].as_i64(),
        Some(request_id)
    );
    assert_eq!(
        course["enrollment"]["can_request_join"].as_bool(),
        Some(false)
    );
    assert_eq!(course["access"]["can_request_join"].as_bool(), Some(true));
}

#[actix_web::test]
async fn learner_catalog_includes_own_draft_but_hides_unscoped_draft_detail() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let learner = create_test_user(&mut conn).await;
    let outsider = create_test_user(&mut conn).await;
    assign_platform_role(&mut conn, learner.id(), "USER").await;
    assign_platform_role(&mut conn, outsider.id(), "USER").await;

    let draft = create_course(&mut conn, &unique_string("OwnDraftCourse")).await;
    assign_course_role(&mut conn, learner.id(), draft.id, "STUDENT").await;
    let chapter_id = create_chapter(&mut conn, draft.id, "Draft module", 0).await;
    create_content(&mut conn, chapter_id, "article", 0).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope()),
    )
    .await;

    let catalog_req = test::TestRequest::get()
        .uri("/courses/catalog?enrollment_status=enrolled&limit=10")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .to_request();
    let catalog_resp = test::call_service(&app, catalog_req).await;
    assert_eq!(catalog_resp.status(), StatusCode::OK);
    let catalog_body: Value = test::read_body_json(catalog_resp).await;
    let courses = catalog_body["courses"].as_array().expect("courses array");
    let own_draft = courses
        .iter()
        .find(|course| course["id"].as_i64() == Some(i64::from(draft.id)))
        .expect("own draft should be visible through course role");
    assert_eq!(own_draft["enrollment"]["state"].as_str(), Some("enrolled"));
    assert_eq!(
        own_draft["content"]["content_types"][0].as_str(),
        Some("article")
    );

    let detail_req = test::TestRequest::get()
        .uri(&format!("/courses/catalog/{}", draft.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .to_request();
    let detail_resp = test::call_service(&app, detail_req).await;
    assert_eq!(detail_resp.status(), StatusCode::OK);
    let detail_body: Value = test::read_body_json(detail_resp).await;
    assert_eq!(
        detail_body["course"]["id"].as_i64(),
        Some(i64::from(draft.id))
    );
    assert_eq!(
        detail_body["chapters"][0]["contents"][0]["content_type"].as_str(),
        Some("article")
    );

    let outsider_req = test::TestRequest::get()
        .uri(&format!("/courses/catalog/{}", draft.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(outsider.id())),
        ))
        .to_request();
    let outsider_resp = test::call_service(&app, outsider_req).await;
    assert_eq!(outsider_resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn learner_learning_endpoint_returns_content_states_and_denies_unscoped_content() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let learner = create_test_user(&mut conn).await;
    let outsider = create_test_user(&mut conn).await;

    let course = create_course(&mut conn, &unique_string("LearningCourse")).await;
    publish_course(&mut conn, course.id).await;
    assign_course_role(&mut conn, learner.id(), course.id, "STUDENT").await;
    let later_chapter_id = create_chapter(&mut conn, course.id, "Module two", 1).await;
    let later_content_id = create_content_with_data(
        &mut conn,
        later_chapter_id,
        "text",
        0,
        Some("Later module."),
    )
    .await;
    let chapter_id = create_chapter(&mut conn, course.id, "Module one", 0).await;
    let text_content_id = create_content_with_data(
        &mut conn,
        chapter_id,
        "text",
        0,
        Some("Welcome to ownership."),
    )
    .await;
    let object_key = format!("courses/{}/chapters/{}/video.mp4", course.id, chapter_id);
    let video_content_id =
        create_content_with_data(&mut conn, chapter_id, "video", 1, Some(&object_key)).await;
    create_upload_job(
        &mut conn,
        &object_key,
        "failed",
        Some("ffmpeg failed during thumbnail extraction"),
    )
    .await;
    let empty_video_id = create_content_with_data(&mut conn, chapter_id, "video", 2, None).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/courses/catalog/{}/learn", course.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(learner.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["course"]["id"].as_i64(), Some(i64::from(course.id)));
    assert_eq!(
        body["active_content_id"].as_i64(),
        Some(i64::from(text_content_id))
    );
    assert_eq!(body["progress_supported"].as_bool(), Some(false));

    let chapters = body["chapters"]
        .as_array()
        .expect("learning chapters array");
    assert_eq!(chapters[0]["id"].as_i64(), Some(i64::from(chapter_id)));
    assert_eq!(
        chapters[1]["id"].as_i64(),
        Some(i64::from(later_chapter_id))
    );
    assert_eq!(
        chapters[1]["contents"][0]["id"].as_i64(),
        Some(i64::from(later_content_id))
    );

    let contents = chapters[0]["contents"]
        .as_array()
        .expect("learning contents array");
    assert_eq!(contents[0]["id"].as_i64(), Some(i64::from(text_content_id)));
    assert_eq!(contents[0]["display_state"].as_str(), Some("ready"));
    assert_eq!(contents[0]["data"].as_str(), Some("Welcome to ownership."));
    assert_eq!(
        contents[1]["id"].as_i64(),
        Some(i64::from(video_content_id))
    );
    assert_eq!(
        contents[1]["display_state"].as_str(),
        Some("failed_processing")
    );
    assert_eq!(contents[1]["processing_status"].as_str(), Some("failed"));
    assert_eq!(
        contents[1]["processing_error"].as_str(),
        Some("ffmpeg failed during thumbnail extraction")
    );
    assert_eq!(contents[2]["id"].as_i64(), Some(i64::from(empty_video_id)));
    assert_eq!(
        contents[2]["display_state"].as_str(),
        Some("unprocessed_upload")
    );

    let denied_req = test::TestRequest::get()
        .uri(&format!("/courses/catalog/{}/learn", course.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(outsider.id())),
        ))
        .to_request();
    let denied_resp = test::call_service(&app, denied_req).await;
    assert_eq!(denied_resp.status(), StatusCode::FORBIDDEN);
}
