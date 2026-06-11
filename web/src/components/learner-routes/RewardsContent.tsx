"use client";

import { AlertTriangle, CreditCard, Trophy } from "lucide-react";
import { useMemo } from "react";
import { type RewardHistoryEntry } from "@/lib/learner";
import styles from "../learner-routes.module.css";
import { EmptyState } from "./EmptyState";
import { RewardCard } from "./RewardCard";
import { StatusPill } from "./StatusPill";
import { SummaryCard } from "./SummaryCard";
import { rewardFilterOptions } from "./rewardFilterOptions";
import { type RewardStatusFilter } from "./RewardStatusFilter";

export function RewardsContent({
  filter,
  onChangeFilter,
  rewards,
}: {
  filter: RewardStatusFilter;
  onChangeFilter: (filter: RewardStatusFilter) => void;
  rewards: RewardHistoryEntry[];
}) {
  const summary = useMemo(() => {
    return {
      credited: rewards.filter((reward) => reward.wallet_credit).length,
      needsHelp: rewards.filter((reward) => reward.status === "failed" || reward.status === "needs_reconciliation").length,
      processing: rewards.filter((reward) => reward.status.includes("token") || reward.status === "amount_approved").length,
    };
  }, [rewards]);

  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Reward records" value={rewards.length} />
        <SummaryCard icon={<CreditCard size={20} aria-hidden />} label="Wallet credits" value={summary.credited} />
        <SummaryCard icon={<AlertTriangle size={20} aria-hidden />} label="Need help" value={summary.needsHelp} />
      </section>

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Reward history</h2>
          <StatusPill label={`${summary.processing} processing`} tone="neutral" />
        </div>
        <div className={styles.filterBar} aria-label="Reward status filters">
          {rewardFilterOptions.map((option) => (
            <button
              aria-pressed={filter === option.value}
              className={`${styles.filterButton} ${filter === option.value ? styles.activeFilter : ""}`}
              key={option.value}
              type="button"
              onClick={() => onChangeFilter(option.value)}
            >
              {option.label}
            </button>
          ))}
        </div>

        {rewards.length ? (
          <div className={styles.itemGrid}>
            {rewards.map((reward) => (
              <RewardCard key={reward.reward_candidate_id} reward={reward} />
            ))}
          </div>
        ) : (
          <EmptyState
            action={
              filter === "all" ? undefined : (
                <button className={styles.secondaryButton} type="button" onClick={() => onChangeFilter("all")}>
                  Clear filter
                </button>
              )
            }
            actionHref={filter === "all" ? "/courses" : undefined}
            actionLabel={filter === "all" ? "Check courses" : undefined}
            detail={
              filter === "all"
                ? "Rewards appear after eligible course activity is submitted and reviewed."
                : "There are no rewards in this status right now."
            }
            title={filter === "all" ? "No reward history yet" : "No matching rewards"}
          />
        )}
      </section>
    </>
  );
}
