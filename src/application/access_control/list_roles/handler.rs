use crate::application::access_control::ports::RoleCatalogStore;
use crate::application::access_control::role_catalog::{
    RoleCatalogEntry, RoleCatalogError, RoleCatalogScope,
};

pub async fn list_platform_roles(
    store: &mut impl RoleCatalogStore,
) -> Result<Vec<RoleCatalogEntry>, RoleCatalogError> {
    list_roles(store, RoleCatalogScope::Platform).await
}

pub async fn list_organization_roles(
    store: &mut impl RoleCatalogStore,
) -> Result<Vec<RoleCatalogEntry>, RoleCatalogError> {
    list_roles(store, RoleCatalogScope::Organization).await
}

pub async fn list_course_roles(
    store: &mut impl RoleCatalogStore,
) -> Result<Vec<RoleCatalogEntry>, RoleCatalogError> {
    list_roles(store, RoleCatalogScope::Course).await
}

pub async fn list_roles(
    store: &mut impl RoleCatalogStore,
    scope: RoleCatalogScope,
) -> Result<Vec<RoleCatalogEntry>, RoleCatalogError> {
    store.list_roles(scope).await
}

#[cfg(test)]
mod tests {
    use super::list_platform_roles;
    use crate::application::access_control::ports::RoleCatalogStore;
    use crate::application::access_control::role_catalog::{
        RoleCatalogEntry, RoleCatalogError, RoleCatalogScope,
    };
    use futures::future::{ready, BoxFuture, FutureExt};

    struct RecordingStore {
        requested_scope: Option<RoleCatalogScope>,
    }

    impl RoleCatalogStore for RecordingStore {
        fn list_roles(
            &mut self,
            scope: RoleCatalogScope,
        ) -> BoxFuture<'_, Result<Vec<RoleCatalogEntry>, RoleCatalogError>> {
            self.requested_scope = Some(scope);
            ready(Ok(vec![RoleCatalogEntry {
                id: 1,
                name: "SUPER_ADMIN".to_string(),
                description: Some("Full platform access".to_string()),
            }]))
            .boxed()
        }
    }

    #[test]
    fn platform_helper_requests_platform_roles() {
        futures::executor::block_on(async {
            let mut store = RecordingStore {
                requested_scope: None,
            };

            let roles = list_platform_roles(&mut store).await.unwrap();

            assert_eq!(store.requested_scope, Some(RoleCatalogScope::Platform));
            assert_eq!(roles[0].name, "SUPER_ADMIN");
        });
    }
}
