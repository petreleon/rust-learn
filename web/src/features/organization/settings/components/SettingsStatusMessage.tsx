"use client";

import { AlertTriangle, CheckCircle2 } from "lucide-react";
import styles from "@/components/organization-routes.module.css";
import { type SettingsSaveState } from "../model/SettingsSaveState";

export function SettingsStatusMessage({
  errorTitle,
  message,
  state,
  successTitle,
}: {
  errorTitle: string;
  message: string | null;
  state: SettingsSaveState;
  successTitle: string;
}) {
  if (!message) {
    return null;
  }

  return (
    <section className={`${styles.warningPanel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        {state === "success" ? <CheckCircle2 size={18} aria-hidden /> : <AlertTriangle size={18} aria-hidden />}
        <h2>{state === "success" ? successTitle : errorTitle}</h2>
      </div>
      <p>{message}</p>
    </section>
  );
}
