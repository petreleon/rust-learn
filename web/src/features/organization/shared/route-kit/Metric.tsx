"use client";

import styles from "@/features/organization/shared/organization-routes.module.css";

export function Metric({ label, value }: { label: string; value: number | string }) {
  return (
    <span className={styles.metricCard}>
      <strong>{value}</strong>
      <span>{label}</span>
    </span>
  );
}
