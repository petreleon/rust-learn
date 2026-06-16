import { DEFAULT_TIMEOUT_MS } from "@/lib/learner/DEFAULT_TIMEOUT_MS";
import { learnerJsonRequest } from "@/lib/learner/learnerJsonRequest";
import { type LearnerRequestOptions } from "@/lib/learner/LearnerRequestOptions";
import { walletBurnPayload } from "../model/walletBurnPayload";
import { type WalletBurnDraft, type WalletBurnResult } from "../model/walletBurnTypes";

export function fetchMyWalletBurns({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: LearnerRequestOptions): Promise<WalletBurnResult[]> {
  return learnerJsonRequest({
    timeoutMs,
    token,
    url: `${apiRoot}/wallets/me/burns`,
  });
}

export function createMyWalletBurn({
  apiRoot = "/api",
  draft,
  idempotencyKey,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: LearnerRequestOptions & {
  draft: WalletBurnDraft;
  idempotencyKey: string;
}): Promise<WalletBurnResult> {
  return learnerJsonRequest({
    body: JSON.stringify(walletBurnPayload(draft, idempotencyKey)),
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/wallets/me/burns`,
  });
}
