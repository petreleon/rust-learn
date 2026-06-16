"use client";

import { useCallback, useEffect, useState } from "react";
import { ProductShell, type ShellNotice } from "@/components/product-shell";
import { fetchCourseDetail, requestCourseJoin, type AssessmentItem, type CourseCatalogDetail, type CourseCatalogItem } from "@/lib/learner";
import { clearStoredSessionToken, fetchCurrentSession, readStoredSessionToken, type CurrentSession, SessionRequestError } from "@/lib/session";
import { loadCourseAssessmentList } from "../assessments/api/courseAssessmentApi";
import { CourseDetailContent } from "./CourseDetailContent";
import { ErrorState } from "./ErrorState";
import { LoadingState } from "./LoadingState";
import { SignedOutState } from "./SignedOutState";
import { StatusPill } from "./StatusPill";
import { enrollmentTone } from "./enrollmentTone";
import { humanize } from "./humanize";
import { learnerNotice } from "./learnerNotice";
import { normalizeRouteError } from "./normalizeRouteError";
import { type LoadState } from "./LoadState";
import { type RouteError } from "./RouteError";

export function LearnerCourseDetailRoute({ courseId }: { courseId: string }) {
  const numericCourseId = Number(courseId);
  const validCourseId = Number.isInteger(numericCourseId) && numericCourseId > 0;
  const [hasToken, setHasToken] = useState(false);
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [detail, setDetail] = useState<CourseCatalogDetail | null>(null);
  const [assessments, setAssessments] = useState<AssessmentItem[]>([]);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [error, setError] = useState<RouteError | null>(null);
  const [actionNotice, setActionNotice] = useState<ShellNotice | null>(null);
  const [joining, setJoining] = useState(false);

  const loadRoute = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setDetail(null);
      setAssessments([]);
      setError(null);
      setLoadState("idle");
      return;
    }

    if (!validCourseId) {
      setHasToken(true);
      setSession(null);
      setDetail(null);
      setAssessments([]);
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

      const [nextDetail, nextAssessments] = await Promise.all([
        fetchCourseDetail({ courseId: numericCourseId, token }),
        loadCourseAssessmentList({ courseId: numericCourseId, token }),
      ]);
      setDetail(nextDetail);
      setAssessments(nextAssessments);
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
      setDetail(null);
      setAssessments([]);
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

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setDetail(null);
    setAssessments([]);
    setError(null);
    setActionNotice(null);
    setLoadState("idle");
  }

  async function requestJoin(course: CourseCatalogItem) {
    const token = readStoredSessionToken();
    if (!token) {
      setLoadState("idle");
      return;
    }

    setJoining(true);
    setActionNotice(null);
    try {
      const joinRequest = await requestCourseJoin({ courseId: course.id, token });
      setActionNotice({
        message: `${course.title} is now ${humanize(joinRequest.status)} and waiting for course staff when review is required.`,
        title: "Enrollment request sent",
        tone: "success",
      });
      await loadRoute();
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (requestError.status === 401) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setActionNotice({
        message: requestError.message,
        title: requestError.code,
        tone: requestError.code === "timeout" || requestError.code === "network_error" ? "warn" : "error",
      });
    } finally {
      setJoining(false);
    }
  }

  const notice = actionNotice || learnerNotice(error, "courses");
  const statusLabel = loadState === "loading" ? "Loading" : detail ? humanize(detail.course.enrollment.state) : "Course access";

  return (
    <ProductShell
      activeNav="learn"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/learn", label: "Learner" },
        { href: "/courses", label: "Courses" },
        { label: detail?.course.title || "Course detail" },
      ]}
      description={detail?.course.description || "Course enrollment, content, teacher, organization, and reward context."}
      eyebrow="Learner"
      isSignedIn={hasToken || Boolean(session)}
      notice={notice}
      onSignOut={signOut}
      session={session}
      statusItems={<StatusPill label={statusLabel} tone={detail ? enrollmentTone(detail.course.enrollment.state) : "neutral"} />}
      title={detail?.course.title || "Course detail"}
    >
      {loadState === "idle" && !session ? <SignedOutState redirect={`/courses/${courseId}`} /> : null}
      {loadState === "loading" ? <LoadingState /> : null}
      {error ? <ErrorState error={error} onRetry={loadRoute} redirect={`/courses/${courseId}`} /> : null}
      {loadState === "success" && detail ? (
        <CourseDetailContent
          assessments={assessments}
          detail={detail}
          joining={joining}
          onRequestJoin={requestJoin}
        />
      ) : null}
    </ProductShell>
  );
}
