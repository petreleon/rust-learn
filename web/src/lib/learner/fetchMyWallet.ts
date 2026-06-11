import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { learnerErrorFromResponse } from "./learnerErrorFromResponse";
import { learnerRawRequest } from "./learnerRawRequest";
import { type LearnerRequestOptions } from "./LearnerRequestOptions";
import { type WalletSummary } from "./WalletSummary";

export async function fetchMyWallet({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: LearnerRequestOptions): Promise<WalletSummary | null> {
  const response = await learnerRawRequest({
    method: "GET",
    timeoutMs,
    token,
    url: `${apiRoot}/wallets/me`,
  });

  if (response.status === 404) {
    return null;
  }

  if (!response.ok) {
    throw await learnerErrorFromResponse(response, "Wallet could not be loaded.");
  }

  return (await response.json()) as WalletSummary;
}
