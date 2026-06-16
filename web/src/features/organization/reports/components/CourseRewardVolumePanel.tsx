"use client";

import { type OrganizationCourseRewardDashboardRow } from "@/lib/organization/OrganizationCourseRewardDashboardRow";
import { Metric } from "@/components/organization-routes/Metric";
import { StatusPill } from "@/components/organization-routes/StatusPill";
import { formatTokenAmount } from "@/components/organization-routes/formatTokenAmount";
import styles from "@/components/organization-routes.module.css";

export function CourseRewardVolumePanel({
  courses,
}: {
  courses: OrganizationCourseRewardDashboardRow[];
}) {
  const hasCourseRows = courses.length > 0;

  return (
    <section className={styles.panel}>
      <div className={styles.sectionHeader}>
        <h2>Course reward volume</h2>
        <StatusPill label={`${courses.length} course${courses.length === 1 ? "" : "s"}`} tone={hasCourseRows ? "good" : "neutral"} />
      </div>
      {hasCourseRows ? (
        <div className={styles.reportGrid}>
          {courses.map((course) => (
            <article className={styles.reportCard} key={course.course_id}>
              <h3>{course.course_title}</h3>
              <div className={styles.metricGrid}>
                <Metric label="Candidates" value={course.reward_candidate_count} />
                <Metric label="Approved" value={course.approved_reward_count} />
                <Metric label="Amount" value={formatTokenAmount(course.approved_amount_total)} />
              </div>
            </article>
          ))}
        </div>
      ) : (
        <p className={styles.muted}>
          No course reward rows are available yet. The CSV export will still include the summary headers for this organization.
        </p>
      )}
    </section>
  );
}
