"use client";

import { type PlatformSessionScope } from "@/lib/session/PlatformSessionScope";
import styles from "@/app/session/page.module.css";
import { formatAccessLabel, pluralize } from "../model/formatAccessLabel";

export function ScopeSummary({ title, scope }: { title: string; scope: PlatformSessionScope }) {
  const permissionCount = scope.effective_permissions.length;
  const roleLabels = scope.roles.length
    ? scope.roles.map(formatAccessLabel)
    : [scope.delegated_permissions.length ? "Delegated access" : "No role"];

  return (
    <div className={styles.scopeSummary}>
      <p className={styles.summaryTitle}>{title}</p>
      <div className={styles.badgeRow}>
        {roleLabels.map((role) => (
          <span className={styles.roleBadge} key={role}>
            {role}
          </span>
        ))}
      </div>
      <div className={styles.permissionList} aria-label={`${title} permission summary`}>
        <span>{permissionCount ? pluralize(permissionCount, "permission") : "No permissions"}</span>
        {scope.direct_permissions.length ? <span>{pluralize(scope.direct_permissions.length, "direct")}</span> : null}
        {scope.delegated_permissions.length ? (
          <span>{pluralize(scope.delegated_permissions.length, "delegated")}</span>
        ) : null}
      </div>
      {permissionCount ? (
        <details className={styles.permissionDetails}>
          <summary>View permission details</summary>
          <ul className={styles.permissionDetailList}>
            {scope.effective_permissions.map((permission) => (
              <li className={styles.permissionDetailItem} key={permission}>
                <span>{formatAccessLabel(permission)}</span>
                <code>{permission}</code>
              </li>
            ))}
          </ul>
        </details>
      ) : null}
    </div>
  );
}
