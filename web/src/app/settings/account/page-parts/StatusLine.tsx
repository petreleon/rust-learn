"use client";

import { CheckCircle2 } from "lucide-react";
import { type ReactNode } from "react";
import styles from "../page.module.css";

export function StatusLine({
  icon = <CheckCircle2 size={16} aria-hidden />,
  label,
  tone,
}: {
  icon?: ReactNode;
  label: string;
  tone: "good" | "neutral" | "warn";
}) {
  return (
    <span className={`${styles.statusLine} ${styles[tone]}`}>
      {icon}
      {label}
    </span>
  );
}
