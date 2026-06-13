use std::sync::Arc;

use crate::application::organizations::list_organization_courses::OrganizationCourseListUseCase;
use crate::application::organizations::list_organization_members::OrganizationMemberListUseCase;
use crate::db::DbPool;
use crate::infra::postgres::organizations::organization_course_list_use_case::PostgresOrganizationCourseListUseCase;
use crate::infra::postgres::organizations::organization_member_list_use_case::PostgresOrganizationMemberListUseCase;

pub struct OrganizationUseCases {
    pub course_list: Arc<dyn OrganizationCourseListUseCase>,
    pub member_list: Arc<dyn OrganizationMemberListUseCase>,
}

pub fn build_organization_use_cases(pool: &DbPool) -> OrganizationUseCases {
    OrganizationUseCases {
        course_list: Arc::new(PostgresOrganizationCourseListUseCase::new(pool.clone())),
        member_list: Arc::new(PostgresOrganizationMemberListUseCase::new(pool.clone())),
    }
}
