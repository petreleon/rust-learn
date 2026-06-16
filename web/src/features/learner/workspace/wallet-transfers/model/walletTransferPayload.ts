import {
  type WalletTransferDraft,
  type WalletTransferOperation,
} from "./walletTransferTypes";

export function walletTransferPayload(draft: WalletTransferDraft) {
  return {
    amount: draft.amount.trim(),
    chain_id: optionalNumber(draft.chainId),
    contract_address: optionalText(draft.contractAddress),
    ethereum_address: draft.ethereumAddress.trim(),
    gas_payer: draft.gasPayer,
    log_index: optionalNumber(draft.logIndex),
    platform_address: optionalText(draft.platformAddress),
    transaction_hash: optionalText(draft.transactionHash),
  };
}

export function walletTransferDraftIsReady(draft: WalletTransferDraft) {
  return Number(draft.amount) > 0 && draft.ethereumAddress.trim().length > 0;
}

export function walletTransferEndpoint(operation: WalletTransferOperation) {
  return operation === "deposit" ? "/wallets/me/deposits" : "/wallets/me/retirements";
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
