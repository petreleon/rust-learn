"use client";

import { ArrowRight, UserPlus, Users } from "lucide-react";
import Link from "next/link";
import styles from "@/features/organization/shared/organization-routes.module.css";

export function MemberAccessWorkflowPanel({
  canInviteMembers,
  organizationId,
}: {
  canInviteMembers: boolean;
  organizationId: number;
}) {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} aria-label="Member access workflow">
      <div className={styles.panelHeader}>
        <UserPlus size={20} aria-hidden />
        <h2>Member access workflow</h2>
      </div>
      <p className={styles.muted}>
        Add-by-email grants access to an existing RustLearn account immediately. This organization
        does not keep a separate pending invite queue in the current contract.
      </p>
      <p className={styles.muted}>
        Pending learner joins are tracked on organization courses, where operators can review
        enrollment pressure before updating course membership.
      </p>
      <div className={styles.permissionRow}>
        <span className={styles.permissionChip}>{canInviteMembers ? "Can add members" : "Invite permission needed"}</span>
        <span className={styles.permissionChip}>Pending joins live on courses</span>
      </div>
      <Link className={styles.secondaryLink} href={`/organizations/${organizationId}/courses`}>
        <Users size={17} aria-hidden />
        Review course joins
        <ArrowRight size={17} aria-hidden />
      </Link>
    </section>
  );
}
