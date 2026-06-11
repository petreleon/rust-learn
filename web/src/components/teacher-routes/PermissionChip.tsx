"use client";

import styles from "../teacher-routes.module.css";

export function PermissionChip({ enabled, label }: { enabled: boolean; label: string }) {
  return <span className={`${styles.permissionChip} ${enabled ? styles.good : styles.neutral}`}>{label}</span>;
}
