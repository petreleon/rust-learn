"use client";

import { ShieldAlert } from "lucide-react";
import styles from "../ops-console.module.css";

export function ServerDeniedNotice({ action }: { action: string }) {
  return (
    <div
      className={`${styles.requirementNotice} ${styles.deniedNotice}`}
      role="status"
      aria-label={`${action}: server denied`}
    >
      <ShieldAlert size={16} aria-hidden />
      <div>
        <strong>Server denied</strong>
        <span>{action} is locked for this JWT.</span>
      </div>
    </div>
  );
}
