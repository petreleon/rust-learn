"use client";

import { ArrowLeft } from "lucide-react";
import Link from "next/link";
import { type OrganizationWorkspaceItem } from "@/lib/organization/OrganizationWorkspaceItem";
import styles from "@/features/organization/shared/organization-routes.module.css";

export function SettingsHero({ organization }: { organization: OrganizationWorkspaceItem }) {
  const roles = organization.roles.join(", ") || "None";
  const roleSummary = organization.roles.length ? roles : "Delegated access only";

  return (
    <section className={styles.workspaceHero}>
      <Link className={styles.secondaryLink} href={`/organizations/${organization.id}`}>
        <ArrowLeft size={17} aria-hidden />
        {organization.name}
      </Link>
      <div className={styles.workspaceTitleBlock}>
        <p className={styles.eyebrow}>{roleSummary}</p>
        <h2>Organization settings</h2>
        <p className={styles.muted}>
          Update the public identity fields for {organization.name}. These values appear in learner-facing course discovery and
          teacher application contexts.
        </p>
      </div>
    </section>
  );
}
