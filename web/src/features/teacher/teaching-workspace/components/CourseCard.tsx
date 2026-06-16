"use client";

import { BriefcaseBusiness, Trophy, Users } from "lucide-react";
import Link from "next/link";
import { type TeacherCourseDashboardItem } from "@/lib/teacher/TeacherCourseDashboardItem";
import { DetailLine } from "@/features/teacher/shared/route-kit/DetailLine";
import { Metric } from "@/features/teacher/shared/route-kit/Metric";
import { PermissionChip } from "@/features/teacher/shared/route-kit/PermissionChip";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { lifecycleTone, organizationNames, statusLabel } from "../model/courseDisplay";

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
          <p className={styles.eyebrow}>{organizationNames(course.organizations.map((item) => item.name))}</p>
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
      <CourseDetails course={course} />
      <CoursePermissions canViewRewards={canViewRewards} course={course} />
      <CourseActions canViewRewards={canViewRewards} canViewStudents={canViewStudents} course={course} />
    </article>
  );
}

function CourseDetails({ course }: { course: TeacherCourseDashboardItem }) {
  return (
    <div className={styles.detailList}>
      <DetailLine label="Chapters" value={String(course.content.chapter_count)} />
      <DetailLine label="Reward policies" value={course.rewards.available ? `${course.rewards.active_policy_count} active` : "No active policy"} />
      <DetailLine label="Teacher approved" value={String(course.reward_queue.teacher_approved_count)} />
      <DetailLine label="Failed rewards" value={String(course.reward_queue.failed_count)} />
    </div>
  );
}

function CoursePermissions({
  canViewRewards,
  course,
}: {
  canViewRewards: boolean;
  course: TeacherCourseDashboardItem;
}) {
  return (
    <div className={styles.permissionRow} aria-label="Course actions">
      <PermissionChip enabled={course.permissions.can_manage_content} label="Content" />
      <PermissionChip enabled={course.permissions.can_manage_enrollments} label="Enrollments" />
      <PermissionChip enabled={canViewRewards} label="Rewards" />
      <PermissionChip enabled={course.permissions.can_manage_settings} label="Settings" />
    </div>
  );
}

function CourseActions({
  canViewRewards,
  canViewStudents,
  course,
}: {
  canViewRewards: boolean;
  canViewStudents: boolean;
  course: TeacherCourseDashboardItem;
}) {
  return (
    <div className={styles.actionRow}>
      <Link className={styles.secondaryLink} href={`/teach/courses/${course.id}`}>
        <BriefcaseBusiness size={16} aria-hidden />
        Workspace
      </Link>
      <EnrollmentAction course={course} />
      {canViewStudents ? (
        <Link className={styles.secondaryLink} href={`/teach/courses/${course.id}/students`}>
          <Users size={16} aria-hidden />
          Students
        </Link>
      ) : null}
      <RewardAction canViewRewards={canViewRewards} course={course} />
    </div>
  );
}

function EnrollmentAction({ course }: { course: TeacherCourseDashboardItem }) {
  if (course.permissions.can_manage_enrollments) {
    return (
      <Link className={styles.secondaryLink} href={`/teach/courses/${course.id}/enrollments`}>
        <Users size={16} aria-hidden />
        Enrollments
      </Link>
    );
  }

  return (
    <button className={styles.secondaryButton} disabled type="button" title="Enrollment permission required">
      <Users size={16} aria-hidden />
      Enrollments
    </button>
  );
}

function RewardAction({
  canViewRewards,
  course,
}: {
  canViewRewards: boolean;
  course: TeacherCourseDashboardItem;
}) {
  if (canViewRewards) {
    return (
      <Link className={styles.secondaryLink} href={`/teach/courses/${course.id}/rewards`}>
        <Trophy size={16} aria-hidden />
        Rewards
      </Link>
    );
  }

  return (
    <button className={styles.secondaryButton} disabled type="button" title="Reward candidate permission required">
      <Trophy size={16} aria-hidden />
      Rewards
    </button>
  );
}
