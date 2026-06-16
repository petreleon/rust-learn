"use client";

import Link from "next/link";
import { type ReactNode } from "react";
import styles from "../learner-workspace.module.css";

export function EmptyState({
  action,
  actionHref,
  actionLabel,
  detail,
  title,
}: {
  action?: ReactNode;
  actionHref?: string;
  actionLabel?: string;
  detail: string;
  title: string;
}) {
  return (
    <section className={styles.statePanel}>
      <h3>{title}</h3>
      <p className={styles.muted}>{detail}</p>
      {action}
      {actionHref && actionLabel ? (
        <Link className={styles.secondaryLink} href={actionHref}>
          {actionLabel}
        </Link>
      ) : null}
    </section>
  );
}
