export type OrganizationBurnSource =
  | "centralized_wallet"
  | "decentralized_direct"
  | "decentralized_platform_mediated";

export type OrganizationBurnDraft = {
  amount: string;
  chainId: string;
  contractAddress: string;
  depositIntentId: string;
  ethereumAddress: string;
  leaderboardVisible: boolean;
  logIndex: string;
  source: OrganizationBurnSource;
  transactionHash: string;
};

export type OrganizationBurnPermissions = {
  can_burn: boolean;
  can_request_burn: boolean;
  kyc_verified: boolean;
  organization_id: number;
  required_permission: string;
};

export type OrganizationTokenBurn = {
  amount: string;
  actor_user_id: number;
  created_at: string;
  fee_amount: string;
  fee_path: string;
  id: number;
  internal_transaction_id: number | null;
  leaderboard_visible: boolean;
  organization_id: number | null;
  source: string;
  status: string;
  transaction_id: number | null;
  wallet_action: string;
};

export function defaultOrganizationBurnDraft(): OrganizationBurnDraft {
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
