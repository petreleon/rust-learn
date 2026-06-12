export type WalletKycGateCopy = {
  actionDetail: string;
  detail: string;
  label: string;
  ready: boolean;
  tone: "good" | "warn";
};

export function walletKycGateCopy(kycVerified: boolean): WalletKycGateCopy {
  if (kycVerified) {
    return {
      actionDetail: "Wallet link, deposit, and retirement actions can be offered when their route contracts are ready.",
      detail: "Identity verification is complete for wallet and payout operations.",
      label: "KYC verified",
      ready: true,
      tone: "good",
    };
  }

  return {
    actionDetail: "Wallet linking, deposits, retirements, and payout actions stay disabled until identity review is verified.",
    detail: "Complete KYC in account settings before using wallet actions.",
    label: "KYC required",
    ready: false,
    tone: "warn",
  };
}
