"use client";

import { RefreshCw, ArrowLeft } from "lucide-react";
import Link from "next/link";
import { type OrganizationMemberList } from "@/lib/organization/OrganizationMemberList";
import { type OrganizationWorkspaceItem } from "@/lib/organization/OrganizationWorkspaceItem";
import styles from "@/components/organization-routes.module.css";

export function MemberDirectoryHero({
  members,
  onRefresh,
  organization,
}: {
  members: OrganizationMemberList;
  onRefresh: () => void;
  organization: OrganizationWorkspaceItem;
}) {
  const permissions = members.operator_permissions;
  const isViewOnly = !permissions.can_invite_members && !permissions.can_manage_members && !permissions.can_assign_roles;

  return (
    <section className={styles.workspaceHero}>
      <div className={styles.workspaceTitleBlock}>
        <Link className={styles.backLink} href={`/organizations/${organization.id}`}>
          <ArrowLeft size={17} aria-hidden />
          {organization.name}
        </Link>
        <p className={styles.eyebrow}>Organization members</p>
        <h2>Member directory</h2>
        <p className={styles.muted}>
          Members load from the organization-scoped directory contract. Invite, role-change, removal, and audit-history actions
          remain separate route work.
        </p>
      </div>
      <div className={styles.actionRow}>
        <button className={styles.secondaryButton} onClick={onRefresh} type="button">
          <RefreshCw size={17} aria-hidden />
          Refresh
        </button>
      </div>
      <div className={styles.permissionRow}>
        {permissions.can_invite_members ? <span className={styles.permissionChip}>Can invite</span> : null}
        {permissions.can_manage_members ? <span className={styles.permissionChip}>Can manage members</span> : null}
        {permissions.can_assign_roles ? <span className={styles.permissionChip}>Can assign roles</span> : null}
        {isViewOnly ? <span className={styles.permissionChip}>View-only directory</span> : null}
      </div>
    </section>
  );
}
