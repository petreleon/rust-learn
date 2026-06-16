"use client";

import { type OrganizationCourseList } from "@/lib/organization/OrganizationCourseList";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { OrganizationCourseCard } from "@/components/organization-routes/OrganizationCourseCard";
import { StatusPill } from "@/components/organization-routes/StatusPill";
import { organizationCourseCounts } from "./courseCounts";

export function CourseListPanel({
  courses,
  onPageChange,
  page,
}: {
  courses: OrganizationCourseList;
  onPageChange: (page: number) => void;
  page: number;
}) {
  const counts = organizationCourseCounts(courses, page);

  return (
    <section className={styles.panel}>
      <div className={styles.sectionHeader}>
        <h2>Course list</h2>
        <StatusPill label={counts.pageLabel} tone="neutral" />
      </div>
      {courses.courses.length ? (
        <div className={styles.courseGrid}>
          {courses.courses.map((course) => (
            <OrganizationCourseCard course={course} key={course.id} />
          ))}
        </div>
      ) : (
        <p className={styles.muted}>
          No organization courses match these filters. Reset filters or check whether the course is attached to this organization.
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
