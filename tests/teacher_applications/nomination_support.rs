use crate::support::*;

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct OrganizationTeacherNominationRequest {
    pub(crate) applicant_user_id: i32,
    pub(crate) requested_scope: Option<String>,
    pub(crate) requested_course_id: Option<i32>,
    pub(crate) experience_summary: String,
    pub(crate) portfolio_links: Option<Vec<String>>,
    pub(crate) idempotency_key: Option<String>,
}

pub(crate) async fn nominate_application(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    request: OrganizationTeacherNominationRequest,
) -> Result<TeacherApplicationOutput, TeacherApplicationError> {
    let use_case = rust_learn::infra::postgres::teacher_applications::teacher_application_nomination_use_case::PostgresTeacherApplicationNominationUseCase::new(
        establish_connection(),
    );
    rust_learn::application::teacher_applications::nominate_application::TeacherApplicationNominationUseCase::nominate_application(
        &use_case,
        rust_learn::application::teacher_applications::nominate_application::TeacherApplicationNominationCommand {
            actor_user_id,
            applicant_user_id: request.applicant_user_id,
            experience_summary: request.experience_summary,
            idempotency_key: request.idempotency_key,
            organization_id,
            portfolio_links: request.portfolio_links,
            requested_course_id: request.requested_course_id,
            requested_scope: request.requested_scope,
        },
    )
    .await
    .map_err(map_nomination_error)
}

#[allow(dead_code)]
pub(crate) fn teacher_application_nomination_data() -> web::Data<
    Arc<
        dyn rust_learn::application::teacher_applications::nominate_application::TeacherApplicationNominationUseCase,
    >,
>{
    web::Data::new(Arc::new(
        rust_learn::infra::postgres::teacher_applications::teacher_application_nomination_use_case::PostgresTeacherApplicationNominationUseCase::new(
            establish_connection(),
        ),
    )
        as Arc<
            dyn rust_learn::application::teacher_applications::nominate_application::TeacherApplicationNominationUseCase,
        >)
}

fn map_nomination_error(
    error: rust_learn::application::teacher_applications::nominate_application::TeacherApplicationNominationError,
) -> TeacherApplicationError {
    match error {
        rust_learn::application::teacher_applications::nominate_application::TeacherApplicationNominationError::PermissionDenied(permission) => {
            TeacherApplicationError::PermissionDenied(permission)
        }
        rust_learn::application::teacher_applications::nominate_application::TeacherApplicationNominationError::InvalidInput(message) => {
            TeacherApplicationError::InvalidInput(message)
        }
        rust_learn::application::teacher_applications::nominate_application::TeacherApplicationNominationError::InvalidTransition(message) => {
            TeacherApplicationError::InvalidTransition(message)
        }
        rust_learn::application::teacher_applications::nominate_application::TeacherApplicationNominationError::NotFound => {
            TeacherApplicationError::NotFound
        }
        rust_learn::application::teacher_applications::nominate_application::TeacherApplicationNominationError::Connection(message)
        | rust_learn::application::teacher_applications::nominate_application::TeacherApplicationNominationError::Database(message) => {
            TeacherApplicationError::Database(message)
        }
    }
}
