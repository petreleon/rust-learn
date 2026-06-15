use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::rewards::notify_wallet_credit::RewardWalletCreditNotificationError;
use crate::domain::rewards::wallet_credit::reward_wallet_credit_notification_message;
use crate::infra::postgres::models::notification::NewNotification;
use crate::infra::postgres::rewards::reward_wallet_credit_notification_mappers::map_diesel_error;
use crate::infra::postgres::schema::{courses, notifications};

pub(super) async fn create_wallet_credit_notification(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    amount: &BigDecimal,
    wallet_id: i32,
    transaction_id: i64,
) -> Result<i64, RewardWalletCreditNotificationError> {
    let course_title = courses::table
        .find(course_id)
        .select(courses::title)
        .first::<String>(conn)
        .await
        .map_err(map_diesel_error)?;
    let message = reward_wallet_credit_notification_message(
        course_id,
        course_title,
        amount.to_string(),
        wallet_id,
        transaction_id,
    );

    diesel::insert_into(notifications::table)
        .values(NewNotification {
            user_id: Some(user_id),
            title: message.title,
            body: &message.body,
        })
        .returning(notifications::id)
        .get_result(conn)
        .await
        .map_err(map_diesel_error)
}
