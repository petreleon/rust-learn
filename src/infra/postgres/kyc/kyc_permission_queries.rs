use diesel_async::AsyncPgConnection;

use crate::application::kyc::KycError;
use crate::config::constants::permissions::Permissions;
use crate::infra::postgres::access_control::permission_checks;
use crate::infra::postgres::kyc::kyc_mappers::map_error;

pub(super) async fn can_review_kyc(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<bool, KycError> {
    let permission = Permissions::REVIEW_KYC_SUBMISSIONS.to_string();
    permission_checks::can_platform_permission(conn, user_id, &permission)
        .await
        .map_err(map_error)
}
