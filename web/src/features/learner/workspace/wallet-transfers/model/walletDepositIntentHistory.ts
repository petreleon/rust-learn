import { type WalletDepositIntentAudit } from "@/lib/learner";

export type WalletHistoryTone = "bad" | "good" | "neutral" | "warn";

export function walletDepositIntentTone(status: string): WalletHistoryTone {
  if (status === "credited") return "good";
  if (status === "failed") return "bad";
  if (status === "ambiguous" || status === "pending_chain_confirmation") return "warn";
  return "neutral";
}

export function walletDepositIntentStatusLabel(status: string) {
  return status.replaceAll("_", " ");
}

export function walletDepositIntentNextStep(intent: WalletDepositIntentAudit) {
  switch (intent.status) {
    case "pending_chain_confirmation":
      return intent.metamask_required
        ? "Waiting for the external wallet action and chain confirmation."
        : "Waiting for platform-paid chain confirmation.";
    case "credited":
      return "Deposit was matched on-chain and credited to this wallet.";
    case "ambiguous":
      return "Recreate this deposit intent or contact support with the transaction hash.";
    case "failed":
      return intent.last_error
        ? `Retry by creating a new deposit intent. Last error: ${intent.last_error}`
        : "Retry by creating a new deposit intent when ready.";
    default:
      return "Deposit intent is waiting for wallet reconciliation.";
  }
}
