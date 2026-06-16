"use client";

import { AlertCircle, Loader2, LogIn } from "lucide-react";
import Link from "next/link";
import styles from "@/features/session/account-settings/page.module.css";
import { type RouteError } from "../model/RouteError";

export function LoadingAccount() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
      <div className={styles.panelHeader}>
        <Loader2 className={styles.spin} size={22} aria-hidden />
        <h2>Loading account</h2>
      </div>
      <p className={styles.muted}>Resolving profile, verification state, wallet readiness, and notification defaults.</p>
    </section>
  );
}

export function SignedOutAccount() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`}>
      <div className={styles.panelHeader}>
        <LogIn size={22} aria-hidden />
        <h2>Sign in required</h2>
      </div>
      <p className={styles.muted}>Account settings load after RustLearn resolves your current session.</p>
      <Link className={styles.primaryLink} href="/login?redirect=/settings/account">
        <LogIn size={18} aria-hidden />
        Sign in
      </Link>
    </section>
  );
}

export function AccountError({ error }: { error: RouteError }) {
  return (
    <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
      <AlertCircle size={18} aria-hidden />
      <span>
        <strong>{error.code}</strong>
        {error.message}
      </span>
      <Link className={styles.secondaryLink} href="/login?redirect=/settings/account">
        Return to login
      </Link>
    </section>
  );
}
