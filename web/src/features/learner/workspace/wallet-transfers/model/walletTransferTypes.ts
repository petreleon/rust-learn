export type WalletTransferOperation = "deposit" | "retire";
export type WalletTransferGasPayer = "platform" | "user";

export type WalletTransferDraft = {
  amount: string;
  chainId: string;
  contractAddress: string;
  ethereumAddress: string;
  gasPayer: WalletTransferGasPayer;
  logIndex: string;
  platformAddress: string;
  transactionHash: string;
};

export type WalletDepositIntentResult = {
  amount: string;
  chain_id: number | null;
  contract_address: string | null;
  ethereum_address: string;
  gas_payer: string;
  id: number;
  log_index: number | null;
  metamask_required: boolean;
  operation: "deposit";
  platform_address: string;
  status: string;
  tax_amount: string;
  transaction_hash: string | null;
  wallet_action: string;
  wallet_delta_on_confirmation: string;
  wallet_id: number;
  wallet_provider: string;
};

export type WalletRetirementResult = {
  amount: string;
  ethereum_address: string;
  external_transaction_id: number;
  gas_payer: string;
  internal_transaction_ids: number[];
  metamask_required: boolean;
  operation: "retire";
  tax_amount: string;
  transaction_id: number;
  wallet_action: string;
  wallet_delta: string;
  wallet_id: number;
  wallet_provider: string;
};

export type WalletTransferResult = WalletDepositIntentResult | WalletRetirementResult;

export function defaultWalletTransferDraft(): WalletTransferDraft {
  return {
    amount: "",
    chainId: "",
    contractAddress: "",
    ethereumAddress: "",
    gasPayer: "platform",
    logIndex: "",
    platformAddress: "",
    transactionHash: "",
  };
}
