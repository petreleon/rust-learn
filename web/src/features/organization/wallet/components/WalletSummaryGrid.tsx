"use client";

import { AlertTriangle, CreditCard, FileText, Trophy } from "lucide-react";
import { type OrganizationWalletAudit } from "@/lib/organization/OrganizationWalletAudit";
import { SummaryCard } from "@/components/organization-routes/SummaryCard";
import { formatTokenAmount } from "@/components/organization-routes/formatTokenAmount";
import styles from "@/components/organization-routes.module.css";

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
