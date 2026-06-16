import { type WalletTransferResult } from "./walletTransferTypes";

export function walletTransferTitle(result: WalletTransferResult) {
  return result.operation === "deposit" ? "Deposit intent created" : "Retirement created";
}

export function walletTransferDelta(result: WalletTransferResult) {
  return result.operation === "deposit"
    ? result.wallet_delta_on_confirmation
    : result.wallet_delta;
}

export function walletTransferStatus(result: WalletTransferResult) {
  if (result.operation === "deposit") return result.status;
  return "created";
}

export function walletTransferActionCopy(result: WalletTransferResult) {
  if (result.metamask_required) {
    return "External wallet action required before chain confirmation.";
  }

  return "RustLearn can complete this transfer through the platform wallet flow.";
}
