"use client";

import { BookOpen, FileText } from "lucide-react";
import { ProductShell } from "@/components/product-shell";
import { routeNotice } from "@/features/teacher/shared/route-kit/routeNotice";
import { StatusLine } from "@/features/teacher/shared/route-kit/StatusLine";
import { TeacherCourseContentPanels } from "@/features/teacher/shared/route-kit/TeacherCourseContentPanels";
import { workspaceSummary } from "@/features/teacher/shared/route-kit/workspaceSummary";
import { type TeacherCourseContentRouteController } from "../route/useTeacherCourseContentRoute";
import { AssessmentAuthoringPanel } from "./AssessmentAuthoringPanel";

export function TeacherCourseContentView({
  courseId,
  route,
}: {
  courseId: string;
  route: TeacherCourseContentRouteController;
}) {
  const courseTitle = route.workspace?.course.title || "Course content";
  return (
    <ProductShell
      activeNav="teach"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/teach", label: "Teach" },
        { href: "/teach/courses", label: "Courses" },
        { href: `/teach/courses/${courseId}`, label: courseTitle },
        { label: "Content" },
      ]}
      description="Structured chapter and text lesson authoring for course-scoped teachers."
      eyebrow="Teacher content"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={routeNotice(route.error)}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<CourseContentStatusItems route={route} />}
      title={courseTitle}
    >
      <TeacherCourseContentPanels
        actionMessage={route.actionMessage}
        actionState={route.actionState}
        assessmentPanel={
          route.workspace ? (
            <AssessmentAuthoringPanel
              actionState={route.actionState}
              assessmentActions={route.assessmentActions}
              canManageContent={route.workspace.course.permissions.can_manage_content}
            />
          ) : null
        }
        chapterDraft={route.chapterDraft}
        contentActions={route.contentActions}
        contentDraft={route.contentDraft}
        courseId={courseId}
        error={route.error}
        loadContentRoute={route.loadContentRoute}
        loadState={route.loadState}
        setChapterDraft={route.setChapterDraft}
        setContentDraft={route.setContentDraft}
        submitChapter={route.submitChapter}
        workspace={route.workspace}
      />
    </ProductShell>
  );
}

function CourseContentStatusItems({ route }: { route: TeacherCourseContentRouteController }) {
  if (!route.workspace) return null;

  const summary = workspaceSummary(route.workspace);

  return (
    <>
      <StatusLine
        icon={<BookOpen size={16} aria-hidden />}
        label={`${route.workspace.chapters.length} chapter${route.workspace.chapters.length === 1 ? "" : "s"}`}
        tone={route.workspace.chapters.length ? "good" : "warn"}
      />
      <StatusLine
        icon={<FileText size={16} aria-hidden />}
        label={`${summary.contentCount} content item${summary.contentCount === 1 ? "" : "s"}`}
        tone={summary.contentCount ? "good" : "warn"}
      />
    </>
  );
}
