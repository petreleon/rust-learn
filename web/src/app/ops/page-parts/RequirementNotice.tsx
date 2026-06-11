"use client";

import { FileCheck } from "lucide-react";
import styles from "../page.module.css";
import { formatFieldList } from "./formatFieldList";

export function RequirementNotice({ action, fields }: { action: string; fields: string[] }) {
  if (fields.length === 0) {
    return null;
  }
  const message = formatFieldList(fields);

  return (
    <div className={styles.requirementNotice} role="status" aria-label={`${action}: ${message}`}>
      <FileCheck size={16} aria-hidden />
      <div>
        <strong>{action}</strong>
        <span>{message}</span>
      </div>
    </div>
  );
}
