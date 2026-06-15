use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};
use std::cmp::Ordering;

use crate::application::access_control::compare_hierarchy::{
    HierarchyCheckError, HierarchyCheckUseCase, HierarchyScope,
};
use crate::domain::access_control::hierarchy::compare_hierarchy_levels;
use crate::infra::postgres::access_control::hierarchy_records;
use crate::infra::postgres::DbPool;

impl HierarchyCheckUseCase for DbPool {
    fn compare_users(
        &self,
        scope: HierarchyScope,
        first_user_id: i32,
        second_user_id: i32,
    ) -> BoxFuture<'_, Result<Ordering, HierarchyCheckError>> {
        async move {
            let mut conn = self
                .get()
                .await
                .map_err(|error| HierarchyCheckError::Connection(error.to_string()))?;

            compare_users_in_scope(&mut conn, scope, first_user_id, second_user_id)
                .await
                .map_err(|error| HierarchyCheckError::Query(error.to_string()))
        }
        .boxed()
    }
}

async fn compare_users_in_scope(
    conn: &mut AsyncPgConnection,
    scope: HierarchyScope,
    first_user_id: i32,
    second_user_id: i32,
) -> QueryResult<Ordering> {
    let first_level = hierarchy_level_for_user(conn, scope, first_user_id).await?;
    let second_level = hierarchy_level_for_user(conn, scope, second_user_id).await?;

    Ok(compare_hierarchy_levels(first_level, second_level))
}

async fn hierarchy_level_for_user(
    conn: &mut AsyncPgConnection,
    scope: HierarchyScope,
    user_id: i32,
) -> QueryResult<Option<i32>> {
    match scope {
        HierarchyScope::Platform => {
            hierarchy_records::platform_min_level_for_user(conn, user_id).await
        }
        HierarchyScope::Course { course_id } => {
            hierarchy_records::course_min_level_for_user(conn, user_id, course_id).await
        }
        HierarchyScope::Organization { organization_id } => {
            hierarchy_records::organization_min_level_for_user(conn, user_id, organization_id).await
        }
    }
}
