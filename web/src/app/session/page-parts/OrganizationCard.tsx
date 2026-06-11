"use client";

import { type OrganizationSessionScope } from "@/lib/session";
import styles from "../page.module.css";
import { ScopeSummary } from "./ScopeSummary";

export function OrganizationCard({ organization }: { organization: OrganizationSessionScope }) {
  return (
    <article className={styles.scopeCard}>
      <h3>{organization.name}</h3>
      <p className={styles.muted}>Organization #{organization.id}</p>
      <ScopeSummary title="Access" scope={organization} />
    </article>
  );
}
