"use client";

import { type OrganizationTeacherApplicationList } from "@/lib/organization/OrganizationTeacherApplicationList";
import { OrganizationTeacherApplicationCard } from "@/components/organization-routes/OrganizationTeacherApplicationCard";
import { StatusPill } from "@/components/organization-routes/StatusPill";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { teacherApplicationCounts } from "./applicationCounts";

export function TeacherApplicationListPanel({
  applications,
  onPageChange,
  page,
}: {
  applications: OrganizationTeacherApplicationList;
  onPageChange: (page: number) => void;
  page: number;
}) {
  const counts = teacherApplicationCounts(applications, page);

  return (
    <section className={styles.panel}>
      <div className={styles.sectionHeader}>
        <h2>Application list</h2>
        <StatusPill label={counts.pageLabel} tone="neutral" />
      </div>
      {applications.applications.length ? (
        <div className={styles.applicationGrid}>
          {applications.applications.map((application) => (
            <OrganizationTeacherApplicationCard application={application} key={application.id} />
          ))}
        </div>
      ) : (
        <p className={styles.muted}>
          No sponsored teacher applications match these filters. Reset filters or check whether the application was sponsored by
          another organization.
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
