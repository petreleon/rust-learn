"use client";

import { AlertTriangle, CreditCard, FileText, Trophy } from "lucide-react";
import { type OrganizationWalletAudit } from "@/lib/organization/OrganizationWalletAudit";
import { SummaryCard } from "@/features/organization/shared/route-kit/SummaryCard";
import { formatTokenAmount } from "@/features/organization/shared/route-kit/formatTokenAmount";
import styles from "@/features/organization/shared/organization-routes.module.css";

export function WalletSummaryGrid({
  audit,
  attentionCount,
}: {
  audit: OrganizationWalletAudit;
  attentionCount: number;
}) {
  return (
    <section className={styles.summaryGrid}>
      <SummaryCard icon={<CreditCard size={20} aria-hidden />} label="Wallet balance" value={formatTokenAmount(audit.wallet.value)} />
      <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Reward audit rows" value={audit.reward_records.length} />
      <SummaryCard icon={<AlertTriangle size={20} aria-hidden />} label="Needs attention" value={attentionCount} />
      <SummaryCard icon={<FileText size={20} aria-hidden />} label="Ledger rows" value={audit.internal_transactions.length} />
    </section>
  );
}
