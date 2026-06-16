"use client";

import { Building2, CheckCircle2, CreditCard, Trophy } from "lucide-react";
import { type OrganizationRewardDashboard } from "@/lib/organization/OrganizationRewardDashboard";
import { SummaryCard } from "@/components/organization-routes/SummaryCard";
import { formatTokenAmount } from "@/components/organization-routes/formatTokenAmount";
import styles from "@/features/organization/shared/organization-routes.module.css";

export function ReportSummaryGrid({ report }: { report: OrganizationRewardDashboard }) {
  return (
    <section className={styles.summaryGrid}>
      <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Reward candidates" value={report.course_reward_count} />
      <SummaryCard icon={<CheckCircle2 size={20} aria-hidden />} label="Approved rewards" value={report.approved_reward_count} />
      <SummaryCard icon={<CreditCard size={20} aria-hidden />} label="Approved amount" value={formatTokenAmount(report.approved_amount_total)} />
      <SummaryCard icon={<Building2 size={20} aria-hidden />} label="Wallet balance" value={formatTokenAmount(report.wallet_balance_total)} />
    </section>
  );
}
