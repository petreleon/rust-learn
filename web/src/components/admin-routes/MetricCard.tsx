"use client";

import styles from "../admin-routes.module.css";
import { formatNumber } from "./formatNumber";

export function MetricCard({
  label,
  tone = "neutral",
  value,
}: {
  label: string;
  tone?: "good" | "neutral" | "warn";
  value: number | string;
}) {
  return (
    <article className={`${styles.metricCard} ${styles[tone]}`}>
      <strong>{typeof value === "number" ? formatNumber(value) : value}</strong>
      <span>{label}</span>
    </article>
  );
}
