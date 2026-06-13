use futures::future::BoxFuture;

use crate::application::teacher_applications::{
    list_applications::{TeacherApplicationListError, TeacherApplicationListQuery},
    TeacherApplicationOutput,
};

pub trait TeacherApplicationListUseCase: Send + Sync {
    fn list_applications(
        &self,
        query: TeacherApplicationListQuery,
    ) -> BoxFuture<'_, Result<Vec<TeacherApplicationOutput>, TeacherApplicationListError>>;
}
