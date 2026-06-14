use std::sync::Arc;

use actix_web::web;

use crate::application::identity::current_session::CurrentSessionUseCase;
use crate::application::identity::list_users::UserListUseCase;
use crate::db::DbPool;
use crate::infra::postgres::identity::current_session_use_case::PostgresCurrentSessionUseCase;
use crate::infra::postgres::identity::user_list_use_case::PostgresUserListUseCase;

#[derive(Clone)]
pub struct IdentityUseCases {
    pub current_session: Arc<dyn CurrentSessionUseCase>,
    pub user_list: Arc<dyn UserListUseCase>,
}

pub fn build_identity_use_cases(pool: &DbPool) -> IdentityUseCases {
    IdentityUseCases {
        current_session: Arc::new(PostgresCurrentSessionUseCase::new(pool.clone())),
        user_list: Arc::new(PostgresUserListUseCase::new(pool.clone())),
    }
}

pub fn configure_identity_app_data(cfg: &mut web::ServiceConfig, identity: &IdentityUseCases) {
    cfg.app_data(web::Data::new(identity.current_session.clone()))
        .app_data(web::Data::new(identity.user_list.clone()));
}
