#[derive(Debug, Clone, Default)]
struct PlatformTeacherApplicationsRequest {
    status: Option<String>,
    search: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

async fn list_platform_applications(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: PlatformTeacherApplicationsRequest,
) -> Result<
    rust_learn::application::teacher_applications::list_platform_review::TeacherApplicationPlatformReviewOutput,
    rust_learn::application::teacher_applications::list_platform_review::TeacherApplicationPlatformReviewError,
> {
    let use_case = rust_learn::infra::postgres::teacher_applications::teacher_application_platform_review_use_case::PostgresTeacherApplicationPlatformReviewUseCase::new(
        establish_connection(),
    );
    rust_learn::application::teacher_applications::list_platform_review::TeacherApplicationPlatformReviewUseCase::list_platform_review_applications(
        &use_case,
        rust_learn::application::teacher_applications::list_platform_review::TeacherApplicationPlatformReviewQuery {
            actor_user_id,
            limit: request.limit,
            offset: request.offset,
            search: request.search,
            status: request.status,
        },
    )
    .await
}

fn teacher_application_platform_review_data() -> web::Data<
    Arc<
        dyn rust_learn::application::teacher_applications::list_platform_review::TeacherApplicationPlatformReviewUseCase,
    >,
> {
    web::Data::new(Arc::new(
        rust_learn::infra::postgres::teacher_applications::teacher_application_platform_review_use_case::PostgresTeacherApplicationPlatformReviewUseCase::new(
            establish_connection(),
        ),
    )
        as Arc<
            dyn rust_learn::application::teacher_applications::list_platform_review::TeacherApplicationPlatformReviewUseCase,
        >)
}
