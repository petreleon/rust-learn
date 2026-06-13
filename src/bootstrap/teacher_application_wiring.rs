use crate::application::teacher_applications::list_application_audit::TeacherApplicationAuditUseCase;
use std::sync::Arc;

use crate::application::teacher_applications::get_my_application::TeacherApplicationSelfUseCase;
use crate::db::DbPool;
use crate::infra::postgres::teacher_applications::teacher_application_audit_use_case::PostgresTeacherApplicationAuditUseCase;
use crate::infra::postgres::teacher_applications::teacher_application_self_use_case::PostgresTeacherApplicationSelfUseCase;

#[derive(Clone)]
pub struct TeacherApplicationUseCases {
    pub audit: Arc<dyn TeacherApplicationAuditUseCase>,
    pub self_status: Arc<dyn TeacherApplicationSelfUseCase>,
}

pub fn build_teacher_application_use_cases(pool: &DbPool) -> TeacherApplicationUseCases {
    TeacherApplicationUseCases {
        audit: Arc::new(PostgresTeacherApplicationAuditUseCase::new(pool.clone())),
        self_status: Arc::new(PostgresTeacherApplicationSelfUseCase::new(pool.clone())),
    }
}
