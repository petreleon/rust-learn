"use client";

import { AlertTriangle, Loader2, RefreshCw } from "lucide-react";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/organization/shared/organization-routes.module.css";

export function SettingsLoadStatePanel({
  onRefresh,
  settingsError,
  settingsLoadState,
}: {
  onRefresh: () => void;
  settingsError: RouteError | null;
  settingsLoadState: LoadState;
}) {
  if (settingsLoadState === "loading") {
    return (
      <section className={`${styles.panel} ${styles.singlePanel}`}>
        <div className={styles.panelHeader}>
          <Loader2 className={styles.spin} size={20} aria-hidden />
          <h2>Loading settings</h2>
        </div>
        <p className={styles.muted}>Fetching current organization values from the backend.</p>
      </section>
    );
  }

  if (settingsLoadState === "error" && settingsError) {
    return (
      <section className={`${styles.errorBox} ${styles.singlePanel}`}>
        <AlertTriangle size={18} aria-hidden />
        <span>
          <strong>{settingsError.code}</strong> {settingsError.message}
        </span>
        <button className={styles.secondaryButton} onClick={onRefresh} type="button">
          <RefreshCw size={17} aria-hidden />
          Retry
        </button>
      </section>
    );
  }

  return null;
}
