"use client";

import { AlertTriangle, Download, Loader2 } from "lucide-react";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/components/admin-routes.module.css";
import { type WalletCreditsCsvState } from "../model/WalletCreditsCsvState";

export function WalletCreditsCsvPanel({
  canExport,
  csvError,
  csvState,
  onDownload,
}: {
  canExport: boolean;
  csvError: RouteError | null;
  csvState: WalletCreditsCsvState;
  onDownload: () => void;
}) {
  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <Download size={20} aria-hidden />
        <div>
          <h2>Wallet credits CSV</h2>
          <p>Download wallet credit rows linked to internal transactions and notifications.</p>
        </div>
      </div>
      {csvError ? (
        <div className={styles.inlineError} role="alert">
          <AlertTriangle size={16} aria-hidden />
          <span>{csvError.message}</span>
        </div>
      ) : null}
      <button
        className={styles.secondaryButton}
        disabled={csvState === "downloading" || !canExport}
        onClick={onDownload}
        type="button"
      >
        {csvState === "downloading" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <Download size={16} aria-hidden />}
        Download wallet credits CSV
      </button>
      {!canExport ? <p className={styles.muted}>CSV download requires the EXPORT_DATA permission.</p> : null}
    </section>
  );
}
