"use client";

import Link from "next/link";
import styles from "../learner-routes.module.css";
import { StatusPill } from "./StatusPill";
import { summarizeRewards } from "./summarizeRewards";

export function DashboardRewardPanel({
  rewardCount,
  summary,
}: {
  rewardCount: number;
  summary: ReturnType<typeof summarizeRewards>;
}) {
  return (
    <article className={styles.itemCard}>
      <div className={styles.itemHeader}>
        <h3>Reward status</h3>
        <StatusPill label={`${rewardCount} recent`} tone={rewardCount ? "good" : "neutral"} />
      </div>
      <div className={styles.detailList}>
        <span>{summary.pendingTeacher} waiting for teacher review</span>
        <span>{summary.processing} processing or amount-approved</span>
        <span>{summary.credited} credited to wallet</span>
        <span>{summary.needsHelp} need help</span>
      </div>
      <div className={styles.actionRow}>
        <Link className={styles.secondaryLink} href="/rewards">
          View rewards
        </Link>
      </div>
    </article>
  );
}
