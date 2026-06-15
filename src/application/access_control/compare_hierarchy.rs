use futures::future::BoxFuture;
use std::cmp::Ordering;
use std::sync::Arc;

pub type HierarchyCheckService = Arc<dyn HierarchyCheckUseCase + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HierarchyScope {
    Platform,
    Course { course_id: i32 },
    Organization { organization_id: i32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HierarchyCheckError {
    Connection(String),
    Query(String),
}

pub trait HierarchyCheckUseCase {
    fn compare_users(
        &self,
        scope: HierarchyScope,
        first_user_id: i32,
        second_user_id: i32,
    ) -> BoxFuture<'_, Result<Ordering, HierarchyCheckError>>;
}
