import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { learnerJsonRequest } from "./learnerJsonRequest";
import { type LearnerRequestOptions } from "./LearnerRequestOptions";
import { type WalletLinkResult } from "./WalletLinkResult";

export async function linkMyWallet({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: LearnerRequestOptions): Promise<WalletLinkResult> {
  return learnerJsonRequest<WalletLinkResult>({
    method: "POST",
    timeoutMs,
    token,
    url: `${apiRoot}/wallets/me/link`,
  });
}
