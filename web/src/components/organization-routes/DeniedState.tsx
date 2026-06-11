"use client";

import { AlertTriangle } from "lucide-react";
import Link from "next/link";
import styles from "../organization-routes.module.css";

export function DeniedState() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>No organization workspace yet</h2>
      </div>
      <p className={styles.muted}>
        This account does not currently include organization roles, direct organization
        permissions, delegated organization permissions, or effective organization permissions.
      </p>
      <Link className={styles.secondaryLink} href="/session">
        Review current session
      </Link>
    </section>
  );
}
