"use client";

import styles from "../page.module.css";

export function EmptyState({ title, detail }: { title: string; detail: string }) {
  return (
    <div className={styles.emptyState}>
      <p>{title}</p>
      <span>{detail}</span>
    </div>
  );
}
