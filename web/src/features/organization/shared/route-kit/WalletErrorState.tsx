"use client";

import { AlertTriangle, RefreshCw } from "lucide-react";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { type RouteError } from "./RouteError";

export function WalletErrorState({
  error,
  onRetry,
}: {
  error: RouteError | null;
  onRetry: () => void;
}) {
  return (
    <section className={styles.errorBox} role="status">
      <AlertTriangle size={20} aria-hidden />
      <span>
        <strong>{error?.code || "wallet_error"}</strong>
        <span>{error?.message || "Organization wallet audit could not be loaded."}</span>
      </span>
      <button className={styles.secondaryButton} onClick={onRetry} type="button">
        <RefreshCw size={17} aria-hidden />
        Retry
      </button>
    </section>
  );
}
