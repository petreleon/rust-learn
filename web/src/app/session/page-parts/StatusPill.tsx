"use client";

import { type ReactNode } from "react";
import styles from "../page.module.css";

export function StatusPill({
  icon,
  label,
  tone,
}: {
  icon?: ReactNode;
  label: string;
  tone: "good" | "neutral" | "warn";
}) {
  return (
    <span className={`${styles.statusPill} ${styles[tone]}`}>
      {icon}
      {label}
    </span>
  );
}
