"use client";

import { AlertTriangle } from "lucide-react";
import Link from "next/link";
import { missingOrganizationPermissions, type OrganizationCapability } from "@/lib/organization";
import styles from "../organization-routes.module.css";

export function TeacherApplicationsDeniedState({
  capability,
  organizationName,
}: {
  capability?: OrganizationCapability;
  organizationName: string;
}) {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>Teacher applications unavailable</h2>
      </div>
      <p className={styles.muted}>
        Your current session can see that {organizationName} exists, but it cannot open sponsored
        teacher application tracking.
      </p>
      <div className={styles.missingList}>
        <strong>Missing scoped permission</strong>
        {(capability
          ? missingOrganizationPermissions(capability)
          : ["VIEW_ORG_TEACHER_APPLICATIONS", "NOMINATE_TEACHER_FOR_PLATFORM_REVIEW"]
        ).map((permission) => (
          <span key={permission}>{permission}</span>
        ))}
      </div>
      <Link className={styles.secondaryLink} href="/organizations">
        Back to organizations
      </Link>
    </section>
  );
}
