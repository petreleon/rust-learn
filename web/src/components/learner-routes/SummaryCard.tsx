"use client";

import { type ReactNode } from "react";
import styles from "../learner-routes.module.css";

export function SummaryCard({
  icon,
  label,
  value,
}: {
  icon: ReactNode;
  label: string;
  value: number | string;
}) {
  return (
    <article className={styles.summaryCard}>
      {icon}
      <strong>{value}</strong>
      <span className={styles.muted}>{label}</span>
    </article>
  );
}
