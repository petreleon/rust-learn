"use client";

import { type FormEvent, useCallback, useEffect, useState } from "react";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherCourseWorkspaceResponse } from "@/lib/teacher/TeacherCourseWorkspaceResponse";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { type ActionState } from "@/features/teacher/shared/route-kit/ActionState";
import { type ChapterDraft } from "@/features/teacher/shared/route-kit/ChapterDraft";
import { type ContentDraft } from "@/features/teacher/shared/route-kit/ContentDraft";
import { defaultChapterDraft } from "@/features/teacher/shared/route-kit/defaultChapterDraft";
import { defaultContentDraft } from "@/features/teacher/shared/route-kit/defaultContentDraft";
import { type LoadState } from "@/features/teacher/shared/route-kit/LoadState";
import { normalizeRouteError } from "@/features/teacher/shared/route-kit/normalizeRouteError";
import { type RouteError } from "@/features/teacher/shared/route-kit/RouteError";
import { contentDraftOrder } from "@/features/teacher/shared/route-kit/contentAuthoringHelpers";
import { loadTeacherCourseContentWorkspace, createTeacherCourseContentChapter } from "../api/courseContentApi";
import { useCourseContentActions } from "./useCourseContentActions";

export function useTeacherCourseContentRoute(courseId: string) {
  const [actionMessage, setActionMessage] = useState<string | null>(null);
  const [actionState, setActionState] = useState<ActionState>("idle");
  const [chapterDraft, setChapterDraft] = useState<ChapterDraft>(defaultChapterDraft);
  const [contentDraft, setContentDraft] = useState<ContentDraft>(defaultContentDraft);
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [workspace, setWorkspace] = useState<TeacherCourseWorkspaceResponse | null>(null);

  const clearRoute = useCallback((nextLoadState: LoadState) => {
    setError(null);
    setLoadState(nextLoadState);
    setSession(null);
    setWorkspace(null);
  }, []);

  const loadContentRoute = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token) {
      setHasToken(false);
      clearRoute("idle");
      return;
    }
    setHasToken(true);
    setLoadState("loading");
    setError(null);
    try {
      const next = await loadTeacherCourseContentWorkspace({ courseId, token });
      setSession(next.session);
      setWorkspace(next.workspace);
      setContentDraft((current) => ({
        ...current,
        chapterId: current.chapterId || next.workspace.chapters[0]?.id.toString() || "",
      }));
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) {
        clearBrowserSession();
        setHasToken(false);
      }
      setError(routeError);
      setSession(null);
      setWorkspace(null);
      setLoadState("error");
    }
  }, [clearRoute, courseId]);

  const contentActions = useCourseContentActions({
    contentDraft,
    courseId,
    loadContentRoute,
    setActionMessage,
    setActionState,
    setContentDraft,
    workspace,
  });
  const isDraftDirty = chapterDraft.title.trim() !== "" || contentActions.isContentDraftDirty;

  async function submitChapter(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = readBrowserSessionToken();
    const title = chapterDraft.title.trim();
    const { order, orderText } = contentDraftOrder({ ...defaultContentDraft, order: chapterDraft.order });
    if (!token || !title || !orderText || !Number.isFinite(order) || order < 0) {
      setActionMessage("Chapter title and a non-negative order are required.");
      return;
    }
    setActionState("saving");
    setActionMessage(null);
    try {
      await createTeacherCourseContentChapter({ courseId, order, title, token });
      setChapterDraft(defaultChapterDraft);
      setActionMessage("Chapter created.");
      await loadContentRoute();
    } catch (nextError) {
      setActionMessage(normalizeRouteError(nextError).message);
    } finally {
      setActionState("idle");
    }
  }

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadContentRoute(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadContentRoute]);

  useEffect(() => {
    if (loadState !== "success" || !workspace || !hasProcessingContent(workspace)) return;
    const interval = window.setInterval(() => void loadContentRoute(), 30000);
    return () => window.clearInterval(interval);
  }, [loadState, workspace, loadContentRoute]);

  useEffect(() => {
    const handleBeforeUnload = (event: BeforeUnloadEvent) => {
      if (isDraftDirty && actionState !== "saving") {
        event.preventDefault();
        event.returnValue = "";
      }
    };
    window.addEventListener("beforeunload", handleBeforeUnload);
    return () => window.removeEventListener("beforeunload", handleBeforeUnload);
  }, [isDraftDirty, actionState]);

  function signOut() {
    if (isDraftDirty && actionState !== "saving" && !window.confirm("You have unsaved changes in your course content draft. Are you sure you want to sign out?")) {
      return;
    }
    clearBrowserSession();
    setHasToken(false);
    clearRoute("idle");
  }

  return {
    actionMessage,
    actionState,
    chapterDraft,
    contentActions,
    contentDraft,
    error,
    hasToken,
    loadContentRoute,
    loadState,
    session,
    setChapterDraft,
    setContentDraft,
    signOut,
    submitChapter,
    workspace,
  };
}

function hasProcessingContent(workspace: TeacherCourseWorkspaceResponse) {
  return workspace.chapters.some((chapter) =>
    chapter.contents.some((content) => content.display_state === "processing"),
  );
}

export type TeacherCourseContentRouteController = ReturnType<typeof useTeacherCourseContentRoute>;
