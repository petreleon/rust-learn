"use client";

import styles from "../learner-routes.module.css";

export function StatusPill({
  label,
  tone,
}: {
  label: string;
  tone: "bad" | "good" | "neutral" | "warn";
}) {
  return <span className={`${styles.statusPill} ${styles[tone]}`}>{label}</span>;
}
