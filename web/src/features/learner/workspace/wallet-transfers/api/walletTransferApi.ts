import { DEFAULT_TIMEOUT_MS } from "@/lib/learner/DEFAULT_TIMEOUT_MS";
import { learnerJsonRequest } from "@/lib/learner/learnerJsonRequest";
import {
  type WalletDepositIntentResult,
  type WalletRetirementResult,
  type WalletTransferDraft,
  type WalletTransferOperation,
  type WalletTransferResult,
} from "../model/walletTransferTypes";
import {
  walletTransferEndpoint,
  walletTransferPayload,
} from "../model/walletTransferPayload";

export async function createWalletTransfer({
  apiRoot = "/api",
  draft,
  operation,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: {
  apiRoot?: string;
  draft: WalletTransferDraft;
  operation: WalletTransferOperation;
  timeoutMs?: number;
  token: string;
}): Promise<WalletTransferResult> {
  return learnerJsonRequest<WalletDepositIntentResult | WalletRetirementResult>({
    body: JSON.stringify(walletTransferPayload(draft)),
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}${walletTransferEndpoint(operation)}`,
  });
}
