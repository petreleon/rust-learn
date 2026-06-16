"use client";

import { FileCheck } from "lucide-react";
import styles from "../ops-console.module.css";
import { formatFieldList } from "../model/formatFieldList";

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
