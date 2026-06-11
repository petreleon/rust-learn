"use client";

import { useCallback, useEffect, useState } from "react";
import { ProductShell } from "@/components/product-shell";
import { fetchCourseLearning, fetchCourseProgress, saveCourseProgress, type CourseLearningResponse } from "@/lib/learner";
import { clearStoredSessionToken, fetchCurrentSession, readStoredSessionToken, type CurrentSession, SessionRequestError } from "@/lib/session";
import { CourseLearningContentView } from "./CourseLearningContentView";
import { ErrorState } from "./ErrorState";
import { LoadingState } from "./LoadingState";
import { SignedOutState } from "./SignedOutState";
import { StatusPill } from "./StatusPill";
import { contentStateTone } from "./contentStateTone";
import { findLearningContent } from "./findLearningContent";
import { humanize } from "./humanize";
import { learnerNotice } from "./learnerNotice";
import { normalizeRouteError } from "./normalizeRouteError";
import { type LoadState } from "./LoadState";
import { type RouteError } from "./RouteError";

export function LearnerCourseLearnRoute({ courseId }: { courseId: string }) {
  const numericCourseId = Number(courseId);
  const validCourseId = Number.isInteger(numericCourseId) && numericCourseId > 0;
  const [hasToken, setHasToken] = useState(false);
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [learning, setLearning] = useState<CourseLearningResponse | null>(null);
  const [selectedContentId, setSelectedContentId] = useState<number | null>(null);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [error, setError] = useState<RouteError | null>(null);

  const loadRoute = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setLearning(null);
      setError(null);
      setLoadState("idle");
      return;
    }

    if (!validCourseId) {
      setHasToken(true);
      setLearning(null);
      setError({
        code: "not_found",
        message: "Course not found.",
        status: 404,
      });
      setLoadState("error");
      return;
    }

    setHasToken(true);
    setLoadState("loading");
    setError(null);

    try {
      const nextSession = await fetchCurrentSession({ token });
      setSession(nextSession);

      const nextLearning = await fetchCourseLearning({ courseId: numericCourseId, token });
      setLearning(nextLearning);

      let progressContentId = nextLearning.active_content_id;
      try {
        const progress = await fetchCourseProgress({ courseId: numericCourseId, token });
        if (progress && progress.content_id) {
          progressContentId = progress.content_id;
        }
      } catch {
        // Progress fetch is best-effort; fall back to active_content_id.
      }
      setSelectedContentId(progressContentId);
      setLoadState("success");
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (
        requestError.status === 401 ||
        (nextError instanceof SessionRequestError && requestError.status === 404)
      ) {
        clearStoredSessionToken();
        setHasToken(false);
        setSession(null);
      }
      setLearning(null);
      setError({
        code: requestError.code,
        message: requestError.message,
        status: requestError.status,
      });
      setLoadState("error");
    }
  }, [numericCourseId, validCourseId]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadRoute(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadRoute]);

  useEffect(() => {
    if (loadState !== "success" || !selectedContentId) return;
    const token = readStoredSessionToken();
    if (!token) return;
    saveCourseProgress({
      contentId: selectedContentId,
      courseId: numericCourseId,
      token,
    }).catch(() => {});
  }, [loadState, numericCourseId, selectedContentId]);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setLearning(null);
    setError(null);
    setLoadState("idle");
  }

  const selectedContent = findLearningContent(learning, selectedContentId);
  const statusLabel = loadState === "loading" ? "Loading" : selectedContent ? humanize(selectedContent.display_state) : "Lesson";

  return (
    <ProductShell
      activeNav="learn"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/learn", label: "Learner" },
        { href: "/courses", label: "Courses" },
        { href: `/courses/${courseId}`, label: learning?.course.title || "Course detail" },
        { label: "Learn" },
      ]}
      description="Course outline, lesson content, and media availability."
      eyebrow="Learner"
      isSignedIn={hasToken || Boolean(session)}
      notice={learnerNotice(error, "courses")}
      onSignOut={signOut}
      session={session}
      statusItems={<StatusPill label={statusLabel} tone={selectedContent ? contentStateTone(selectedContent.display_state) : "neutral"} />}
      title={learning?.course.title || "Course learning"}
    >
      {loadState === "idle" && !session ? <SignedOutState redirect={`/courses/${courseId}/learn`} /> : null}
      {loadState === "loading" ? <LoadingState /> : null}
      {error ? <ErrorState error={error} onRetry={loadRoute} redirect={`/courses/${courseId}/learn`} /> : null}
      {loadState === "success" && learning ? (
        <CourseLearningContentView
          courseId={numericCourseId}
          learning={learning}
          selectedContent={selectedContent}
          selectedContentId={selectedContentId}
          onSelectContent={setSelectedContentId}
        />
      ) : null}
    </ProductShell>
  );
}
