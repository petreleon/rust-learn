"use client";

import { AlertTriangle } from "lucide-react";
import Link from "next/link";
import styles from "../workspace-route.module.css";

export function ErrorState({
  error,
  redirect,
}: {
  error: { code: string; message: string };
  redirect: string;
}) {
  return (
    <section className={styles.errorPanel} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>{error.code}</h2>
      </div>
      <p>{error.message}</p>
      <Link className={styles.secondaryLink} href={`/login?redirect=${redirect}`}>
        Return to login
      </Link>
    </section>
  );
}
