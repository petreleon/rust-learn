"use client";

import { ArrowLeft, BookOpen, FileText, Trophy, Users } from "lucide-react";
import Link from "next/link";
import { type TeacherCourseWorkspaceResponse } from "@/lib/teacher/TeacherCourseWorkspaceResponse";
import { ChapterList } from "@/features/teacher/shared/route-kit/ChapterList";
import { PermissionChip } from "@/features/teacher/shared/route-kit/PermissionChip";
import { SummaryCard } from "@/features/teacher/shared/route-kit/SummaryCard";
import { statusLabel } from "@/features/teacher/shared/route-kit/statusLabel";
import { type ActionState } from "@/shared/route-state/ActionState";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import { workspaceSummary } from "../model/workspaceSummary";
import { type CourseSettingsDraft } from "../model/courseSettingsModel";
import { CourseSettingsPanel } from "./CourseSettingsPanel";
import { WorkspaceActionPanel } from "./WorkspaceActionPanel";

export function WorkspaceContent({
  courseAction,
  workspace,
}: {
  courseAction: {
    actionMessage: string | null;
    actionState: ActionState;
    submitCourseLifecycle: (status: string) => void;
    submitCourseSettings: (draft: CourseSettingsDraft) => void;
  };
  workspace: TeacherCourseWorkspaceResponse;
}) {
  const totals = workspaceSummary(workspace);
  const organizationNames =
    workspace.course.organizations.map((organization) => organization.name).join(", ") || "Personal course";
  const settingsKey = [
    workspace.course.id,
    workspace.course.title,
    workspace.course.lifecycle_status,
    workspace.course.description,
    workspace.course.topics,
    workspace.course.prerequisites,
  ].join(":");
  const canViewRewards =
    workspace.course.permissions.can_view_reward_candidates ||
    workspace.course.permissions.can_approve_reward_candidates;
  return (
    <>
      <section className={styles.workspaceHero}>
        <Link className={styles.secondaryLink} href="/teach/courses">
          <ArrowLeft size={17} aria-hidden />
          Teaching courses
        </Link>
        <div className={styles.workspaceTitleBlock}>
          <p className={styles.eyebrow}>{organizationNames}</p>
          <h2>{workspace.course.title}</h2>
          <p className={styles.muted}>
            Course lifecycle is {statusLabel(workspace.course.lifecycle_status)}. Individual content publication state
            is not stored yet, so content inherits the course lifecycle.
          </p>
        </div>
        <div className={styles.permissionRow} aria-label="Workspace permissions">
          <PermissionChip enabled={workspace.course.permissions.can_manage_settings} label="Settings" />
          <PermissionChip enabled={workspace.course.permissions.can_manage_content} label="Content" />
          <PermissionChip enabled={workspace.course.permissions.can_manage_enrollments} label="Enrollments" />
          <PermissionChip enabled={canViewRewards} label="Rewards" />
        </div>
      </section>

      <section className={styles.summaryGrid}>
        <SummaryCard
          icon={<FileText size={20} aria-hidden />}
          label="Content items"
          tone={totals.contentCount ? "neutral" : "warn"}
          value={totals.contentCount}
        />
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Chapters" value={workspace.chapters.length} />
        <SummaryCard
          icon={<Users size={20} aria-hidden />}
          label="Enrollment requests"
          tone={pendingEnrollmentCount(workspace) ? "warn" : "neutral"}
          value={pendingEnrollmentCount(workspace)}
        />
        <SummaryCard
          icon={<Trophy size={20} aria-hidden />}
          label="Reward reviews"
          tone={workspace.course.reward_queue.pending_teacher_count ? "warn" : "neutral"}
          value={workspace.course.reward_queue.pending_teacher_count}
        />
      </section>

      <section className={styles.twoColumn}>
        <WorkspaceActionPanel workspace={workspace} />
        <CourseSettingsPanel
          actionMessage={courseAction.actionMessage}
          actionState={courseAction.actionState}
          key={settingsKey}
          onLifecycleSubmit={courseAction.submitCourseLifecycle}
          onSettingsSubmit={courseAction.submitCourseSettings}
          organizationNames={organizationNames}
          workspace={workspace}
        />
      </section>

      <section className={styles.courseSection}>
        <div className={styles.sectionHeader}>
          <div>
            <h2>Course content</h2>
            <p className={styles.muted}>
              Structured chapters, content types, stored-data presence, and latest processing state.
            </p>
          </div>
        </div>
        <ChapterList chapters={workspace.chapters} />
      </section>
    </>
  );
}

function pendingEnrollmentCount(workspace: TeacherCourseWorkspaceResponse) {
  return (
    workspace.course.roster.pending_join_request_count +
    workspace.course.roster.waitlisted_join_request_count
  );
}
