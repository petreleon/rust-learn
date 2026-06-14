use super::support::{
    WalletTokenGasPayer, WalletTokenOperation, TOKEN_TRANSFER_ACTION_METAMASK_PERMIT_SIGNATURE,
    TOKEN_TRANSFER_ACTION_METAMASK_PRESIGNED_TRANSFER, TOKEN_TRANSFER_ACTION_METAMASK_TRANSFER,
    TOKEN_TRANSFER_ACTION_PLATFORM_TRANSFER, TOKEN_TRANSFER_WALLET_PROVIDER_METAMASK,
    TOKEN_TRANSFER_WALLET_PROVIDER_PLATFORM,
};

pub(super) fn normalize_address(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

pub(super) fn addresses_equal(left: &str, right: &str) -> bool {
    normalize_address(left) == normalize_address(right)
}

#[derive(Debug, Clone, Copy)]
pub(super) struct WalletTransferInteraction {
    pub(super) provider: &'static str,
    pub(super) metamask_required: bool,
    pub(super) action: &'static str,
}

pub(super) fn wallet_interaction_for_transfer(
    operation: WalletTokenOperation,
    gas_payer: WalletTokenGasPayer,
) -> WalletTransferInteraction {
    match (operation, gas_payer) {
        (WalletTokenOperation::Deposit, WalletTokenGasPayer::User) => WalletTransferInteraction {
            provider: TOKEN_TRANSFER_WALLET_PROVIDER_METAMASK,
            metamask_required: true,
            action: TOKEN_TRANSFER_ACTION_METAMASK_TRANSFER,
        },
        (WalletTokenOperation::Deposit, WalletTokenGasPayer::Platform) => {
            WalletTransferInteraction {
                provider: TOKEN_TRANSFER_WALLET_PROVIDER_METAMASK,
                metamask_required: true,
                action: TOKEN_TRANSFER_ACTION_METAMASK_PERMIT_SIGNATURE,
            }
        }
        (WalletTokenOperation::Retire, WalletTokenGasPayer::User) => WalletTransferInteraction {
            provider: TOKEN_TRANSFER_WALLET_PROVIDER_METAMASK,
            metamask_required: true,
            action: TOKEN_TRANSFER_ACTION_METAMASK_PRESIGNED_TRANSFER,
        },
        (WalletTokenOperation::Retire, WalletTokenGasPayer::Platform) => {
            WalletTransferInteraction {
                provider: TOKEN_TRANSFER_WALLET_PROVIDER_PLATFORM,
                metamask_required: false,
                action: TOKEN_TRANSFER_ACTION_PLATFORM_TRANSFER,
            }
        }
    }
}
