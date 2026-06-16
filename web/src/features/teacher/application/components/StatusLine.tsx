"use client";

import { type ReactNode } from "react";
import styles from "@/app/teach/apply/page.module.css";

export function StatusLine({
  icon,
  label,
  tone,
}: {
  icon: ReactNode;
  label: string;
  tone: "good" | "neutral" | "warn" | "bad";
}) {
  return (
    <span className={`${styles.statusLine} ${styles[tone]}`}>
      {icon}
      {label}
    </span>
  );
}
