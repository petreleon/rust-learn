"use client";

import { type PlatformSessionScope } from "@/lib/session";
import styles from "../page.module.css";
import { previewLimit } from "./previewLimit";

export function ScopeSummary({ title, scope }: { title: string; scope: PlatformSessionScope }) {
  const visiblePermissions = scope.effective_permissions.slice(0, previewLimit);
  const remainingCount = scope.effective_permissions.length - visiblePermissions.length;

  return (
    <div className={styles.scopeSummary}>
      <p className={styles.summaryTitle}>{title}</p>
      <div className={styles.badgeRow}>
        {scope.roles.map((role) => (
          <span className={styles.roleBadge} key={role}>
            {role}
          </span>
        ))}
        {scope.roles.length === 0 ? <span className={styles.roleBadge}>Delegated</span> : null}
      </div>
      <div className={styles.permissionList}>
        {visiblePermissions.map((permission) => (
          <span key={permission}>{permission}</span>
        ))}
        {remainingCount > 0 ? <span>+{remainingCount}</span> : null}
        {scope.effective_permissions.length === 0 ? <span>No permissions</span> : null}
      </div>
    </div>
  );
}
