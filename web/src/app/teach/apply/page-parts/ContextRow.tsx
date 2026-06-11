"use client";

import styles from "../page.module.css";

export function ContextRow({ label, value }: { label: string; value: string }) {
  return (
    <div className={styles.contextRow}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}
