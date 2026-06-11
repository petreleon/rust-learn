"use client";

import { type ReactNode } from "react";
import styles from "../teacher-routes.module.css";

export function StatusLine({
  icon,
  label,
  tone,
}: {
  icon: ReactNode;
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
