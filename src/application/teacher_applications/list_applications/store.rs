use futures::future::BoxFuture;

use crate::application::access_control::check_permission::AccessDecisionStore;
use crate::application::teacher_applications::{
    list_applications::TeacherApplicationListError, TeacherApplicationOutput,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TeacherApplicationListFilter {
    pub status: Option<String>,
    pub applicant_user_id: Option<i32>,
    pub organization_sponsor_id: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub trait TeacherApplicationListStore:
    AccessDecisionStore<Error = TeacherApplicationListError>
{
    fn list_applications(
        &mut self,
        filter: TeacherApplicationListFilter,
    ) -> BoxFuture<'_, Result<Vec<TeacherApplicationOutput>, TeacherApplicationListError>>;
}
