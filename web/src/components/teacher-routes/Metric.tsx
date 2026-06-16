"use client";

import styles from "@/features/teacher/shared/teacher-routes.module.css";

export function Metric({
  label,
  tone = "neutral",
  value,
}: {
  label: string;
  tone?: "neutral" | "warn";
  value: number;
}) {
  return (
    <div className={`${styles.metricCard} ${styles[tone]}`}>
      <strong>{value}</strong>
      <span>{label}</span>
    </div>
  );
}
