"use client";

import Link from "next/link";
import { type OrganizationDashboardAlert } from "@/lib/organization";
import styles from "../organization-routes.module.css";
import { StatusPill } from "./StatusPill";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";

export function DashboardAlertCard({ alert }: { alert: OrganizationDashboardAlert }) {
  const tone = alert.severity === "warning" ? "warn" : "good";
  return (
    <article className={styles.compactRow}>
      <span>
        <StatusPill label={formatUnderscoreLabel(alert.kind)} tone={tone} />
        {alert.message}
      </span>
      {alert.action_href && alert.action_label ? (
        <Link className={styles.secondaryLink} href={alert.action_href}>
          {alert.action_label}
        </Link>
      ) : null}
    </article>
  );
}
