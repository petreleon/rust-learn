"use client";

import { ArrowLeft, BookOpen, FileText, ShieldCheck, Trophy, Users } from "lucide-react";
import Link from "next/link";
import { type TeacherCourseWorkspaceResponse } from "@/lib/teacher/TeacherCourseWorkspaceResponse";
import { ChapterList } from "@/components/teacher-routes/ChapterList";
import { DetailLine } from "@/components/teacher-routes/DetailLine";
import { PermissionChip } from "@/components/teacher-routes/PermissionChip";
import { SummaryCard } from "@/components/teacher-routes/SummaryCard";
import { statusLabel } from "@/components/teacher-routes/statusLabel";
import styles from "@/components/teacher-routes.module.css";
import { workspaceSummary } from "../model/workspaceSummary";
import { WorkspaceActionPanel } from "./WorkspaceActionPanel";

export function WorkspaceContent({ workspace }: { workspace: TeacherCourseWorkspaceResponse }) {
  const totals = workspaceSummary(workspace);
  const organizationNames =
    workspace.course.organizations.map((organization) => organization.name).join(", ") || "Personal course";
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
        <section className={styles.panel}>
          <div className={styles.panelHeader}>
            <ShieldCheck size={20} aria-hidden />
            <h2>Publication and ownership</h2>
          </div>
          <div className={styles.detailList}>
            <DetailLine label="Lifecycle" value={statusLabel(workspace.publication.course_lifecycle_status)} />
            <DetailLine label="Teacher roles" value={workspace.teacher_roles.join(", ") || "Delegated permission"} />
            <DetailLine label="Organizations" value={organizationNames} />
            <DetailLine
              label="Per-content publication"
              value={workspace.publication.content_publication_status_supported ? "Supported" : "Inherited from course"}
            />
          </div>
        </section>
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
