"use client";

import { AlertTriangle, RefreshCw } from "lucide-react";
import Link from "next/link";
import styles from "../learner-routes.module.css";
import { loginHref } from "./loginHref";
import { type RouteError } from "./RouteError";

export function ErrorState({
  error,
  onRetry,
  redirect,
}: {
  error: RouteError;
  onRetry: () => void;
  redirect: string;
}) {
  const needsSignIn = error.code === "unauthorized" || error.code === "missing_token" || error.code === "missing_user";
  const needsVerification = error.code === "unverified_email";

  return (
    <section className={styles.errorPanel} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>{error.code}</h2>
      </div>
      <p>{error.message}</p>
      {needsVerification ? (
        <Link className={styles.secondaryLink} href="/verify-email">
          Verify email
        </Link>
      ) : needsSignIn ? (
        <Link className={styles.secondaryLink} href={loginHref(redirect)}>
          Return to login
        </Link>
      ) : (
        <button className={styles.secondaryButton} type="button" onClick={onRetry}>
          <RefreshCw size={18} aria-hidden />
          Retry
        </button>
      )}
    </section>
  );
}
