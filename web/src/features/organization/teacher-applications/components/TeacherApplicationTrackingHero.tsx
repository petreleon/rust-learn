"use client";

import { ArrowLeft, RefreshCw } from "lucide-react";
import Link from "next/link";
import { type OrganizationTeacherApplicationList } from "@/lib/organization/OrganizationTeacherApplicationList";
import { type OrganizationWorkspaceItem } from "@/lib/organization/OrganizationWorkspaceItem";
import { StatusPill } from "@/components/organization-routes/StatusPill";
import styles from "@/features/organization/shared/organization-routes.module.css";

export function TeacherApplicationTrackingHero({
  applications,
  onRefresh,
  organization,
}: {
  applications: OrganizationTeacherApplicationList;
  onRefresh: () => void;
  organization: OrganizationWorkspaceItem;
}) {
  const permissions = applications.operator_permissions;

  return (
    <section className={styles.workspaceHero}>
      <div className={styles.workspaceTitleBlock}>
        <Link className={styles.backLink} href={`/organizations/${organization.id}`}>
          <ArrowLeft size={17} aria-hidden />
          {organization.name}
        </Link>
        <p className={styles.eyebrow}>Teacher nominations</p>
        <h2>Sponsored application tracking</h2>
        <p className={styles.muted}>
          Applications load from the organization-scoped tracking contract. Nomination submission still needs a searchable
          applicant picker before it should become a normal operator form.
        </p>
      </div>
      <div className={styles.actionRow}>
        <button className={styles.secondaryButton} onClick={onRefresh} type="button">
          <RefreshCw size={17} aria-hidden />
          Refresh
        </button>
      </div>
      <div className={styles.permissionRow}>
        {permissions.can_view_applications ? <span className={styles.permissionChip}>Can view tracking</span> : null}
        {permissions.can_nominate_teachers ? <span className={styles.permissionChip}>Can nominate</span> : null}
        {!permissions.can_view_applications && !permissions.can_nominate_teachers ? (
          <span className={styles.permissionChip}>Tracking unavailable</span>
        ) : null}
        <StatusPill label={`${applications.summary.rejected} rejected`} tone={applications.summary.rejected ? "warn" : "neutral"} />
      </div>
    </section>
  );
}
