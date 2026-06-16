"use client";

import { type OrganizationMemberList } from "@/lib/organization/OrganizationMemberList";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { OrganizationMemberCard } from "@/components/organization-routes/OrganizationMemberCard";
import { StatusPill } from "@/components/organization-routes/StatusPill";
import { memberDirectoryCounts } from "./memberCounts";

export function MemberListPanel({
  canAssignRoles,
  canManageMembers,
  members,
  onAssignRole,
  onPageChange,
  onRemoveMember,
  page,
}: {
  canAssignRoles: boolean;
  canManageMembers: boolean;
  members: OrganizationMemberList;
  onAssignRole: (memberId: number, roleName: string) => void;
  onPageChange: (page: number) => void;
  onRemoveMember: (memberId: number) => void;
  page: number;
}) {
  const counts = memberDirectoryCounts(members, page);

  return (
    <section className={styles.panel}>
      <div className={styles.sectionHeader}>
        <h2>Member list</h2>
        <StatusPill label={counts.pageLabel} tone="neutral" />
      </div>
      {members.members.length ? (
        <div className={styles.memberGrid}>
          {members.members.map((member) => (
            <OrganizationMemberCard
              canAssignRoles={canAssignRoles}
              canManageMembers={canManageMembers}
              key={member.id}
              member={member}
              onAssignRole={onAssignRole}
              onRemoveMember={onRemoveMember}
            />
          ))}
        </div>
      ) : (
        <p className={styles.muted}>
          No organization members match these filters. Reset filters or check whether the member still has a role in this
          organization.
        </p>
      )}
      <div className={styles.paginationRow}>
        <button className={styles.secondaryButton} disabled={!counts.canGoBack} onClick={() => onPageChange(Math.max(0, page - 1))} type="button">
          Previous
        </button>
        <span>{counts.rangeLabel}</span>
        <button className={styles.secondaryButton} disabled={!counts.canGoForward} onClick={() => onPageChange(page + 1)} type="button">
          Next
        </button>
      </div>
    </section>
  );
}
