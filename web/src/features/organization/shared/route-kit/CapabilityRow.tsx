"use client";

import { enabledOrganizationCapabilities, type OrganizationWorkspaceItem } from "@/lib/organization";
import styles from "@/features/organization/shared/organization-routes.module.css";

export function CapabilityRow({ organization }: { organization: OrganizationWorkspaceItem }) {
  const enabledCapabilities = enabledOrganizationCapabilities(organization);

  return (
    <div className={styles.capabilityRow} aria-label={`${organization.name} capabilities`}>
      {enabledCapabilities.length ? (
        enabledCapabilities.slice(0, 4).map((capability) => (
          <span className={styles.statusPill} key={capability.key}>
            {capability.label}
          </span>
        ))
      ) : (
        <span className={styles.statusPill}>View-only membership</span>
      )}
      {enabledCapabilities.length > 4 ? <span className={styles.statusPill}>+{enabledCapabilities.length - 4}</span> : null}
    </div>
  );
}
