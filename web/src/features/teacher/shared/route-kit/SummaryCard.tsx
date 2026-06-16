"use client";

import { type ReactNode } from "react";
import styles from "@/features/teacher/shared/teacher-routes.module.css";

export function SummaryCard({
  icon,
  label,
  tone = "neutral",
  value,
}: {
  icon: ReactNode;
  label: string;
  tone?: "good" | "neutral" | "warn";
  value: number;
}) {
  return (
    <article className={styles.summaryCard}>
      <span className={`${styles.smallIcon} ${styles[tone]}`}>{icon}</span>
      <strong>{value}</strong>
      <span>{label}</span>
    </article>
  );
}
