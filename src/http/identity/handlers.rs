use std::sync::Arc;

use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpMessage, HttpRequest};
use futures::future::{ready, Ready};

use crate::application::identity::current_session::CurrentSessionUseCase;
use crate::domain::identity::UserJWT;
use crate::http::errors::ApiError;
use crate::http::identity::dto::CurrentSessionResponse;
use crate::http::identity::errors::{current_session_auth_error, current_session_error};

pub async fn get_current_session(
    session: web::Data<Arc<dyn CurrentSessionUseCase>>,
    user: CurrentSessionUserId,
) -> Result<web::Json<CurrentSessionResponse>, ApiError> {
    let current_user_id = user.into_inner();

    session
        .get_current_session(current_user_id)
        .await
        .map(CurrentSessionResponse::from)
        .map(web::Json)
        .map_err(|error| current_session_error(current_user_id, error))
}

pub struct CurrentSessionUserId(i32);

impl CurrentSessionUserId {
    fn into_inner(self) -> i32 {
        self.0
    }
}

impl FromRequest for CurrentSessionUserId {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(current_session_user_id(req).map_err(Into::into))
    }
}

fn current_session_user_id(req: &HttpRequest) -> Result<CurrentSessionUserId, ApiError> {
    req.extensions()
        .get::<UserJWT>()
        .map(|user| CurrentSessionUserId(user.user_id))
        .ok_or_else(current_session_auth_error)
}
