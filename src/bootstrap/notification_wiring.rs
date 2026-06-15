use std::sync::Arc;

use actix_web::web;

use crate::application::notifications::delivery::NotificationDeliveryUseCase;
use crate::application::notifications::notification_inbox::NotificationInboxUseCase;
use crate::application::notifications::preference_service::NotificationPreferencesUseCase;
use crate::infra::notifications::NotificationsState;
use crate::infra::postgres::notifications::notification_inbox_use_case::PostgresNotificationInboxUseCase;
use crate::infra::postgres::notifications::notification_preferences_use_case::PostgresNotificationPreferencesUseCase;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct NotificationUseCases {
    pub delivery: Arc<dyn NotificationDeliveryUseCase>,
    pub inbox: Arc<dyn NotificationInboxUseCase>,
    pub preferences: Arc<dyn NotificationPreferencesUseCase>,
}

pub fn build_notification_use_cases(pool: &DbPool) -> NotificationUseCases {
    NotificationUseCases {
        delivery: Arc::new(NotificationsState::new(pool.clone())),
        inbox: Arc::new(PostgresNotificationInboxUseCase::new(pool.clone())),
        preferences: Arc::new(PostgresNotificationPreferencesUseCase::new(pool.clone())),
    }
}

pub fn configure_notification_app_data(
    cfg: &mut web::ServiceConfig,
    use_cases: &NotificationUseCases,
) {
    cfg.app_data(web::Data::new(use_cases.delivery.clone()))
        .app_data(web::Data::new(use_cases.inbox.clone()))
        .app_data(web::Data::new(use_cases.preferences.clone()));
}
