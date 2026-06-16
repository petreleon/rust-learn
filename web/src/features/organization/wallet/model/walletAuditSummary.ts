import { type OrganizationWalletAudit } from "@/lib/organization/OrganizationWalletAudit";
import { numericAmount } from "@/features/organization/shared/route-kit/numericAmount";
import { sumAmounts } from "@/features/organization/shared/route-kit/sumAmounts";
import { walletRewardNeedsAttention } from "@/features/organization/shared/route-kit/walletRewardNeedsAttention";

export function walletAuditSummary(audit: OrganizationWalletAudit) {
  const walletBalance = numericAmount(audit.wallet.value);
  const approvedAmountTotal = sumAmounts(audit.reward_records.map((record) => record.approved_amount));
  const uncreditedAmountTotal = sumAmounts(
    audit.reward_records
      .filter((record) => record.wallet_credit_record_id === null)
      .map((record) => record.approved_amount),
  );
  const attentionRecords = audit.reward_records.filter((record) => walletRewardNeedsAttention(record.reconciliation_status));
  const budgetTone: "good" | "warn" = uncreditedAmountTotal > walletBalance ? "warn" : "good";

  return {
    approvedAmountTotal,
    attentionRecords,
    budgetTone,
    hasAuditRows:
      audit.internal_transactions.length > 0 ||
      audit.external_transactions.length > 0 ||
      audit.reward_records.length > 0 ||
      audit.compensation_records.length > 0,
    uncreditedAmountTotal,
    walletBalance,
  };
}
