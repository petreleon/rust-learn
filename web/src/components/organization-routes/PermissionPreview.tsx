"use client";

import { type OrganizationWorkspaceItem } from "@/lib/organization";
import styles from "../organization-routes.module.css";

export function PermissionPreview({ organization }: { organization: OrganizationWorkspaceItem }) {
  const permissions = organization.permissionPreview;
  const remaining = organization.effectivePermissionCount - permissions.length;

  return (
    <div className={styles.permissionList}>
      {permissions.length ? (
        permissions.map((permission) => <span key={permission}>{permission}</span>)
      ) : (
        <span>No effective permissions resolved</span>
      )}
      {remaining > 0 ? <span>+{remaining} more</span> : null}
    </div>
  );
}
