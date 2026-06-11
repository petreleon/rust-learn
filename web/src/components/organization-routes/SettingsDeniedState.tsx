"use client";

import { Settings, ShieldCheck } from "lucide-react";
import { missingOrganizationPermissions, type OrganizationCapability } from "@/lib/organization";
import styles from "../organization-routes.module.css";

export function SettingsDeniedState({
  capability,
  organizationName,
}: {
  capability?: OrganizationCapability;
  organizationName: string;
}) {
  const missingPermissions = capability ? missingOrganizationPermissions(capability) : [];
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`}>
      <div className={styles.panelHeader}>
        <ShieldCheck size={20} aria-hidden />
        <h2>Settings unavailable</h2>
      </div>
      <p className={styles.muted}>
        Your current session cannot manage settings for {organizationName}.
        {missingPermissions.length ? ` Missing permission${missingPermissions.length === 1 ? "" : "s"}: ${missingPermissions.join(", ")}.` : ""}
      </p>
    </section>
  );
}
