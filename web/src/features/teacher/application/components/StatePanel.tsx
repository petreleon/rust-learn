"use client";

import { type ReactNode } from "react";
import styles from "@/app/teach/apply/page.module.css";

export function StatePanel({
  action,
  detail,
  icon,
  title,
}: {
  action?: ReactNode;
  detail: string;
  icon: ReactNode;
  title: string;
}) {
  return (
    <section className={`${styles.statusPanel} ${styles.singlePanel}`} aria-live="polite">
      <div className={styles.panelHeader}>
        {icon}
        <h2>{title}</h2>
      </div>
      <p className={styles.muted}>{detail}</p>
      {action}
    </section>
  );
}
