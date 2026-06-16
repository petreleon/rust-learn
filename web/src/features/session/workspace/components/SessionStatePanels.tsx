"use client";

import { AlertCircle, Loader2, LogIn } from "lucide-react";
import Link from "next/link";
import styles from "@/app/session/page.module.css";
import { type RouteError } from "../model/RouteError";

export function SignedOutPanel() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`}>
      <div className={styles.panelHeader}>
        <LogIn size={20} aria-hidden />
        <h2>Sign in required</h2>
      </div>
      <p className={styles.muted}>
        The workspace loads from the session created by the login flow. No manual token paste is needed on product routes.
      </p>
      <Link className={styles.primaryLink} href="/login?redirect=/session">
        <LogIn size={18} aria-hidden />
        Sign in
      </Link>
    </section>
  );
}

export function LoadingPanel() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
      <div className={styles.panelHeader}>
        <Loader2 className={styles.spin} size={20} aria-hidden />
        <h2>Loading workspace</h2>
      </div>
      <p className={styles.muted}>Resolving your account, permissions, organizations, courses, and delegated access.</p>
    </section>
  );
}

export function ErrorPanel({ error }: { error: RouteError }) {
  return (
    <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
      <AlertCircle size={18} aria-hidden />
      <span>
        <strong>{error.code}</strong>
        {error.message}
      </span>
      <Link className={styles.secondaryLink} href="/login?redirect=/session">
        Return to login
      </Link>
    </section>
  );
}
