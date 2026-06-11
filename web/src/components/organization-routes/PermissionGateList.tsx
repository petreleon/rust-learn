"use client";

import { type OrganizationDashboardSummary } from "@/lib/organization";
import styles from "../organization-routes.module.css";

export function PermissionGateList({ dashboard }: { dashboard: OrganizationDashboardSummary }) {
  const gatedSections = [
    { label: "Members", section: dashboard.members },
    { label: "Courses", section: dashboard.courses },
    { label: "Teacher applications", section: dashboard.teacher_applications },
    { label: "Reports", section: dashboard.rewards },
    { label: "Wallet", section: dashboard.wallet },
  ].filter((item) => !item.section.available);

  if (!gatedSections.length) {
    return (
      <div className={styles.permissionRow}>
        <span className={styles.permissionChip}>All dashboard sections visible</span>
      </div>
    );
  }

  return (
    <div className={styles.missingList}>
      <strong>Gated dashboard sections</strong>
      {gatedSections.map((item) => (
        <span key={item.label}>
          {item.label}: {item.section.missing_permissions.join(", ")}
        </span>
      ))}
    </div>
  );
}
