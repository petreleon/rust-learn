"use client";

import { type PlatformSessionScope } from "@/lib/session";
import styles from "../workspace-route.module.css";

export function PermissionPreview({ scope }: { scope: PlatformSessionScope }) {
  const visiblePermissions = scope.effective_permissions.slice(0, 6);
  const remaining = scope.effective_permissions.length - visiblePermissions.length;
  return (
    <div className={styles.permissionList}>
      {visiblePermissions.map((permission) => (
        <span key={permission}>{permission}</span>
      ))}
      {remaining > 0 ? <span>+{remaining}</span> : null}
      {scope.effective_permissions.length === 0 ? <span>No permissions</span> : null}
    </div>
  );
}
