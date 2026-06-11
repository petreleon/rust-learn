"use client";

import { AlertTriangle, RefreshCw } from "lucide-react";
import styles from "../admin-routes.module.css";
import { type RouteError } from "./RouteError";

export function PanelError({
  error,
  onRetry,
  title,
}: {
  error: RouteError | null;
  onRetry: () => void;
  title: string;
}) {
  return (
    <section className={`${styles.panel} ${styles.errorBox}`} role="alert">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <div>
          <h2>{title}</h2>
          <p>{error?.message || "The request failed before the API returned details."}</p>
        </div>
      </div>
      <button className={styles.secondaryButton} onClick={onRetry} type="button">
        <RefreshCw size={16} aria-hidden />
        Retry
      </button>
    </section>
  );
}
