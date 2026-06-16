"use client";

import { type ReactNode } from "react";
import styles from "@/features/organization/shared/organization-routes.module.css";

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
      <span className={styles.smallIcon}>{icon}</span>
      <strong>{value}</strong>
      <span>{label}</span>
    </article>
  );
}
