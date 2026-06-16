"use client";

import { BookOpen, FileText } from "lucide-react";
import { useCallback, useEffect, useState } from "react";
import { ProductShell } from "@/components/product-shell";
import { clearStoredSessionToken, fetchCurrentSession, readStoredSessionToken, type CurrentSession } from "@/lib/session";
import { fetchTeachingCourseWorkspace, type TeacherCourseWorkspaceResponse } from "@/lib/teacher";
import { type ActionState } from "@/components/teacher-routes/ActionState";
import { type ChapterDraft } from "@/components/teacher-routes/ChapterDraft";
import { createChapterSubmitAction } from "@/components/teacher-routes/chapterSubmitAction";
import { type ContentDraft } from "@/components/teacher-routes/ContentDraft";
import { defaultChapterDraft } from "@/components/teacher-routes/defaultChapterDraft";
import { defaultContentDraft } from "@/components/teacher-routes/defaultContentDraft";
import { type LoadState } from "@/components/teacher-routes/LoadState";
import { normalizeRouteError } from "@/components/teacher-routes/normalizeRouteError";
import { type RouteError } from "@/components/teacher-routes/RouteError";
import { routeNotice } from "@/components/teacher-routes/routeNotice";
import { StatusLine } from "@/components/teacher-routes/StatusLine";
import { TeacherCourseContentPanels } from "@/components/teacher-routes/TeacherCourseContentPanels";
import { useContentAuthoringActions } from "@/components/teacher-routes/useContentAuthoringActions";
import { workspaceSummary } from "@/components/teacher-routes/workspaceSummary";

export function TeacherCourseContentRoute({ courseId }: { courseId: string }) {
  const [actionMessage, setActionMessage] = useState<string | null>(null);
  const [actionState, setActionState] = useState<ActionState>("idle");
  const [chapterDraft, setChapterDraft] = useState<ChapterDraft>(defaultChapterDraft);
  const [contentDraft, setContentDraft] = useState<ContentDraft>(defaultContentDraft);
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [workspace, setWorkspace] = useState<TeacherCourseWorkspaceResponse | null>(null);

  const loadContentRoute = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setWorkspace(null);
      setError(null);
      setLoadState("idle");
      return;
    }
    setHasToken(true);
    setLoadState("loading");
    setError(null);
    try {
      const [nextSession, nextWorkspace] = await Promise.all([
        fetchCurrentSession({ token }),
        fetchTeachingCourseWorkspace({ courseId, token }),
      ]);
      setSession(nextSession);
      setWorkspace(nextWorkspace);
      setContentDraft((current) => ({
        ...current,
        chapterId: current.chapterId || nextWorkspace.chapters[0]?.id.toString() || "",
      }));
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setSession(null);
      setWorkspace(null);
      setError(routeError);
      setLoadState("error");
    }
  }, [courseId]);

  const contentActions = useContentAuthoringActions({
    contentDraft,
    courseId,
    loadContentRoute,
    setActionMessage,
    setActionState,
    setContentDraft,
    workspace,
  });
  const isDraftDirty = chapterDraft.title.trim() !== "" || contentActions.isContentDraftDirty;
  const summary = workspace ? workspaceSummary(workspace) : null;
  const submitChapter = createChapterSubmitAction({
    chapterDraft,
    courseId,
    loadContentRoute,
    setActionMessage,
    setActionState,
    setChapterDraft,
  });

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadContentRoute(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadContentRoute]);

  useEffect(() => {
    if (loadState !== "success" || !workspace) return;
    const hasProcessing = workspace.chapters.some((chapter) =>
      chapter.contents.some((content) => content.display_state === "processing"),
    );
    if (!hasProcessing) return;
    const interval = window.setInterval(() => void loadContentRoute(), 30000);
    return () => window.clearInterval(interval);
  }, [loadState, workspace, loadContentRoute]);

  useEffect(() => {
    const handleBeforeUnload = (e: BeforeUnloadEvent) => {
      if (isDraftDirty && actionState !== "saving") {
        e.preventDefault();
        e.returnValue = "";
      }
    };
    window.addEventListener("beforeunload", handleBeforeUnload);
    return () => window.removeEventListener("beforeunload", handleBeforeUnload);
  }, [isDraftDirty, actionState]);

  function signOut() {
    if (isDraftDirty && actionState !== "saving") {
      if (!window.confirm("You have unsaved changes in your course content draft. Are you sure you want to sign out?")) {
        return;
      }
    }
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setWorkspace(null);
    setError(null);
    setLoadState("idle");
  }

  const courseTitle = workspace?.course.title || "Course content";
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
      isSignedIn={hasToken || Boolean(session)}
      notice={routeNotice(error)}
      onSignOut={signOut}
      session={session}
      statusItems={
        workspace && summary ? (
          <>
            <StatusLine icon={<BookOpen size={16} aria-hidden />} label={`${workspace.chapters.length} chapter${workspace.chapters.length === 1 ? "" : "s"}`} tone={workspace.chapters.length ? "good" : "warn"} />
            <StatusLine icon={<FileText size={16} aria-hidden />} label={`${summary.contentCount} content item${summary.contentCount === 1 ? "" : "s"}`} tone={summary.contentCount ? "good" : "warn"} />
          </>
        ) : null
      }
      title={courseTitle}
    >
      <TeacherCourseContentPanels
        actionMessage={actionMessage}
        actionState={actionState}
        chapterDraft={chapterDraft}
        contentActions={contentActions}
        contentDraft={contentDraft}
        courseId={courseId}
        error={error}
        loadContentRoute={loadContentRoute}
        loadState={loadState}
        setChapterDraft={setChapterDraft}
        setContentDraft={setContentDraft}
        submitChapter={submitChapter}
        workspace={workspace}
      />
    </ProductShell>
  );
}
