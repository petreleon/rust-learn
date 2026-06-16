"use client";

import { CheckCircle2 } from "lucide-react";
import styles from "@/features/admin/shared/admin-routes.module.css";

export function EmptyState({ text }: { text: string }) {
  return (
    <div className={styles.emptyState} role="status">
      <CheckCircle2 size={18} aria-hidden />
      <span>{text}</span>
    </div>
  );
}
