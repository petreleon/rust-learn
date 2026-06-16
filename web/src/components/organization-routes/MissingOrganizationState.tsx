"use client";

import { AlertTriangle } from "lucide-react";
import Link from "next/link";
import styles from "@/features/organization/shared/organization-routes.module.css";

export function MissingOrganizationState() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>Organization unavailable</h2>
      </div>
      <p className={styles.muted}>
        This organization is not visible to your current session. It may have been removed,
        renamed, or your scoped access may have changed.
      </p>
      <Link className={styles.secondaryLink} href="/organizations">
        Back to organizations
      </Link>
    </section>
  );
}
