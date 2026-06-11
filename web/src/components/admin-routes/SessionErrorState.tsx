"use client";

import { AlertTriangle } from "lucide-react";
import Link from "next/link";
import styles from "../admin-routes.module.css";
import { type RouteError } from "./RouteError";

export function SessionErrorState({ error }: { error: RouteError }) {
  return (
    <section className={`${styles.panel} ${styles.errorBox}`} role="alert">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <div>
          <h2>Admin session failed</h2>
          <p>{error.message}</p>
        </div>
      </div>
      <Link className={styles.secondaryLink} href="/login?redirect=/admin">
        Return to login
      </Link>
    </section>
  );
}
