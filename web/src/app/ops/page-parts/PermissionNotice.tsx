"use client";

import { ShieldAlert } from "lucide-react";
import styles from "../page.module.css";

export function PermissionNotice({ title, detail }: { title: string; detail: string }) {
  return (
    <div className={styles.panelNotice} role="status" aria-label={`${title}: ${detail}`}>
      <ShieldAlert size={16} aria-hidden />
      <div>
        <strong>{title}</strong>
        <span>{detail}</span>
      </div>
    </div>
  );
}
