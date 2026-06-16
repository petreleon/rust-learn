"use client";

import { ArrowLeft, RefreshCw } from "lucide-react";
import Link from "next/link";
import { type OrganizationWorkspaceItem } from "@/lib/organization/OrganizationWorkspaceItem";
import styles from "@/components/organization-routes.module.css";

export function CourseDirectoryHero({
  onRefresh,
  organization,
}: {
  onRefresh: () => void;
  organization: OrganizationWorkspaceItem;
}) {
  return (
    <section className={styles.workspaceHero}>
      <div className={styles.workspaceTitleBlock}>
        <Link className={styles.backLink} href={`/organizations/${organization.id}`}>
          <ArrowLeft size={17} aria-hidden />
          {organization.name}
        </Link>
        <p className={styles.eyebrow}>Organization courses</p>
        <h2>Sponsored course workspace</h2>
        <p className={styles.muted}>
          Courses load from the organization-scoped course contract. Editing, publishing, and organization-course ownership
          changes remain separate route work.
        </p>
      </div>
      <div className={styles.actionRow}>
        <button className={styles.secondaryButton} onClick={onRefresh} type="button">
          <RefreshCw size={17} aria-hidden />
          Refresh
        </button>
      </div>
    </section>
  );
}
