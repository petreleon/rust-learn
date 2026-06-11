"use client";

import styles from "../teacher-routes.module.css";

export function DetailLine({ label, value }: { label: string; value: string }) {
  return (
    <div className={styles.detailLine}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}
