#[derive(Debug, Clone, serde::Serialize)]
struct SubmitTeacherApplicationRequest {
    requested_scope: String,
    requested_organization_id: Option<i32>,
    requested_course_id: Option<i32>,
    experience_summary: String,
    organization_sponsor_id: Option<i32>,
    portfolio_links: Option<Vec<String>>,
    idempotency_key: Option<String>,
}

async fn submit_application(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: SubmitTeacherApplicationRequest,
) -> Result<TeacherApplicationOutput, TeacherApplicationError> {
    let use_case = rust_learn::infra::postgres::teacher_applications::teacher_application_submit_use_case::PostgresTeacherApplicationSubmitUseCase::new(
        establish_connection(),
    );
    rust_learn::application::teacher_applications::submit_application::TeacherApplicationSubmitUseCase::submit_application(
        &use_case,
        rust_learn::application::teacher_applications::submit_application::TeacherApplicationSubmitCommand {
            actor_user_id,
            experience_summary: request.experience_summary,
            idempotency_key: request.idempotency_key,
            organization_sponsor_id: request.organization_sponsor_id,
            portfolio_links: request.portfolio_links,
            requested_course_id: request.requested_course_id,
            requested_organization_id: request.requested_organization_id,
            requested_scope: request.requested_scope,
        },
    )
    .await
    .map_err(map_submit_error)
}

fn teacher_application_submit_data() -> web::Data<
    Arc<
        dyn rust_learn::application::teacher_applications::submit_application::TeacherApplicationSubmitUseCase,
    >,
> {
    web::Data::new(Arc::new(
        rust_learn::infra::postgres::teacher_applications::teacher_application_submit_use_case::PostgresTeacherApplicationSubmitUseCase::new(
            establish_connection(),
        ),
    )
        as Arc<
            dyn rust_learn::application::teacher_applications::submit_application::TeacherApplicationSubmitUseCase,
        >)
}

fn map_submit_error(
    error: rust_learn::application::teacher_applications::submit_application::TeacherApplicationSubmitError,
) -> TeacherApplicationError {
    match error {
        rust_learn::application::teacher_applications::submit_application::TeacherApplicationSubmitError::PermissionDenied(permission) => {
            TeacherApplicationError::PermissionDenied(permission)
        }
        rust_learn::application::teacher_applications::submit_application::TeacherApplicationSubmitError::InvalidInput(message) => {
            TeacherApplicationError::InvalidInput(message)
        }
        rust_learn::application::teacher_applications::submit_application::TeacherApplicationSubmitError::InvalidTransition(message) => {
            TeacherApplicationError::InvalidTransition(message)
        }
        rust_learn::application::teacher_applications::submit_application::TeacherApplicationSubmitError::Connection(message)
        | rust_learn::application::teacher_applications::submit_application::TeacherApplicationSubmitError::Database(message) => {
            TeacherApplicationError::Database(message)
        }
    }
}
