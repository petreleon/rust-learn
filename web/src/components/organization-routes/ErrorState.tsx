"use client";

import { AlertTriangle } from "lucide-react";
import Link from "next/link";
import styles from "../organization-routes.module.css";
import { type RouteError } from "./RouteError";

export function ErrorState({
  error,
  redirect,
}: {
  error: RouteError;
  redirect: string;
}) {
  return (
    <section className={styles.errorBox} role="status">
      <AlertTriangle size={20} aria-hidden />
      <span>
        <strong>{error.code}</strong>
        <span>{error.message}</span>
      </span>
      <Link className={styles.secondaryLink} href={`/login?redirect=${redirect}`}>
        Return to login
      </Link>
    </section>
  );
}
