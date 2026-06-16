export type WalletBurnSource =
  | "centralized_wallet"
  | "decentralized_direct"
  | "decentralized_platform_mediated";

export type WalletBurnDraft = {
  amount: string;
  chainId: string;
  contractAddress: string;
  depositIntentId: string;
  ethereumAddress: string;
  leaderboardVisible: boolean;
  logIndex: string;
  source: WalletBurnSource;
  transactionHash: string;
};

export type WalletBurnResult = {
  amount: string;
  created_at: string;
  deposit_intent_id: number | null;
  external_transaction_id: number | null;
  fee_amount: string;
  fee_path: string;
  id: number;
  internal_transaction_id: number | null;
  leaderboard_visible: boolean;
  metamask_required: boolean;
  source: string;
  status: string;
  transaction_id: number | null;
  wallet_action: string;
};

export function defaultWalletBurnDraft(): WalletBurnDraft {
  return {
    amount: "",
    chainId: "",
    contractAddress: "",
    depositIntentId: "",
    ethereumAddress: "",
    leaderboardVisible: true,
    logIndex: "",
    source: "centralized_wallet",
    transactionHash: "",
  };
}
