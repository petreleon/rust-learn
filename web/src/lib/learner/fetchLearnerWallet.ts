import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { DEFAULT_WALLET_REWARD_LIMIT } from "./DEFAULT_WALLET_REWARD_LIMIT";
import { fetchMyWalletAudit } from "./fetchMyWalletAudit";
import { fetchMyWallet } from "./fetchMyWallet";
import { fetchRewardHistory } from "./fetchRewardHistory";
import { type LearnerRequestOptions } from "./LearnerRequestOptions";
import { type LearnerWalletSnapshot } from "./LearnerWalletSnapshot";

export async function fetchLearnerWallet({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: LearnerRequestOptions): Promise<LearnerWalletSnapshot> {
  const [wallet, rewardHistory] = await Promise.all([
    fetchMyWallet({
      apiRoot,
      timeoutMs,
      token,
    }),
    fetchRewardHistory({
      apiRoot,
      limit: DEFAULT_WALLET_REWARD_LIMIT,
      timeoutMs,
      token,
    }),
  ]);
  const walletAudit = wallet
    ? await fetchMyWalletAudit({
        apiRoot,
        timeoutMs,
        token,
      })
    : null;

  return {
    reward_history: rewardHistory,
    wallet_history: walletAudit?.deposit_intents || [],
    wallet,
  };
}
