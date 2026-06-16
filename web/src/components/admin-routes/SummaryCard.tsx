"use client";

import { type ReactNode } from "react";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { formatNumber } from "./formatNumber";

export function SummaryCard({ icon, label, value }: { icon: ReactNode; label: string; value: number }) {
  return (
    <article className={styles.summaryCard}>
      <span className={styles.smallIcon}>{icon}</span>
      <strong>{formatNumber(value)}</strong>
      <span>{label}</span>
    </article>
  );
}
