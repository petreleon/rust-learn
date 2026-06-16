"use client";

import { ShieldCheck } from "lucide-react";
import { type AdminRole, type AdminUserProfile } from "@/lib/admin";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { EmptyState } from "@/features/admin/shared/route-kit/EmptyState";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { type RouteError } from "@/shared/route-state/RouteError";

export function RoleAssignmentPanel({
  canAssign,
  canViewRoleCatalog,
  error,
  onAssign,
  onRoleNameChange,
  roleName,
  roles,
  rolesError,
  rolesState,
  state,
  user,
}: {
  canAssign: boolean;
  canViewRoleCatalog: boolean;
  error: RouteError | null;
  onAssign: () => void;
  onRoleNameChange: (value: string) => void;
  roleName: string;
  roles: AdminRole[];
  rolesError: RouteError | null;
  rolesState: string;
  state: string;
  user: AdminUserProfile | null;
}) {
  const canUseCatalog = canViewRoleCatalog && roles.length > 0;

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <ShieldCheck size={20} aria-hidden />
        <div>
          <h2>Platform role assignment</h2>
          <p>Assignments use the backend hierarchy checks before changing the target user.</p>
        </div>
        <StatusPill label={canAssign ? "Assign enabled" : "Assign gated"} tone={canAssign ? "good" : "neutral"} />
      </div>
      {!user ? <EmptyState text="Select a user before assigning a platform role." /> : null}
      {user && !canAssign ? <EmptyState text="This session is missing ASSIGN_ROLES_TO_USER." /> : null}
      {user && canAssign ? (
        <div className={styles.decisionForm}>
          <label>
            Role
            {canUseCatalog ? (
              <select onChange={(event) => onRoleNameChange(event.target.value)} value={roleName}>
                {roles.map((role) => (
                  <option key={role.id} value={role.name}>{role.name}</option>
                ))}
              </select>
            ) : (
              <input
                onChange={(event) => onRoleNameChange(event.target.value)}
                placeholder="Platform role name"
                value={roleName}
              />
            )}
          </label>
          {!canViewRoleCatalog ? <p className={styles.inlineNotice}>Role catalog requires VIEW_ROLE_ASSIGNMENTS.</p> : null}
          {rolesState === "error" && rolesError ? <p className={styles.inlineError}>{rolesError.message}</p> : null}
          {state === "success" ? <p className={styles.inlineNotice}>Role assigned successfully.</p> : null}
          {state === "error" && error ? <p className={styles.inlineError}>{error.message}</p> : null}
          <button className={styles.primaryButton} disabled={!roleName || state === "loading"} onClick={onAssign} type="button">
            {state === "loading" ? "Assigning" : "Assign role"}
          </button>
        </div>
      ) : null}
    </section>
  );
}
