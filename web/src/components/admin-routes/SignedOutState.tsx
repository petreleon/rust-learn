"use client";

import { AlertTriangle } from "lucide-react";
import Link from "next/link";
import styles from "../admin-routes.module.css";

export function SignedOutState() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <div>
          <h2>Sign in required</h2>
          <p>Platform admin data loads only after the current session is resolved.</p>
        </div>
      </div>
      <Link className={styles.secondaryLink} href="/login?redirect=/admin">
        Return to login
      </Link>
    </section>
  );
}
