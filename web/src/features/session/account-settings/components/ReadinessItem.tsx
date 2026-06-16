"use client";

import styles from "@/app/settings/account/page.module.css";
import { StatusLine } from "./StatusLine";

export function ReadinessItem({
  detail,
  label,
  tone,
}: {
  detail: string;
  label: string;
  tone: "good" | "neutral" | "warn";
}) {
  return (
    <div className={styles.readinessItem}>
      <StatusLine label={label} tone={tone} />
      <p className={styles.muted}>{detail}</p>
    </div>
  );
}
