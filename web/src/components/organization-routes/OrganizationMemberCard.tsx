"use client";

import { UserMinus } from "lucide-react";
import { useState } from "react";
import { type OrganizationMemberListItem } from "@/lib/organization";
import styles from "../organization-routes.module.css";
import { AssignRoleControl } from "./AssignRoleControl";
import { Metric } from "./Metric";
import { StatusPill } from "./StatusPill";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";

export function OrganizationMemberCard({
  canAssignRoles,
  canManageMembers,
  member,
  onAssignRole,
  onRemoveMember,
}: {
  canAssignRoles?: boolean;
  canManageMembers?: boolean;
  member: OrganizationMemberListItem;
  onAssignRole?: (memberId: number, roleName: string) => void;
  onRemoveMember?: (memberId: number) => void;
}) {
  const previewPermissions = member.effective_permissions.slice(0, 4);
  const remainingPermissions = member.effective_permissions.length - previewPermissions.length;
  const [removeConfirm, setRemoveConfirm] = useState(false);

  return (
    <article className={styles.memberCard}>
      <div className={styles.sectionHeader}>
        <div>
          <h3>{member.name}</h3>
          <p className={styles.muted}>{member.email}</p>
        </div>
        <StatusPill label={member.roles[0] ? formatUnderscoreLabel(member.roles[0]) : "No role"} tone="neutral" />
      </div>
      <div className={styles.metricGrid}>
        <Metric label="Direct" value={member.direct_permission_count} />
        <Metric label="Delegated" value={member.delegated_permission_count} />
        <Metric label="Effective" value={member.effective_permission_count} />
      </div>
      <div className={styles.compactList}>
        <div className={styles.compactRow}>
          <span>Email</span>
          <strong>{member.email_verified ? "Verified" : "Pending"}</strong>
        </div>
        <div className={styles.compactRow}>
          <span>KYC</span>
          <strong>{member.kyc_verified ? "Verified" : "Pending"}</strong>
        </div>
        <div className={styles.compactRow}>
          <span>Joined</span>
          <strong>{new Date(member.joined_at).toLocaleDateString()}</strong>
        </div>
      </div>
      {previewPermissions.length ? (
        <div className={styles.compactList}>
          <div className={styles.compactRow}>
            <span>Permissions</span>
            <div>
              {previewPermissions.map((p) => (
                <span className={styles.permissionChip} key={p}>{p}</span>
              ))}
              {remainingPermissions > 0 ? <span className={styles.permissionChip}>+{remainingPermissions}</span> : null}
            </div>
          </div>
        </div>
      ) : null}
      {member.roles.length > 1 ? (
        <div className={styles.compactList}>
          <div className={styles.compactRow}>
            <span>Other roles</span>
            <div>
              {member.roles.slice(1).map((role) => (
                <span className={styles.statusPill} key={role}>{formatUnderscoreLabel(role)}</span>
              ))}
            </div>
          </div>
        </div>
      ) : null}
      {canAssignRoles && onAssignRole ? (
        <AssignRoleControl memberId={member.id} onAssign={onAssignRole} />
      ) : null}
      {canManageMembers && onRemoveMember ? (
        <div className={styles.actionRow} style={{ gap: "0.5rem", marginTop: "0.5rem" }}>
          {removeConfirm ? (
            <>
              <button
                className={styles.primaryButton}
                onClick={() => onRemoveMember(member.id)}
                style={{ background: "var(--color-warn, #dc2626)", borderColor: "var(--color-warn, #dc2626)" }}
                type="button"
              >
                <UserMinus size={17} aria-hidden />
                Confirm
              </button>
              <button className={styles.secondaryButton} onClick={() => setRemoveConfirm(false)} type="button">
                Cancel
              </button>
            </>
          ) : (
            <button
              className={styles.secondaryButton}
              onClick={() => setRemoveConfirm(true)}
              style={{ color: "var(--color-warn, #dc2626)" }}
              type="button"
            >
              <UserMinus size={17} aria-hidden />
              Remove
            </button>
          )}
        </div>
      ) : null}
    </article>
  );
}
