"use client";

import { type ReactNode } from "react";
import styles from "@/features/session/account-settings/page.module.css";

export function MetricCard({
  icon,
  label,
  value,
}: {
  icon: ReactNode;
  label: string;
  value: number;
}) {
  return (
    <div className={styles.metricCard}>
      {icon}
      <strong>{value}</strong>
      <span>{label}</span>
    </div>
  );
}
