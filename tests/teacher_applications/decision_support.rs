use crate::support::*;

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct TeacherApplicationDecisionRequest {
    pub(crate) status: String,
    pub(crate) decision_reason: Option<String>,
}

pub(crate) async fn decide_application(
    _conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    application_id: i64,
    request: TeacherApplicationDecisionRequest,
) -> Result<TeacherApplicationOutput, TeacherApplicationError> {
    let use_case = rust_learn::infra::postgres::teacher_applications::teacher_application_decision_use_case::PostgresTeacherApplicationDecisionUseCase::new(
        establish_connection(),
    );
    rust_learn::application::teacher_applications::decide_application::TeacherApplicationDecisionUseCase::decide_application(
        &use_case,
        rust_learn::application::teacher_applications::decide_application::TeacherApplicationDecisionCommand {
            actor_user_id,
            application_id,
            decision_reason: request.decision_reason,
            status: request.status,
        },
    )
    .await
    .map_err(map_decision_error)
}

#[allow(dead_code)]
pub(crate) fn teacher_application_decision_data() -> web::Data<
    Arc<
        dyn rust_learn::application::teacher_applications::decide_application::TeacherApplicationDecisionUseCase,
    >,
>{
    web::Data::new(Arc::new(
        rust_learn::infra::postgres::teacher_applications::teacher_application_decision_use_case::PostgresTeacherApplicationDecisionUseCase::new(
            establish_connection(),
        ),
    )
        as Arc<
            dyn rust_learn::application::teacher_applications::decide_application::TeacherApplicationDecisionUseCase,
        >)
}

fn map_decision_error(
    error: rust_learn::application::teacher_applications::decide_application::TeacherApplicationDecisionError,
) -> TeacherApplicationError {
    match error {
        rust_learn::application::teacher_applications::decide_application::TeacherApplicationDecisionError::PermissionDenied(permission) => {
            TeacherApplicationError::PermissionDenied(permission)
        }
        rust_learn::application::teacher_applications::decide_application::TeacherApplicationDecisionError::InvalidInput(message) => {
            TeacherApplicationError::InvalidInput(message)
        }
        rust_learn::application::teacher_applications::decide_application::TeacherApplicationDecisionError::InvalidTransition(message) => {
            TeacherApplicationError::InvalidTransition(message)
        }
        rust_learn::application::teacher_applications::decide_application::TeacherApplicationDecisionError::NotFound => {
            TeacherApplicationError::NotFound
        }
        rust_learn::application::teacher_applications::decide_application::TeacherApplicationDecisionError::Connection(message)
        | rust_learn::application::teacher_applications::decide_application::TeacherApplicationDecisionError::Database(message) => {
            TeacherApplicationError::Database(message)
        }
    }
}
