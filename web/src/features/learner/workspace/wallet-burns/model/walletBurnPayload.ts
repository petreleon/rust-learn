import { type WalletBurnDraft, type WalletBurnSource } from "./walletBurnTypes";

export function walletBurnDraftIsReady(draft: WalletBurnDraft) {
  if (Number(draft.amount) <= 0) return false;
  if (draft.source === "centralized_wallet") return true;
  if (!draft.ethereumAddress.trim()) return false;
  if (draft.source === "decentralized_direct") return Boolean(draft.transactionHash.trim());
  return Boolean(draft.depositIntentId.trim() || draft.transactionHash.trim());
}

export function walletBurnFeePath(source: WalletBurnSource) {
  if (source === "decentralized_direct") return "network_fee_paid_by_user";
  if (source === "decentralized_platform_mediated") return "platform_deposit_fee";
  return "none";
}

export function walletBurnPayload(draft: WalletBurnDraft, idempotencyKey: string) {
  return {
    amount: draft.amount.trim(),
    chain_id: optionalNumber(draft.chainId),
    contract_address: optionalText(draft.contractAddress),
    deposit_intent_id: optionalNumber(draft.depositIntentId),
    ethereum_address: optionalText(draft.ethereumAddress),
    fee_path: walletBurnFeePath(draft.source),
    idempotency_key: idempotencyKey,
    leaderboard_visible: draft.leaderboardVisible,
    log_index: optionalNumber(draft.logIndex),
    source: draft.source,
    transaction_hash: optionalText(draft.transactionHash),
  };
}

function optionalText(value: string) {
  const trimmed = value.trim();
  return trimmed ? trimmed : undefined;
}

function optionalNumber(value: string) {
  const trimmed = value.trim();
  if (!trimmed) return undefined;
  return Number(trimmed);
}
