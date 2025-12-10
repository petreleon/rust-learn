use actix_service::{forward_ready, Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error, web, HttpMessage};
use futures::future::{self, Ready, LocalBoxFuture};
use futures::FutureExt;
use std::marker::PhantomData;
use std::rc::Rc;
use diesel::PgConnection;

use crate::db::DbPool;
use crate::models::user_jwt::UserJWT;

/// A strategy trait to encapsulate the specific logic for extracting data from the request
/// and checking permissions against the database.
pub trait PermissionCheckStrategy: 'static {
    /// The data needed to perform the permission check (e.g., extracted ID, permission name).
    type ExtractedData: Clone + 'static;

    /// Extract necessary data from the request (e.g., resource ID from path).
    /// This happens synchronously before the future is spawned.
    fn extract(&self, req: &ServiceRequest) -> Result<Self::ExtractedData, Error>;

    /// Perform the actual permission check using the database connection and the authenticated user ID.
    fn check(
        &self,
        conn: &mut PgConnection,
        user_id: i32,
        data: Self::ExtractedData,
    ) -> Result<(), Error>;
}

/// A generic middleware that handles the boilerplate of:
/// 1. Extracting the DB pool.
/// 2. extracting UserJWT.
/// 3. Delegating data extraction and key checks to the `Strategy`.
pub struct PermissionMiddleware<S, Strategy> {
    service: PhantomData<S>,
    strategy: Rc<Strategy>,
}

impl<S, Strategy> PermissionMiddleware<S, Strategy>
where
    Strategy: PermissionCheckStrategy,
{
    pub fn from_strategy(strategy: Strategy) -> Self {
        PermissionMiddleware {
            service: PhantomData,
            strategy: Rc::new(strategy),
        }
    }
}

impl<S, B, Strategy> Transform<S, ServiceRequest> for PermissionMiddleware<S, Strategy>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
    Strategy: PermissionCheckStrategy,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PermissionMiddlewareService<S, Strategy>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(PermissionMiddlewareService {
            service,
            strategy: self.strategy.clone(),
        }))
    }
}

pub struct PermissionMiddlewareService<S, Strategy> {
    service: S,
    strategy: Rc<Strategy>,
}

impl<S, B, Strategy> Service<ServiceRequest> for PermissionMiddlewareService<S, Strategy>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
    Strategy: PermissionCheckStrategy,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let strategy = self.strategy.clone();

        // 1. Get DB Pool
        let db_pool = match req.app_data::<web::Data<DbPool>>() {
            Some(pool) => pool.clone(),
            None => {
                let error = actix_web::error::ErrorInternalServerError("Failed to access database pool");
                return future::ready(Err(error)).boxed_local();
            },
        };

        // 2. Extract Data (Strategy-specific)
        let extracted_data_result = strategy.extract(&req);
        let extracted_data = match extracted_data_result {
            Ok(data) => data,
            Err(e) => return future::ready(Err(e)).boxed_local(),
        };

        // 3. Get UserJWT
        let user_jwt_opt = req.extensions().get::<UserJWT>().cloned();

        let fut = self.service.call(req);

        async move {
            let mut conn = match db_pool.get() {
                Ok(conn) => conn,
                Err(_) => return Err(actix_web::error::ErrorInternalServerError("Failed to get database connection")),
            };

            let user_jwt = match user_jwt_opt {
                Some(u) => u,
                None => return Err(actix_web::error::ErrorUnauthorized("Unauthorized access")),
            };

            // 4. Check Permission (Strategy-specific)
            strategy.check(&mut conn, user_jwt.user_id, extracted_data)?;

            fut.await
        }.boxed_local()
    }
}
