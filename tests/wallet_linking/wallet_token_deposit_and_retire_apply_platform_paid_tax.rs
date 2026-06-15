use crate::http_support::assign_platform_permission_role;
use crate::platform_paid_tax_helpers::assert_wallet_tax_permissions_and_update;
use crate::platform_paid_transfer_helpers::{
    assert_platform_paid_retirement_and_audit, create_and_credit_platform_paid_deposit,
};
use crate::support::*;

#[actix_web::test]
async fn wallet_token_deposit_and_retire_apply_platform_paid_tax() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let tax_admin = create_test_user(&mut conn, "wallet_tax_admin").await;
    let deposit_tax_only = create_test_user(&mut conn, "wallet_deposit_tax_only").await;
    let stranger = create_test_user(&mut conn, "wallet_tax_stranger").await;
    let learner = create_test_user(&mut conn, "wallet_tax_learner").await;
    mark_user_kyc_verified(&mut conn, learner.id()).await;
    assign_platform_permission_role(&mut conn, tax_admin.id(), Permissions::SET_DEPOSIT_TAX).await;
    assign_platform_permission_role(&mut conn, tax_admin.id(), Permissions::SET_RETIRE_TAX).await;
    assign_platform_permission_role(
        &mut conn,
        deposit_tax_only.id(),
        Permissions::SET_DEPOSIT_TAX,
    )
    .await;
    set_persistent_state(
        &mut conn,
        "platform_importer_address",
        "0x00000000000000000000000000000000000000bb",
    )
    .await
    .expect("failed to configure platform importer address");
    drop(conn);

    assert_wallet_tax_permissions_and_update(
        &pool,
        tax_admin.id(),
        deposit_tax_only.id(),
        stranger.id(),
        learner.id(),
    )
    .await;
    let deposit = create_and_credit_platform_paid_deposit(&pool, learner.id()).await;
    assert_platform_paid_retirement_and_audit(&pool, learner.id(), deposit).await;
}
