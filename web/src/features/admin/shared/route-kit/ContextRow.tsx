"use client";

import styles from "@/features/admin/shared/admin-routes.module.css";

export function ContextRow({ label, value }: { label: string; value: string }) {
  return (
    <div className={styles.contextRow}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}
