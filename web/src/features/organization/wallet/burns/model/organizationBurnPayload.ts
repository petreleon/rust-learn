import {
  type OrganizationBurnDraft,
  type OrganizationBurnSource,
} from "./organizationBurnTypes";

export function organizationBurnDraftIsReady(draft: OrganizationBurnDraft) {
  if (Number(draft.amount) <= 0) return false;
  if (draft.source === "centralized_wallet") return true;
  if (!draft.ethereumAddress.trim()) return false;
  if (draft.source === "decentralized_direct") return Boolean(draft.transactionHash.trim());
  return Boolean(draft.depositIntentId.trim() || draft.transactionHash.trim());
}

export function organizationBurnFeePath(source: OrganizationBurnSource) {
  if (source === "decentralized_direct") return "network_fee_paid_by_user";
  if (source === "decentralized_platform_mediated") return "platform_deposit_fee";
  return "none";
}

export function organizationBurnPayload(draft: OrganizationBurnDraft, idempotencyKey: string) {
  return {
    amount: draft.amount.trim(),
    chain_id: optionalNumber(draft.chainId),
    contract_address: optionalText(draft.contractAddress),
    deposit_intent_id: optionalNumber(draft.depositIntentId),
    ethereum_address: optionalText(draft.ethereumAddress),
    fee_path: organizationBurnFeePath(draft.source),
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
