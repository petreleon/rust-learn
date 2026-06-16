"use client";

import { AlertTriangle, Building2, CheckCircle2, Loader2, ShieldCheck, Users } from "lucide-react";
import { type OrganizationMemberList } from "@/lib/organization/OrganizationMemberList";
import { type OrganizationWorkspaceItem } from "@/lib/organization/OrganizationWorkspaceItem";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/components/organization-routes.module.css";
import { MemberErrorState } from "@/components/organization-routes/MemberErrorState";
import { SummaryCard } from "@/components/organization-routes/SummaryCard";
import { type MemberActionState } from "../model/MemberActionState";
import { MemberDirectoryHero } from "./MemberDirectoryHero";
import { MemberFilterPanel } from "./MemberFilterPanel";
import { MemberListPanel } from "./MemberListPanel";
import { memberDirectoryCounts } from "./memberCounts";

type Props = {
  assignRoleMessage: string | null;
  assignRoleState: MemberActionState;
  canAssignRoles: boolean;
  canManageMembers: boolean;
  draftSearch: string;
  loadState: LoadState;
  members: OrganizationMemberList | null;
  onApplyFilters: () => void;
  onAssignRole: (memberId: number, roleName: string) => void;
  onDraftSearchChange: (value: string) => void;
  onPageChange: (page: number) => void;
  onPermissionFilterChange: (value: string) => void;
  onRefresh: () => void;
  onRemoveMember: (memberId: number) => void;
  onResetFilters: () => void;
  onRoleFilterChange: (value: string) => void;
  organization: OrganizationWorkspaceItem;
  page: number;
  permissionFilter: string;
  removeMemberMessage: string | null;
  roleFilter: string;
  routeError: RouteError | null;
};

export function OrganizationMembersContent(props: Props) {
  if (props.loadState === "loading" || props.loadState === "idle") {
    return <MemberLoadingState />;
  }

  if (props.loadState === "error") {
    return <MemberErrorState error={props.routeError} onRetry={props.onRefresh} />;
  }

  if (!props.members) {
    return null;
  }

  const counts = memberDirectoryCounts(props.members, props.page);

  return (
    <>
      <MemberDirectoryHero members={props.members} onRefresh={props.onRefresh} organization={props.organization} />
      <MemberActionMessage message={props.assignRoleMessage || props.removeMemberMessage} state={props.assignRoleState} />
      <section className={styles.summaryGrid}>
        <SummaryCard icon={<Users size={20} aria-hidden />} label="Matching members" value={props.members.total} />
        <SummaryCard icon={<ShieldCheck size={20} aria-hidden />} label="Delegated access" value={counts.delegatedCount} />
        <SummaryCard icon={<CheckCircle2 size={20} aria-hidden />} label="Verified emails" value={counts.verifiedEmailCount} />
        <SummaryCard icon={<Building2 size={20} aria-hidden />} label="KYC ready" value={counts.kycReadyCount} />
      </section>
      <MemberFilterPanel
        draftSearch={props.draftSearch}
        onApplyFilters={props.onApplyFilters}
        onDraftSearchChange={props.onDraftSearchChange}
        onPermissionFilterChange={props.onPermissionFilterChange}
        onResetFilters={props.onResetFilters}
        onRoleFilterChange={props.onRoleFilterChange}
        permissionFilter={props.permissionFilter}
        roleFilter={props.roleFilter}
      />
      <MemberListPanel
        canAssignRoles={props.canAssignRoles}
        canManageMembers={props.canManageMembers}
        members={props.members}
        onAssignRole={props.onAssignRole}
        onPageChange={props.onPageChange}
        onRemoveMember={props.onRemoveMember}
        page={props.page}
      />
    </>
  );
}

function MemberLoadingState() {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
      <div className={styles.panelHeader}>
        <Loader2 className={styles.spin} size={20} aria-hidden />
        <h2>Loading organization members</h2>
      </div>
      <div className={styles.skeletonGrid} aria-hidden>
        <div className={styles.skeleton} />
        <div className={styles.skeleton} />
        <div className={styles.skeleton} />
      </div>
    </section>
  );
}

function MemberActionMessage({ message, state }: { message: string | null; state: MemberActionState }) {
  if (!message) {
    return null;
  }

  return (
    <section className={`${styles.warningPanel} ${styles.singlePanel}`} role="status">
      <div className={styles.panelHeader}>
        {state === "success" ? <CheckCircle2 size={18} aria-hidden /> : <AlertTriangle size={18} aria-hidden />}
        <h2>{state === "success" ? "Role assigned" : "Member action"}</h2>
      </div>
      <p>{message}</p>
    </section>
  );
}
