use std::sync::Arc;

use crate::application::teacher_applications::get_my_application::TeacherApplicationSelfUseCase;
use crate::db::DbPool;
use crate::infra::postgres::teacher_applications::teacher_application_self_use_case::PostgresTeacherApplicationSelfUseCase;

#[derive(Clone)]
pub struct TeacherApplicationUseCases {
    pub self_status: Arc<dyn TeacherApplicationSelfUseCase>,
}

pub fn build_teacher_application_use_cases(pool: &DbPool) -> TeacherApplicationUseCases {
    TeacherApplicationUseCases {
        self_status: Arc::new(PostgresTeacherApplicationSelfUseCase::new(pool.clone())),
    }
}
