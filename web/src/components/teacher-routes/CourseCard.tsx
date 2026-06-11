"use client";

import { BriefcaseBusiness, Trophy, Users } from "lucide-react";
import Link from "next/link";
import { type TeacherCourseDashboardItem } from "@/lib/teacher";
import styles from "../teacher-routes.module.css";
import { DetailLine } from "./DetailLine";
import { Metric } from "./Metric";
import { PermissionChip } from "./PermissionChip";
import { lifecycleTone } from "./lifecycleTone";
import { statusLabel } from "./statusLabel";

export function CourseCard({ course }: { course: TeacherCourseDashboardItem }) {
  const pendingCount = course.roster.pending_join_request_count + course.roster.waitlisted_join_request_count;
  const statusTone = lifecycleTone(course.lifecycle_status);
  const canViewStudents =
    course.permissions.can_manage_enrollments ||
    course.permissions.can_view_reward_candidates ||
    course.permissions.can_approve_reward_candidates;
  const canViewRewards =
    course.permissions.can_view_reward_candidates ||
    course.permissions.can_approve_reward_candidates;
  return (
    <article className={styles.courseCard}>
      <div className={styles.courseTop}>
        <div>
          <p className={styles.eyebrow}>{course.organizations.map((organization) => organization.name).join(", ") || "Personal course"}</p>
          <h3>{course.title}</h3>
        </div>
        <span className={`${styles.statusPill} ${styles[statusTone]}`}>{statusLabel(course.lifecycle_status)}</span>
      </div>

      <div className={styles.metricGrid}>
        <Metric label="Learners" value={course.roster.enrolled_student_count} />
        <Metric label="Enrollment" value={pendingCount} tone={pendingCount ? "warn" : "neutral"} />
        <Metric label="Rewards" value={course.reward_queue.pending_teacher_count} tone={course.reward_queue.pending_teacher_count ? "warn" : "neutral"} />
        <Metric label="Content" value={course.content.content_count} tone={course.content.has_content ? "neutral" : "warn"} />
      </div>

      <div className={styles.detailList}>
        <DetailLine label="Chapters" value={String(course.content.chapter_count)} />
        <DetailLine label="Reward policies" value={course.rewards.available ? `${course.rewards.active_policy_count} active` : "No active policy"} />
        <DetailLine label="Teacher approved" value={String(course.reward_queue.teacher_approved_count)} />
        <DetailLine label="Failed rewards" value={String(course.reward_queue.failed_count)} />
      </div>

      <div className={styles.permissionRow} aria-label="Course actions">
        <PermissionChip enabled={course.permissions.can_manage_content} label="Content" />
        <PermissionChip enabled={course.permissions.can_manage_enrollments} label="Enrollments" />
        <PermissionChip enabled={canViewRewards} label="Rewards" />
        <PermissionChip enabled={course.permissions.can_manage_settings} label="Settings" />
      </div>

      <div className={styles.actionRow}>
        <Link className={styles.secondaryLink} href={`/teach/courses/${course.id}`}>
          <BriefcaseBusiness size={16} aria-hidden />
          Workspace
        </Link>
        {course.permissions.can_manage_enrollments ? (
          <Link className={styles.secondaryLink} href={`/teach/courses/${course.id}/enrollments`}>
            <Users size={16} aria-hidden />
            Enrollments
          </Link>
        ) : (
          <button className={styles.secondaryButton} disabled type="button" title="Enrollment permission required">
            <Users size={16} aria-hidden />
            Enrollments
          </button>
        )}
        {canViewStudents ? (
          <Link className={styles.secondaryLink} href={`/teach/courses/${course.id}/students`}>
            <Users size={16} aria-hidden />
            Students
          </Link>
        ) : null}
        {canViewRewards ? (
          <Link className={styles.secondaryLink} href={`/teach/courses/${course.id}/rewards`}>
            <Trophy size={16} aria-hidden />
            Rewards
          </Link>
        ) : (
          <button className={styles.secondaryButton} disabled type="button" title="Reward candidate permission required">
            <Trophy size={16} aria-hidden />
            Rewards
          </button>
        )}
      </div>
    </article>
  );
}
