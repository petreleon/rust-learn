"use client";

import { BriefcaseBusiness, FileText, Trophy, Users } from "lucide-react";
import Link from "next/link";
import { type TeacherCourseWorkspaceResponse } from "@/lib/teacher/TeacherCourseWorkspaceResponse";
import styles from "@/features/teacher/shared/teacher-routes.module.css";

export function WorkspaceActionPanel({ workspace }: { workspace: TeacherCourseWorkspaceResponse }) {
  const canViewStudents =
    workspace.course.permissions.can_manage_enrollments ||
    workspace.course.permissions.can_view_reward_candidates ||
    workspace.course.permissions.can_approve_reward_candidates;
  const canViewRewards =
    workspace.course.permissions.can_view_reward_candidates ||
    workspace.course.permissions.can_approve_reward_candidates;
  const actions = [
    {
      detail: workspace.course.permissions.can_manage_content
        ? "Create chapters and text lessons from structured forms; upload and destructive editing stay separate."
        : "This session cannot manage course content.",
      enabled: workspace.course.permissions.can_manage_content,
      href: `/teach/courses/${workspace.course.id}/content`,
      icon: <FileText size={17} aria-hidden />,
      label: "Content authoring",
    },
    {
      detail: workspace.course.permissions.can_manage_enrollments
        ? "Review join requests and roster access from the enrollment workspace."
        : "This session cannot manage enrollment decisions.",
      enabled: workspace.course.permissions.can_manage_enrollments,
      href: `/teach/courses/${workspace.course.id}/enrollments`,
      icon: <Users size={17} aria-hidden />,
      label: "Enrollment queue",
    },
    {
      detail: canViewStudents
        ? "Review enrolled learners, progress support, and reward evidence."
        : "This session cannot view course student progress.",
      enabled: canViewStudents,
      href: `/teach/courses/${workspace.course.id}/students`,
      icon: <Users size={17} aria-hidden />,
      label: "Student progress",
    },
    {
      detail: canViewRewards
        ? "Review course-scoped reward evidence without platform payout controls."
        : "This session cannot view student reward candidates.",
      enabled: canViewRewards,
      href: `/teach/courses/${workspace.course.id}/rewards`,
      icon: <Trophy size={17} aria-hidden />,
      label: "Reward review",
    },
  ];

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <BriefcaseBusiness size={20} aria-hidden />
        <h2>Workspace actions</h2>
      </div>
      <div className={styles.priorityList}>
        {actions.map((action) => (
          <article className={styles.priorityItem} key={action.label}>
            <span className={`${styles.smallIcon} ${action.enabled ? styles.good : styles.neutral}`}>
              {action.icon}
            </span>
            <div>
              <strong>{action.label}</strong>
              <p>{action.detail}</p>
              {action.enabled && action.href ? (
                <Link className={styles.secondaryLink} href={action.href}>
                  {action.icon}
                  Open
                </Link>
              ) : null}
            </div>
          </article>
        ))}
      </div>
    </section>
  );
}
