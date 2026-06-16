"use client";

import { type OrganizationCourseListItem } from "@/lib/organization";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { Metric } from "./Metric";
import { StatusPill } from "./StatusPill";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";

export function OrganizationCourseCard({ course }: { course: OrganizationCourseListItem }) {
  const teacherNames = course.teachers.map((teacher) => teacher.name).join(", ");
  const rewardEvents = course.rewards.event_types.length
    ? course.rewards.event_types.map(formatUnderscoreLabel).join(", ")
    : "No active reward policy";

  return (
    <article className={styles.courseCard}>
      <div className={styles.sectionHeader}>
        <div>
          <h3>{course.title}</h3>
          <p className={styles.muted}>{teacherNames || "No teacher assigned"}</p>
        </div>
        <StatusPill label={formatUnderscoreLabel(course.lifecycle_status)} tone={course.lifecycle_status === "published" ? "good" : "neutral"} />
      </div>
      <div className={styles.metricGrid}>
        <Metric label="Chapters" value={course.content.chapter_count} />
        <Metric label="Lessons" value={course.content.content_count} />
        <Metric label="Students" value={course.roster.enrolled_student_count} />
      </div>
      <div className={styles.metricGrid}>
        <Metric label="Pending joins" value={course.roster.pending_join_request_count} />
        <Metric label="Reward review" value={course.reward_queue.pending_teacher_count} />
        <Metric label="Approved queue" value={course.reward_queue.teacher_approved_count} />
      </div>
      <div className={styles.compactList}>
        <div className={styles.compactRow}>
          <span>Reward status</span>
          <strong>{rewardEvents}</strong>
        </div>
        <div className={styles.compactRow}>
          <span>Content types</span>
          <strong>{course.content.content_types.length ? course.content.content_types.join(", ") : "None yet"}</strong>
        </div>
      </div>
      <div className={styles.permissionRow}>
        {course.permissions.can_manage_enrollments ? <span className={styles.permissionChip}>Can review joins</span> : null}
        {course.permissions.can_submit_reward_events ? <span className={styles.permissionChip}>Can submit rewards</span> : null}
        {course.permissions.can_create_courses ? <span className={styles.permissionChip}>Can create courses</span> : null}
        {!course.permissions.can_manage_enrollments &&
        !course.permissions.can_submit_reward_events &&
        !course.permissions.can_create_courses ? (
          <span className={styles.permissionChip}>View only</span>
        ) : null}
      </div>
    </article>
  );
}
