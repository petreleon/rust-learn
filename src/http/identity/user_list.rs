use std::sync::Arc;

use actix_web::web;

use crate::application::identity::list_users::UserListUseCase;
use crate::http::errors::ApiError;
use crate::http::identity::dto::{ListUsersRequest, UsersResponse};
use crate::http::identity::errors::list_users_error;

pub(super) async fn list_users(
    use_case: web::Data<Arc<dyn UserListUseCase>>,
    query: web::Query<ListUsersRequest>,
) -> Result<web::Json<UsersResponse>, ApiError> {
    use_case
        .list_users(query.into_inner().into())
        .await
        .map(UsersResponse::from)
        .map(web::Json)
        .map_err(list_users_error)
}
