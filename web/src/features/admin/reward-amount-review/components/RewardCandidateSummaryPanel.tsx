"use client";

import { CheckCircle2, Clock3, FileText, UserCheck, XCircle } from "lucide-react";
import { type PlatformRewardCandidatesResponse } from "@/lib/admin/PlatformRewardCandidatesResponse";
import { type LoadState } from "@/shared/route-state/LoadState";
import styles from "@/components/admin-routes.module.css";
import { SummaryCard } from "@/components/admin-routes/SummaryCard";
import { countCandidatesByStatus } from "../model/rewardCandidateDisplay";

export function RewardCandidateSummaryPanel({
  candidates,
  state,
}: {
  candidates: PlatformRewardCandidatesResponse | null;
  state: LoadState;
}) {
  const visible = candidates?.candidates ?? [];

  return (
    <section className={styles.summaryGrid} aria-label="Reward candidate summary">
      <SummaryCard
        icon={<UserCheck size={20} aria-hidden />}
        label="Teacher approved"
        value={countCandidatesByStatus(visible, "teacher_approved")}
      />
      <SummaryCard
        icon={<CheckCircle2 size={20} aria-hidden />}
        label="Amount approved"
        value={countCandidatesByStatus(visible, "amount_approved")}
      />
      <SummaryCard
        icon={<XCircle size={20} aria-hidden />}
        label="Amount rejected"
        value={countCandidatesByStatus(visible, "amount_rejected")}
      />
      <SummaryCard
        icon={<Clock3 size={20} aria-hidden />}
        label="Token pending"
        value={countCandidatesByStatus(visible, "token_pending")}
      />
      <SummaryCard
        icon={<FileText size={20} aria-hidden />}
        label={state === "loading" ? "Loading" : "Total visible"}
        value={candidates?.total ?? 0}
      />
    </section>
  );
}
