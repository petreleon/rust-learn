"use client";

import {
  AlertCircle,
  ArrowLeft,
  BookOpen,
  BriefcaseBusiness,
  CheckCircle2,
  Clock3,
  FileText,
  Filter,
  Loader2,
  LogIn,
  RefreshCw,
  Search,
  Send,
  ShieldCheck,
  Trophy,
  UserMinus,
  Users,
} from "lucide-react";
import Link from "next/link";
import { type FormEvent, type ReactNode, useCallback, useEffect, useMemo, useState } from "react";
import { ProductShell, type ShellNotice } from "@/components/product-shell";
import {
  clearStoredSessionToken,
  fetchCurrentSession,
  readStoredSessionToken,
  type CurrentSession,
  SessionRequestError,
} from "@/lib/session";
import {
  createTeacherChapter,
  createTeacherContent,
  decideTeacherRewardCandidate,
  decideTeacherJoinRequest,
  fetchMyTeacherApplication,
  fetchTeacherRewardCandidates,
  fetchTeachingCourseEnrollments,
  fetchTeachingCourseStudents,
  fetchTeachingCourseWorkspace,
  fetchTeachingCourses,
  removeTeacherEnrollment,
  TeacherRequestError,
  type TeacherApplication,
  type TeacherApplicationSnapshot,
  type TeacherCourseDashboardItem,
  type TeacherCourseEnrollmentWorkspaceResponse,
  type TeacherCourseJoinRequestItem,
  type TeacherCourseRosterLearner,
  type TeacherCourseStudentProgressItem,
  type TeacherCourseStudentsResponse,
  type TeacherCourseWorkspaceContent,
  type TeacherCourseWorkspaceResponse,
  type TeacherCoursesResponse,
  type TeacherEnrollmentUserSummary,
  type TeacherRewardCandidate,
  type TeacherRewardCandidateDecisionStatus,
  type TeacherRewardCandidateStatusFilter,
} from "@/lib/teacher";
import styles from "./teacher-routes.module.css";

type TeacherRouteView = "dashboard" | "courses";
type LoadState = "idle" | "loading" | "success" | "error";

type RouteError = {
  code: string;
  message: string;
  status: number;
};

type CourseQuery = {
  lifecycleStatus: string;
  search: string;
};

type ChapterDraft = {
  order: string;
  title: string;
};

type ContentDraft = {
  chapterId: string;
  contentType: string;
  data: string;
  order: string;
};

type ActionState = "idle" | "saving";

const emptyApplicationSnapshot: TeacherApplicationSnapshot = {
  application: null,
  audit_events: [],
};

const emptyCourses: TeacherCoursesResponse = {
  courses: [],
  lifecycle_status: null,
  limit: 25,
  offset: 0,
  search: null,
  total: 0,
};

const defaultCourseQuery: CourseQuery = {
  lifecycleStatus: "all",
  search: "",
};

const defaultChapterDraft: ChapterDraft = {
  order: "1",
  title: "",
};

const defaultContentDraft: ContentDraft = {
  chapterId: "",
  contentType: "article",
  data: "",
  order: "1",
};

const enrollmentStatusOptions = ["open", "pending", "waitlisted", "approved", "rejected", "all"] as const;

type EnrollmentStatusFilter = (typeof enrollmentStatusOptions)[number];

type DecisionStatus = "approved" | "rejected" | "waitlisted";

type DecisionDraft = {
  reason: string;
  status: DecisionStatus;
};

const defaultDecisionDraft: DecisionDraft = {
  reason: "",
  status: "approved",
};

const rewardStatusOptions: TeacherRewardCandidateStatusFilter[] = [
  "pending_teacher_approval",
  "teacher_approved",
  "teacher_rejected",
  "failed",
  "all",
];

type RewardDecisionDraft = {
  reason: string;
  status: TeacherRewardCandidateDecisionStatus;
};

const defaultRewardDecisionDraft: RewardDecisionDraft = {
  reason: "",
  status: "teacher_approved",
};

const submitTeacherApplicationPermission = "SUBMIT_TEACHER_APPLICATION";

export function TeacherRoute({ view }: { view: TeacherRouteView }) {
  const [applicationSnapshot, setApplicationSnapshot] = useState<TeacherApplicationSnapshot>(emptyApplicationSnapshot);
  const [courses, setCourses] = useState<TeacherCoursesResponse>(emptyCourses);
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [query, setQuery] = useState<CourseQuery>(defaultCourseQuery);
  const [session, setSession] = useState<CurrentSession | null>(null);

  const loadTeacherRoute = useCallback(
    async (nextQuery: CourseQuery) => {
      const token = readStoredSessionToken();
      if (!token) {
        setHasToken(false);
        setSession(null);
        setApplicationSnapshot(emptyApplicationSnapshot);
        setCourses(emptyCourses);
        setError(null);
        setLoadState("idle");
        return;
      }

      setHasToken(true);
      setLoadState("loading");
      setError(null);

      try {
        const [nextSession, nextCourses, nextApplicationSnapshot] = await Promise.all([
          fetchCurrentSession({ token }),
          fetchTeachingCourses({
            lifecycleStatus: nextQuery.lifecycleStatus,
            search: nextQuery.search,
            token,
          }),
          fetchMyTeacherApplication({ token }),
        ]);
        setSession(nextSession);
        setCourses(nextCourses);
        setApplicationSnapshot(nextApplicationSnapshot);
        setLoadState("success");
      } catch (nextError) {
        const routeError = normalizeRouteError(nextError);
        if (routeError.status === 401 || routeError.status === 404) {
          clearStoredSessionToken();
          setHasToken(false);
        }
        setSession(null);
        setApplicationSnapshot(emptyApplicationSnapshot);
        setCourses(emptyCourses);
        setError(routeError);
        setLoadState("error");
      }
    },
    [],
  );

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadTeacherRoute(defaultCourseQuery), 0);
    return () => window.clearTimeout(timeout);
  }, [loadTeacherRoute]);

  const canSubmitApplication = Boolean(session?.platform.effective_permissions.includes(submitTeacherApplicationPermission));
  const totals = useMemo(() => dashboardTotals(courses.courses), [courses.courses]);
  const notice = routeNotice(error);
  const application = applicationSnapshot.application;
  const canUseTeacherSurface = courses.total > 0 || canSubmitApplication || Boolean(application);
  const previewCourses = view === "dashboard" ? courses.courses.slice(0, 3) : courses.courses;

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setApplicationSnapshot(emptyApplicationSnapshot);
    setCourses(emptyCourses);
    setError(null);
    setLoadState("idle");
  }

  function applyFilters(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    void loadTeacherRoute(query);
  }

  return (
    <ProductShell
      activeNav="teach"
      breadcrumbs={view === "courses" ? [{ href: "/session", label: "Workspace" }, { href: "/teach", label: "Teach" }, { label: "Courses" }] : [{ href: "/session", label: "Workspace" }, { label: "Teach" }]}
      description={
        view === "courses"
          ? "Teaching courses, lifecycle state, enrollment pressure, reward review queues, and scoped actions."
          : "Teaching work, application state, course health, enrollment pressure, and reward review queues."
      }
      eyebrow="Teacher"
      isSignedIn={hasToken || Boolean(session)}
      notice={notice}
      onSignOut={signOut}
      session={session}
      statusItems={
        <>
          <StatusLine icon={<BookOpen size={16} aria-hidden />} label={`${courses.total} teaching course${courses.total === 1 ? "" : "s"}`} tone={courses.total ? "good" : "neutral"} />
          <StatusLine icon={<Users size={16} aria-hidden />} label={`${totals.pendingEnrollmentCount} enrollment request${totals.pendingEnrollmentCount === 1 ? "" : "s"}`} tone={totals.pendingEnrollmentCount ? "warn" : "neutral"} />
          <StatusLine icon={<Trophy size={16} aria-hidden />} label={`${totals.pendingRewardCount} reward review${totals.pendingRewardCount === 1 ? "" : "s"}`} tone={totals.pendingRewardCount ? "warn" : "neutral"} />
        </>
      }
      title={view === "courses" ? "Teaching courses" : "Teaching workspace"}
    >
      {loadState === "loading" ? (
        <StatePanel
          detail="Resolving teaching courses, enrollment queues, reward candidates, and application status."
          icon={<Loader2 className={styles.spin} size={22} aria-hidden />}
          title="Loading teaching workspace"
        />
      ) : null}

      {loadState === "idle" ? (
        <StatePanel
          action={
            <Link className={styles.primaryLink} href={`/login?redirect=${view === "courses" ? "/teach/courses" : "/teach"}`}>
              <LogIn size={18} aria-hidden />
              Sign in
            </Link>
          }
          detail="Teaching work loads from your signed-in RustLearn session."
          icon={<LogIn size={22} aria-hidden />}
          title="Sign in required"
        />
      ) : null}

      {loadState === "error" && error ? (
        <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
          <AlertCircle size={18} aria-hidden />
          <span>
            <strong>{error.code}</strong>
            {error.message}
          </span>
          <button className={styles.secondaryButton} type="button" onClick={() => void loadTeacherRoute(query)}>
            <RefreshCw size={17} aria-hidden />
            Retry
          </button>
        </section>
      ) : null}

      {loadState === "success" && session ? (
        <>
          {!canUseTeacherSurface ? (
            <section className={`${styles.warningPanel} ${styles.singlePanel}`}>
              <div className={styles.panelHeader}>
                <ShieldCheck size={20} aria-hidden />
                <h2>No teaching access yet</h2>
              </div>
              <p className={styles.muted}>
                This session has no teaching courses, no visible teacher application, and no application submission permission.
              </p>
            </section>
          ) : null}

          {view === "dashboard" ? (
            <DashboardView
              application={application}
              canSubmitApplication={canSubmitApplication}
              courses={previewCourses}
              hasMoreCourses={courses.total > previewCourses.length}
              totals={totals}
            />
          ) : (
            <CoursesView
              courses={previewCourses}
              onApplyFilters={applyFilters}
              onQueryChange={setQuery}
              query={query}
              total={courses.total}
            />
          )}
        </>
      ) : null}
    </ProductShell>
  );
}

export function TeacherCourseWorkspaceRoute({ courseId }: { courseId: string }) {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [workspace, setWorkspace] = useState<TeacherCourseWorkspaceResponse | null>(null);

  const loadWorkspace = useCallback(async () => {
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
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        if (routeError.status === 401) {
          clearStoredSessionToken();
          setHasToken(false);
        }
      }
      setSession(null);
      setWorkspace(null);
      setError(routeError);
      setLoadState("error");
    }
  }, [courseId]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadWorkspace(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadWorkspace]);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setWorkspace(null);
    setError(null);
    setLoadState("idle");
  }

  const workspaceTotals = workspace ? workspaceSummary(workspace) : null;
  const courseTitle = workspace?.course.title || "Course workspace";

  return (
    <ProductShell
      activeNav="teach"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/teach", label: "Teach" },
        { href: "/teach/courses", label: "Courses" },
        { label: courseTitle },
      ]}
      description="Course-scoped teaching workspace for lifecycle, content structure, roster pressure, and reward review readiness."
      eyebrow="Teacher course"
      isSignedIn={hasToken || Boolean(session)}
      notice={routeNotice(error)}
      onSignOut={signOut}
      session={session}
      statusItems={
        workspace && workspaceTotals ? (
          <>
            <StatusLine icon={<ShieldCheck size={16} aria-hidden />} label={statusLabel(workspace.course.lifecycle_status)} tone={lifecycleTone(workspace.course.lifecycle_status)} />
            <StatusLine icon={<FileText size={16} aria-hidden />} label={`${workspaceTotals.contentCount} content item${workspaceTotals.contentCount === 1 ? "" : "s"}`} tone={workspaceTotals.contentCount ? "good" : "warn"} />
            <StatusLine icon={<Trophy size={16} aria-hidden />} label={`${workspace.course.reward_queue.pending_teacher_count} reward review${workspace.course.reward_queue.pending_teacher_count === 1 ? "" : "s"}`} tone={workspace.course.reward_queue.pending_teacher_count ? "warn" : "neutral"} />
          </>
        ) : null
      }
      title={courseTitle}
    >
      {loadState === "loading" ? (
        <StatePanel
          detail="Loading course lifecycle, teaching permissions, chapters, content, processing signals, roster pressure, and reward queues."
          icon={<Loader2 className={styles.spin} size={22} aria-hidden />}
          title="Loading course workspace"
        />
      ) : null}

      {loadState === "idle" ? (
        <StatePanel
          action={
            <Link className={styles.primaryLink} href={`/login?redirect=/teach/courses/${courseId}`}>
              <LogIn size={18} aria-hidden />
              Sign in
            </Link>
          }
          detail="Course workspaces load from your signed-in teaching session."
          icon={<LogIn size={22} aria-hidden />}
          title="Sign in required"
        />
      ) : null}

      {loadState === "error" && error ? (
        <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
          <AlertCircle size={18} aria-hidden />
          <span>
            <strong>{error.code}</strong>
            {error.message}
          </span>
          <button className={styles.secondaryButton} type="button" onClick={() => void loadWorkspace()}>
            <RefreshCw size={17} aria-hidden />
            Retry
          </button>
        </section>
      ) : null}

      {loadState === "success" && workspace ? <WorkspaceView workspace={workspace} /> : null}
    </ProductShell>
  );
}

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

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadContentRoute(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadContentRoute]);

  useEffect(() => {
    const handleBeforeUnload = (e: BeforeUnloadEvent) => {
      const isDirty =
        chapterDraft.title.trim() !== "" ||
        contentDraft.data.trim() !== "";

      if (isDirty && actionState !== "saving") {
        e.preventDefault();
        e.returnValue = "";
      }
    };
    window.addEventListener("beforeunload", handleBeforeUnload);
    return () => window.removeEventListener("beforeunload", handleBeforeUnload);
  }, [chapterDraft.title, contentDraft.data, actionState]);

  function signOut() {
    const isDirty =
      chapterDraft.title.trim() !== "" ||
      contentDraft.data.trim() !== "";

    if (isDirty && actionState !== "saving") {
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

  async function submitChapter(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = readStoredSessionToken();
    const title = chapterDraft.title.trim();
    const orderText = chapterDraft.order.trim();
    const order = Number(orderText);
    if (!token || !title || !orderText || !Number.isFinite(order) || order < 0) {
      setActionMessage("Chapter title and a non-negative order are required.");
      return;
    }

    setActionState("saving");
    setActionMessage(null);
    try {
      await createTeacherChapter({
        courseId,
        payload: { order, title },
        token,
      });
      setChapterDraft(defaultChapterDraft);
      setActionMessage("Chapter created.");
      await loadContentRoute();
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      setActionMessage(routeError.message);
    } finally {
      setActionState("idle");
    }
  }

  async function submitContent(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = readStoredSessionToken();
    const chapterId = contentDraft.chapterId;
    const orderText = contentDraft.order.trim();
    const order = Number(orderText);
    const data = contentDraft.data.trim();
    if (!token || !chapterId || !orderText || !Number.isFinite(order) || order < 0 || !data) {
      setActionMessage("Chapter, a non-negative order, and lesson body are required.");
      return;
    }

    setActionState("saving");
    setActionMessage(null);
    try {
      await createTeacherContent({
        chapterId,
        courseId,
        payload: {
          content_type: contentDraft.contentType,
          data,
          order,
        },
        token,
      });
      setContentDraft((current) => ({ ...current, data: "", order: "1" }));
      setActionMessage("Content item created.");
      await loadContentRoute();
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      setActionMessage(routeError.message);
    } finally {
      setActionState("idle");
    }
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
        workspace ? (
          <>
            <StatusLine icon={<BookOpen size={16} aria-hidden />} label={`${workspace.chapters.length} chapter${workspace.chapters.length === 1 ? "" : "s"}`} tone={workspace.chapters.length ? "good" : "warn"} />
            <StatusLine icon={<FileText size={16} aria-hidden />} label={`${workspaceSummary(workspace).contentCount} content item${workspaceSummary(workspace).contentCount === 1 ? "" : "s"}`} tone={workspaceSummary(workspace).contentCount ? "good" : "warn"} />
          </>
        ) : null
      }
      title={courseTitle}
    >
      {loadState === "loading" ? (
        <StatePanel
          detail="Loading course content structure and authoring permissions."
          icon={<Loader2 className={styles.spin} size={22} aria-hidden />}
          title="Loading content authoring"
        />
      ) : null}

      {loadState === "idle" ? (
        <StatePanel
          action={
            <Link className={styles.primaryLink} href={`/login?redirect=/teach/courses/${courseId}/content`}>
              <LogIn size={18} aria-hidden />
              Sign in
            </Link>
          }
          detail="Content authoring loads from your signed-in teaching session."
          icon={<LogIn size={22} aria-hidden />}
          title="Sign in required"
        />
      ) : null}

      {loadState === "error" && error ? (
        <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
          <AlertCircle size={18} aria-hidden />
          <span>
            <strong>{error.code}</strong>
            {error.message}
          </span>
          <button className={styles.secondaryButton} type="button" onClick={() => void loadContentRoute()}>
            <RefreshCw size={17} aria-hidden />
            Retry
          </button>
        </section>
      ) : null}

      {loadState === "success" && workspace ? (
        <ContentAuthoringView
          actionMessage={actionMessage}
          actionState={actionState}
          chapterDraft={chapterDraft}
          contentDraft={contentDraft}
          onChapterDraftChange={setChapterDraft}
          onContentDraftChange={setContentDraft}
          onSubmitChapter={submitChapter}
          onSubmitContent={submitContent}
          workspace={workspace}
        />
      ) : null}
    </ProductShell>
  );
}

export function TeacherCourseEnrollmentsRoute({ courseId }: { courseId: string }) {
  const [actionMessage, setActionMessage] = useState<string | null>(null);
  const [actionState, setActionState] = useState<ActionState>("idle");
  const [confirmRemovalUserId, setConfirmRemovalUserId] = useState<number | null>(null);
  const [decisionDrafts, setDecisionDrafts] = useState<Record<number, DecisionDraft>>({});
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [statusFilter, setStatusFilter] = useState<EnrollmentStatusFilter>("open");
  const [workspace, setWorkspace] = useState<TeacherCourseEnrollmentWorkspaceResponse | null>(null);

  const loadEnrollmentRoute = useCallback(async () => {
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
        fetchTeachingCourseEnrollments({ courseId, status: statusFilter, token }),
      ]);
      setSession(nextSession);
      setWorkspace(nextWorkspace);
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
  }, [courseId, statusFilter]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadEnrollmentRoute(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadEnrollmentRoute]);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setWorkspace(null);
    setError(null);
    setLoadState("idle");
  }

  function updateDecisionDraft(requestId: number, draft: DecisionDraft) {
    setDecisionDrafts((current) => ({
      ...current,
      [requestId]: draft,
    }));
  }

  async function submitDecision(request: TeacherCourseJoinRequestItem, event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = readStoredSessionToken();
    const draft = decisionDrafts[request.id] || defaultDecisionDraft;
    if (!token) {
      setActionMessage("Sign in again before deciding enrollment requests.");
      return;
    }

    setActionState("saving");
    setActionMessage(null);
    try {
      await decideTeacherJoinRequest({
        courseId,
        payload: {
          decision_reason: draft.reason.trim() || null,
          status: draft.status,
        },
        requestId: request.id,
        token,
      });
      setDecisionDrafts((current) => {
        const next = { ...current };
        delete next[request.id];
        return next;
      });
      setActionMessage("Enrollment request updated.");
      await loadEnrollmentRoute();
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      setActionMessage(routeError.message);
    } finally {
      setActionState("idle");
    }
  }

  async function removeLearner(learner: TeacherCourseRosterLearner) {
    const token = readStoredSessionToken();
    if (!token) {
      setActionMessage("Sign in again before changing roster access.");
      return;
    }

    if (confirmRemovalUserId !== learner.user.id) {
      setConfirmRemovalUserId(learner.user.id);
      setActionMessage(`Confirm removal for ${learner.user.name}.`);
      return;
    }

    setActionState("saving");
    setActionMessage(null);
    try {
      await removeTeacherEnrollment({
        courseId,
        token,
        userId: learner.user.id,
      });
      setConfirmRemovalUserId(null);
      setActionMessage("Learner removed from the course roster.");
      await loadEnrollmentRoute();
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      setActionMessage(routeError.message);
    } finally {
      setActionState("idle");
    }
  }

  const courseTitle = workspace?.course.title || "Course enrollments";
  return (
    <ProductShell
      activeNav="teach"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/teach", label: "Teach" },
        { href: "/teach/courses", label: "Courses" },
        { href: `/teach/courses/${courseId}`, label: courseTitle },
        { label: "Enrollments" },
      ]}
      description="Review course join requests and roster access without raw learner ids."
      eyebrow="Teacher enrollments"
      isSignedIn={hasToken || Boolean(session)}
      notice={routeNotice(error)}
      onSignOut={signOut}
      session={session}
      statusItems={
        workspace ? (
          <>
            <StatusLine icon={<Clock3 size={16} aria-hidden />} label={`${workspace.join_requests.total} request${workspace.join_requests.total === 1 ? "" : "s"}`} tone={workspace.join_requests.total ? "warn" : "neutral"} />
            <StatusLine icon={<Users size={16} aria-hidden />} label={`${workspace.roster.total} learner${workspace.roster.total === 1 ? "" : "s"}`} tone={workspace.roster.total ? "good" : "neutral"} />
          </>
        ) : null
      }
      title={courseTitle}
    >
      {loadState === "loading" ? (
        <StatePanel
          detail="Loading enrollment requests, roster access, and course-scoped permissions."
          icon={<Loader2 className={styles.spin} size={22} aria-hidden />}
          title="Loading enrollments"
        />
      ) : null}

      {loadState === "idle" ? (
        <StatePanel
          action={
            <Link className={styles.primaryLink} href={`/login?redirect=/teach/courses/${courseId}/enrollments`}>
              <LogIn size={18} aria-hidden />
              Sign in
            </Link>
          }
          detail="Enrollment management loads from your signed-in teaching session."
          icon={<LogIn size={22} aria-hidden />}
          title="Sign in required"
        />
      ) : null}

      {loadState === "error" && error ? (
        <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
          <AlertCircle size={18} aria-hidden />
          <span>
            <strong>{error.code}</strong>
            {error.message}
          </span>
          <button className={styles.secondaryButton} type="button" onClick={() => void loadEnrollmentRoute()}>
            <RefreshCw size={17} aria-hidden />
            Retry
          </button>
        </section>
      ) : null}

      {loadState === "success" && workspace ? (
        <EnrollmentWorkspaceView
          actionMessage={actionMessage}
          actionState={actionState}
          confirmRemovalUserId={confirmRemovalUserId}
          decisionDrafts={decisionDrafts}
          onDecisionDraftChange={updateDecisionDraft}
          onRefresh={() => void loadEnrollmentRoute()}
          onRemoveLearner={(learner) => void removeLearner(learner)}
          onStatusFilterChange={setStatusFilter}
          onSubmitDecision={submitDecision}
          statusFilter={statusFilter}
          workspace={workspace}
        />
      ) : null}
    </ProductShell>
  );
}

export function TeacherCourseStudentsRoute({ courseId }: { courseId: string }) {
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [students, setStudents] = useState<TeacherCourseStudentsResponse | null>(null);

  const loadStudentsRoute = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setStudents(null);
      setError(null);
      setLoadState("idle");
      return;
    }

    setHasToken(true);
    setLoadState("loading");
    setError(null);

    try {
      const [nextSession, nextStudents] = await Promise.all([
        fetchCurrentSession({ token }),
        fetchTeachingCourseStudents({ courseId, token }),
      ]);
      setSession(nextSession);
      setStudents(nextStudents);
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setSession(null);
      setStudents(null);
      setError(routeError);
      setLoadState("error");
    }
  }, [courseId]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadStudentsRoute(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadStudentsRoute]);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setStudents(null);
    setError(null);
    setLoadState("idle");
  }

  const courseTitle = students?.course.title || "Student progress";
  return (
    <ProductShell
      activeNav="teach"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/teach", label: "Teach" },
        { href: "/teach/courses", label: "Courses" },
        { href: `/teach/courses/${courseId}`, label: courseTitle },
        { label: "Students" },
      ]}
      description="Teacher-visible learner roster with honest progress support and reward evidence."
      eyebrow="Teacher students"
      isSignedIn={hasToken || Boolean(session)}
      notice={routeNotice(error)}
      onSignOut={signOut}
      session={session}
      statusItems={
        students ? (
          <>
            <StatusLine icon={<Users size={16} aria-hidden />} label={`${students.total} learner${students.total === 1 ? "" : "s"}`} tone={students.total ? "good" : "neutral"} />
            <StatusLine icon={<Trophy size={16} aria-hidden />} label={students.reward_evidence_supported ? "Reward evidence" : "No reward evidence"} tone={students.reward_evidence_supported ? "good" : "neutral"} />
          </>
        ) : null
      }
      title={courseTitle}
    >
      {loadState === "loading" ? (
        <StatePanel
          detail="Loading learner roster, progress support, and reward evidence."
          icon={<Loader2 className={styles.spin} size={22} aria-hidden />}
          title="Loading student progress"
        />
      ) : null}

      {loadState === "idle" ? (
        <StatePanel
          action={
            <Link className={styles.primaryLink} href={`/login?redirect=/teach/courses/${courseId}/students`}>
              <LogIn size={18} aria-hidden />
              Sign in
            </Link>
          }
          detail="Student progress loads from your signed-in teaching session."
          icon={<LogIn size={22} aria-hidden />}
          title="Sign in required"
        />
      ) : null}

      {loadState === "error" && error ? (
        <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
          <AlertCircle size={18} aria-hidden />
          <span>
            <strong>{error.code}</strong>
            {error.message}
          </span>
          <button className={styles.secondaryButton} type="button" onClick={() => void loadStudentsRoute()}>
            <RefreshCw size={17} aria-hidden />
            Retry
          </button>
        </section>
      ) : null}

      {loadState === "success" && students ? <StudentProgressView students={students} /> : null}
    </ProductShell>
  );
}

export function TeacherCourseRewardsRoute({ courseId }: { courseId: string }) {
  const [actionMessage, setActionMessage] = useState<string | null>(null);
  const [actionState, setActionState] = useState<ActionState>("idle");
  const [candidates, setCandidates] = useState<TeacherRewardCandidate[]>([]);
  const [decisionDrafts, setDecisionDrafts] = useState<Record<number, RewardDecisionDraft>>({});
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [statusFilter, setStatusFilter] = useState<TeacherRewardCandidateStatusFilter>("pending_teacher_approval");
  const [students, setStudents] = useState<TeacherCourseStudentsResponse | null>(null);

  const loadRewardsRoute = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setStudents(null);
      setCandidates([]);
      setError(null);
      setLoadState("idle");
      return;
    }

    setHasToken(true);
    setLoadState("loading");
    setError(null);

    try {
      const [nextSession, nextStudents, nextCandidates] = await Promise.all([
        fetchCurrentSession({ token }),
        fetchTeachingCourseStudents({ courseId, token }),
        fetchTeacherRewardCandidates({ courseId, status: statusFilter, token }),
      ]);
      setSession(nextSession);
      setStudents(nextStudents);
      setCandidates(nextCandidates);
      setLoadState("success");
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setSession(null);
      setStudents(null);
      setCandidates([]);
      setError(routeError);
      setLoadState("error");
    }
  }, [courseId, statusFilter]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadRewardsRoute(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadRewardsRoute]);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setStudents(null);
    setCandidates([]);
    setError(null);
    setLoadState("idle");
  }

  function updateDecisionDraft(candidateId: number, draft: RewardDecisionDraft) {
    setDecisionDrafts((current) => ({
      ...current,
      [candidateId]: draft,
    }));
  }

  async function submitRewardDecision(candidate: TeacherRewardCandidate, event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = readStoredSessionToken();
    const draft = decisionDrafts[candidate.id] || defaultRewardDecisionDraft;
    if (!token) {
      setActionMessage("Sign in again before deciding reward candidates.");
      return;
    }

    setActionState("saving");
    setActionMessage(null);
    try {
      await decideTeacherRewardCandidate({
        candidateId: candidate.id,
        courseId,
        payload: {
          decision_reason: draft.reason.trim() || null,
          status: draft.status,
        },
        token,
      });
      setDecisionDrafts((current) => {
        const next = { ...current };
        delete next[candidate.id];
        return next;
      });
      setActionMessage("Reward candidate decision saved.");
      await loadRewardsRoute();
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      setActionMessage(
        routeError.status === 409
          ? `${routeError.message} The queue has been refreshed.`
          : routeError.message,
      );
      if (routeError.status === 409) {
        await loadRewardsRoute();
      }
    } finally {
      setActionState("idle");
    }
  }

  const courseTitle = students?.course.title || "Reward review";
  return (
    <ProductShell
      activeNav="teach"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/teach", label: "Teach" },
        { href: "/teach/courses", label: "Courses" },
        { href: `/teach/courses/${courseId}`, label: courseTitle },
        { label: "Rewards" },
      ]}
      description="Review course-scoped reward evidence and apply teacher decisions without platform payout controls."
      eyebrow="Teacher rewards"
      isSignedIn={hasToken || Boolean(session)}
      notice={routeNotice(error)}
      onSignOut={signOut}
      session={session}
      statusItems={
        students ? (
          <>
            <StatusLine icon={<Clock3 size={16} aria-hidden />} label={`${students.course.reward_queue.pending_teacher_count} pending`} tone={students.course.reward_queue.pending_teacher_count ? "warn" : "neutral"} />
            <StatusLine icon={<Trophy size={16} aria-hidden />} label={`${candidates.length} shown`} tone={candidates.length ? "good" : "neutral"} />
          </>
        ) : null
      }
      title={courseTitle}
    >
      {loadState === "loading" ? (
        <StatePanel
          detail="Loading reward candidates, learner context, and course-scoped permissions."
          icon={<Loader2 className={styles.spin} size={22} aria-hidden />}
          title="Loading reward review"
        />
      ) : null}

      {loadState === "idle" ? (
        <StatePanel
          action={
            <Link className={styles.primaryLink} href={`/login?redirect=/teach/courses/${courseId}/rewards`}>
              <LogIn size={18} aria-hidden />
              Sign in
            </Link>
          }
          detail="Reward review loads from your signed-in teaching session."
          icon={<LogIn size={22} aria-hidden />}
          title="Sign in required"
        />
      ) : null}

      {loadState === "error" && error ? (
        <section className={`${styles.errorBox} ${styles.singlePanel}`} role="status">
          <AlertCircle size={18} aria-hidden />
          <span>
            <strong>{error.code}</strong>
            {error.message}
          </span>
          <button className={styles.secondaryButton} type="button" onClick={() => void loadRewardsRoute()}>
            <RefreshCw size={17} aria-hidden />
            Retry
          </button>
        </section>
      ) : null}

      {loadState === "success" && students ? (
        <RewardReviewView
          actionMessage={actionMessage}
          actionState={actionState}
          candidates={candidates}
          decisionDrafts={decisionDrafts}
          onDecisionDraftChange={updateDecisionDraft}
          onRefresh={() => void loadRewardsRoute()}
          onStatusFilterChange={setStatusFilter}
          onSubmitDecision={submitRewardDecision}
          statusFilter={statusFilter}
          students={students}
        />
      ) : null}
    </ProductShell>
  );
}

function DashboardView({
  application,
  canSubmitApplication,
  courses,
  hasMoreCourses,
  totals,
}: {
  application: TeacherApplication | null;
  canSubmitApplication: boolean;
  courses: TeacherCourseDashboardItem[];
  hasMoreCourses: boolean;
  totals: ReturnType<typeof dashboardTotals>;
}) {
  return (
    <>
      <section className={styles.summaryGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Teaching courses" value={totals.courseCount} />
        <SummaryCard icon={<Users size={20} aria-hidden />} label="Enrollment requests" value={totals.pendingEnrollmentCount} tone={totals.pendingEnrollmentCount ? "warn" : "neutral"} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Reward reviews" value={totals.pendingRewardCount} tone={totals.pendingRewardCount ? "warn" : "neutral"} />
        <SummaryCard icon={<FileText size={20} aria-hidden />} label="Course content" value={totals.contentCount} />
      </section>

      <section className={styles.twoColumn}>
        <ApplicationPanel application={application} canSubmitApplication={canSubmitApplication} />
        <section className={styles.panel}>
          <div className={styles.panelHeader}>
            <BriefcaseBusiness size={20} aria-hidden />
            <h2>Next teaching work</h2>
          </div>
          <PriorityList totals={totals} />
        </section>
      </section>

      <section className={styles.courseSection}>
        <div className={styles.sectionHeader}>
          <div>
            <h2>Course health</h2>
            <p className={styles.muted}>Lifecycle state, content readiness, enrollment pressure, and reward queues.</p>
          </div>
          {hasMoreCourses ? (
            <Link className={styles.secondaryLink} href="/teach/courses">
              <BookOpen size={17} aria-hidden />
              View all courses
            </Link>
          ) : null}
        </div>
        <CourseGrid courses={courses} emptyDetail="Approved or delegated teaching courses will appear here." />
      </section>
    </>
  );
}

function CoursesView({
  courses,
  onApplyFilters,
  onQueryChange,
  query,
  total,
}: {
  courses: TeacherCourseDashboardItem[];
  onApplyFilters: (event: FormEvent<HTMLFormElement>) => void;
  onQueryChange: (query: CourseQuery) => void;
  query: CourseQuery;
  total: number;
}) {
  return (
    <>
      <form className={styles.filterPanel} onSubmit={onApplyFilters}>
        <label>
          <span>Search</span>
          <div className={styles.inputWithIcon}>
            <Search size={17} aria-hidden />
            <input
              type="search"
              value={query.search}
              onChange={(event) => onQueryChange({ ...query, search: event.target.value })}
              placeholder="Course title"
            />
          </div>
        </label>
        <label>
          <span>Lifecycle</span>
          <select
            value={query.lifecycleStatus}
            onChange={(event) => onQueryChange({ ...query, lifecycleStatus: event.target.value })}
          >
            <option value="all">All states</option>
            <option value="draft">Draft</option>
            <option value="submitted">Submitted</option>
            <option value="needs_changes">Needs changes</option>
            <option value="approved">Approved</option>
            <option value="published">Published</option>
            <option value="archived">Archived</option>
            <option value="suspended">Suspended</option>
          </select>
        </label>
        <button className={styles.primaryButton} type="submit">
          <Filter size={17} aria-hidden />
          Apply filters
        </button>
      </form>

      <section className={styles.courseSection}>
        <div className={styles.sectionHeader}>
          <div>
            <h2>Visible teaching courses</h2>
            <p className={styles.muted}>
              {total} course{total === 1 ? "" : "s"} {total === 1 ? "matches" : "match"} the current teaching scope.
            </p>
          </div>
        </div>
        <CourseGrid courses={courses} emptyDetail="No teaching courses match the current filters." />
      </section>
    </>
  );
}

function WorkspaceView({ workspace }: { workspace: TeacherCourseWorkspaceResponse }) {
  const totals = workspaceSummary(workspace);
  const organizationNames = workspace.course.organizations.map((organization) => organization.name).join(", ") || "Personal course";
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
            Course lifecycle is {statusLabel(workspace.course.lifecycle_status)}. Individual content publication state is not stored yet, so content inherits the course lifecycle.
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
        <SummaryCard icon={<FileText size={20} aria-hidden />} label="Content items" value={totals.contentCount} tone={totals.contentCount ? "neutral" : "warn"} />
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Chapters" value={workspace.chapters.length} />
        <SummaryCard icon={<Users size={20} aria-hidden />} label="Enrollment requests" value={workspace.course.roster.pending_join_request_count + workspace.course.roster.waitlisted_join_request_count} tone={workspace.course.roster.pending_join_request_count || workspace.course.roster.waitlisted_join_request_count ? "warn" : "neutral"} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Reward reviews" value={workspace.course.reward_queue.pending_teacher_count} tone={workspace.course.reward_queue.pending_teacher_count ? "warn" : "neutral"} />
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
            <DetailLine label="Per-content publication" value={workspace.publication.content_publication_status_supported ? "Supported" : "Inherited from course"} />
          </div>
        </section>
      </section>

      <section className={styles.courseSection}>
        <div className={styles.sectionHeader}>
          <div>
            <h2>Course content</h2>
            <p className={styles.muted}>Structured chapters, content types, stored-data presence, and latest processing state.</p>
          </div>
        </div>
        <ChapterList chapters={workspace.chapters} />
      </section>
    </>
  );
}

function ContentAuthoringView({
  actionMessage,
  actionState,
  chapterDraft,
  contentDraft,
  onChapterDraftChange,
  onContentDraftChange,
  onSubmitChapter,
  onSubmitContent,
  workspace,
}: {
  actionMessage: string | null;
  actionState: ActionState;
  chapterDraft: ChapterDraft;
  contentDraft: ContentDraft;
  onChapterDraftChange: (draft: ChapterDraft) => void;
  onContentDraftChange: (draft: ContentDraft) => void;
  onSubmitChapter: (event: FormEvent<HTMLFormElement>) => void;
  onSubmitContent: (event: FormEvent<HTMLFormElement>) => void;
  workspace: TeacherCourseWorkspaceResponse;
}) {
  const canManageContent = workspace.course.permissions.can_manage_content;
  return (
    <>
      <section className={styles.workspaceHero}>
        <Link className={styles.secondaryLink} href={`/teach/courses/${workspace.course.id}`}>
          <ArrowLeft size={17} aria-hidden />
          Course workspace
        </Link>
        <div className={styles.workspaceTitleBlock}>
          <p className={styles.eyebrow}>{statusLabel(workspace.course.lifecycle_status)}</p>
          <h2>Content authoring</h2>
          <p className={styles.muted}>
            Create chapters and text lessons from structured forms. Upload, processing retry, and destructive editing controls remain separate until their contracts are complete.
          </p>
        </div>
      </section>

      {actionMessage ? (
        <section className={`${styles.warningPanel} ${styles.singlePanel}`} role="status">
          <div className={styles.panelHeader}>
            <AlertCircle size={18} aria-hidden />
            <h2>Authoring update</h2>
          </div>
          <p>{actionMessage}</p>
        </section>
      ) : null}

      <section className={styles.twoColumn}>
        <form className={styles.authoringForm} onSubmit={onSubmitChapter}>
          <div className={styles.panelHeader}>
            <BookOpen size={20} aria-hidden />
            <h2>Create chapter</h2>
          </div>
          <label>
            <span>Title</span>
            <input
              disabled={!canManageContent || actionState === "saving"}
              maxLength={120}
              onChange={(event) => onChapterDraftChange({ ...chapterDraft, title: event.target.value })}
              placeholder="Chapter title"
              value={chapterDraft.title}
            />
          </label>
          <label>
            <span>Order</span>
            <input
              disabled={!canManageContent || actionState === "saving"}
              min="0"
              onChange={(event) => onChapterDraftChange({ ...chapterDraft, order: event.target.value })}
              type="number"
              value={chapterDraft.order}
            />
          </label>
          <button className={styles.primaryButton} disabled={!canManageContent || actionState === "saving"} type="submit">
            <Send size={17} aria-hidden />
            Create chapter
          </button>
          {!canManageContent ? <p className={styles.muted}>This session can view content structure but cannot author course content.</p> : null}
        </form>

        <form className={styles.authoringForm} onSubmit={onSubmitContent}>
          <div className={styles.panelHeader}>
            <FileText size={20} aria-hidden />
            <h2>Create text content</h2>
          </div>
          <label>
            <span>Chapter</span>
            <select
              disabled={!canManageContent || actionState === "saving" || !workspace.chapters.length}
              onChange={(event) => onContentDraftChange({ ...contentDraft, chapterId: event.target.value })}
              value={contentDraft.chapterId || workspace.chapters[0]?.id.toString() || ""}
            >
              {workspace.chapters.length ? (
                workspace.chapters.map((chapter) => (
                  <option key={chapter.id} value={chapter.id}>
                    {chapter.title}
                  </option>
                ))
              ) : (
                <option value="">Create a chapter first</option>
              )}
            </select>
          </label>
          <label>
            <span>Type</span>
            <select
              disabled={!canManageContent || actionState === "saving"}
              onChange={(event) => onContentDraftChange({ ...contentDraft, contentType: event.target.value })}
              value={contentDraft.contentType}
            >
              <option value="article">Article</option>
              <option value="text">Text lesson</option>
            </select>
          </label>
          <label>
            <span>Order</span>
            <input
              disabled={!canManageContent || actionState === "saving"}
              min="0"
              onChange={(event) => onContentDraftChange({ ...contentDraft, order: event.target.value })}
              type="number"
              value={contentDraft.order}
            />
          </label>
          <label>
            <span>Lesson body</span>
            <textarea
              disabled={!canManageContent || actionState === "saving" || !workspace.chapters.length}
              onChange={(event) => onContentDraftChange({ ...contentDraft, data: event.target.value })}
              placeholder="Write the lesson content"
              rows={5}
              value={contentDraft.data}
            />
          </label>
          <button className={styles.primaryButton} disabled={!canManageContent || actionState === "saving" || !workspace.chapters.length} type="submit">
            <Send size={17} aria-hidden />
            Create content
          </button>
        </form>
      </section>

      <section className={styles.courseSection}>
        <div className={styles.sectionHeader}>
          <div>
            <h2>Current structure</h2>
            <p className={styles.muted}>New chapters and text content appear here after the workspace refreshes.</p>
          </div>
        </div>
        <ChapterList chapters={workspace.chapters} />
      </section>
    </>
  );
}

function EnrollmentWorkspaceView({
  actionMessage,
  actionState,
  confirmRemovalUserId,
  decisionDrafts,
  onDecisionDraftChange,
  onRefresh,
  onRemoveLearner,
  onStatusFilterChange,
  onSubmitDecision,
  statusFilter,
  workspace,
}: {
  actionMessage: string | null;
  actionState: ActionState;
  confirmRemovalUserId: number | null;
  decisionDrafts: Record<number, DecisionDraft>;
  onDecisionDraftChange: (requestId: number, draft: DecisionDraft) => void;
  onRefresh: () => void;
  onRemoveLearner: (learner: TeacherCourseRosterLearner) => void;
  onStatusFilterChange: (status: EnrollmentStatusFilter) => void;
  onSubmitDecision: (request: TeacherCourseJoinRequestItem, event: FormEvent<HTMLFormElement>) => void;
  statusFilter: EnrollmentStatusFilter;
  workspace: TeacherCourseEnrollmentWorkspaceResponse;
}) {
  const openRequestCount =
    workspace.course.roster.pending_join_request_count + workspace.course.roster.waitlisted_join_request_count;
  return (
    <>
      <section className={styles.workspaceHero}>
        <Link className={styles.secondaryLink} href={`/teach/courses/${workspace.course.id}`}>
          <ArrowLeft size={17} aria-hidden />
          Course workspace
        </Link>
        <div className={styles.workspaceTitleBlock}>
          <p className={styles.eyebrow}>{statusLabel(workspace.course.lifecycle_status)}</p>
          <h2>Enrollment queue</h2>
          <p className={styles.muted}>
            Review join requests, waitlist learners, approve access, and remove roster access from a course-scoped workspace.
          </p>
        </div>
      </section>

      <section className={styles.summaryGrid}>
        <SummaryCard icon={<Clock3 size={20} aria-hidden />} label="Open requests" value={openRequestCount} tone={openRequestCount ? "warn" : "neutral"} />
        <SummaryCard icon={<Users size={20} aria-hidden />} label="Enrolled learners" value={workspace.roster.total} tone={workspace.roster.total ? "good" : "neutral"} />
        <SummaryCard icon={<CheckCircle2 size={20} aria-hidden />} label="Pending shown" value={workspace.join_requests.requests.filter((request) => request.status === "pending").length} tone="neutral" />
        <SummaryCard icon={<ShieldCheck size={20} aria-hidden />} label="Teacher roles" value={workspace.teacher_roles.length} tone={workspace.teacher_roles.length ? "good" : "neutral"} />
      </section>

      {actionMessage ? (
        <section className={`${styles.warningPanel} ${styles.singlePanel}`} role="status">
          <div className={styles.panelHeader}>
            <AlertCircle size={18} aria-hidden />
            <h2>Enrollment update</h2>
          </div>
          <p>{actionMessage}</p>
        </section>
      ) : null}

      <section className={styles.filterPanel} aria-label="Enrollment request filters">
        <label>
          <span>Request status</span>
          <select
            onChange={(event) => onStatusFilterChange(event.target.value as EnrollmentStatusFilter)}
            value={statusFilter}
          >
            {enrollmentStatusOptions.map((status) => (
              <option key={status} value={status}>
                {statusLabel(status)}
              </option>
            ))}
          </select>
        </label>
        <button className={styles.secondaryButton} type="button" onClick={onRefresh}>
          <RefreshCw size={17} aria-hidden />
          Refresh
        </button>
      </section>

      {!workspace.progress_supported || !workspace.reward_eligibility_supported ? (
        <section className={`${styles.warningPanel} ${styles.singlePanel}`}>
          <div className={styles.panelHeader}>
            <AlertCircle size={18} aria-hidden />
            <h2>Roster signals</h2>
          </div>
          <p>
            Persisted progress and reward eligibility are not available in this enrollment route yet; student progress stays in the next milestone route.
          </p>
        </section>
      ) : null}

      <section className={styles.courseSection}>
        <div className={styles.sectionHeader}>
          <div>
            <h2>Join requests</h2>
            <p className={styles.muted}>Showing {statusLabel(workspace.join_requests.status || "all")} requests with learner context.</p>
          </div>
          <span className={`${styles.statusPill} ${workspace.join_requests.total ? styles.warn : styles.neutral}`}>
            {workspace.join_requests.total} total
          </span>
        </div>
        {workspace.join_requests.requests.length ? (
          <div className={styles.enrollmentList}>
            {workspace.join_requests.requests.map((request) => (
              <EnrollmentRequestCard
                actionState={actionState}
                draft={decisionDrafts[request.id] || defaultDecisionDraft}
                key={request.id}
                onDraftChange={(draft) => onDecisionDraftChange(request.id, draft)}
                onSubmit={(event) => onSubmitDecision(request, event)}
                request={request}
              />
            ))}
          </div>
        ) : (
          <StatePanel
            detail="No learner join requests match this filter."
            icon={<CheckCircle2 size={22} aria-hidden />}
            title="No matching requests"
          />
        )}
      </section>

      <section className={styles.courseSection}>
        <div className={styles.sectionHeader}>
          <div>
            <h2>Roster</h2>
            <p className={styles.muted}>Learners with course access. Removal uses a two-step confirmation.</p>
          </div>
          <span className={`${styles.statusPill} ${workspace.roster.total ? styles.good : styles.neutral}`}>
            {workspace.roster.total} enrolled
          </span>
        </div>
        {workspace.roster.learners.length ? (
          <div className={styles.enrollmentList}>
            {workspace.roster.learners.map((learner) => (
              <RosterLearnerCard
                actionState={actionState}
                confirmRemoval={confirmRemovalUserId === learner.user.id}
                key={learner.user.id}
                learner={learner}
                onRemove={() => onRemoveLearner(learner)}
              />
            ))}
          </div>
        ) : (
          <StatePanel
            detail="No enrolled learners are visible for this course."
            icon={<Users size={22} aria-hidden />}
            title="Roster is empty"
          />
        )}
      </section>
    </>
  );
}

function EnrollmentRequestCard({
  actionState,
  draft,
  onDraftChange,
  onSubmit,
  request,
}: {
  actionState: ActionState;
  draft: DecisionDraft;
  onDraftChange: (draft: DecisionDraft) => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
  request: TeacherCourseJoinRequestItem;
}) {
  return (
    <article className={styles.enrollmentCard}>
      <div className={styles.courseTop}>
        <div>
          <p className={styles.eyebrow}>{request.requester.email}</p>
          <h3>{request.requester.name}</h3>
        </div>
        <span className={`${styles.statusPill} ${styles[joinRequestTone(request.status)]}`}>
          {statusLabel(request.status)}
        </span>
      </div>

      <div className={styles.detailList}>
        <DetailLine label="Requested" value={formatDateTime(request.created_at)} />
        <DetailLine label="Updated" value={formatDateTime(request.updated_at)} />
        <DetailLine label="Email" value={request.requester.email_verified ? "Verified" : "Needs verification"} />
        <DetailLine label="KYC" value={request.requester.kyc_verified ? "Verified" : "Not verified"} />
        {request.decision_reason ? <DetailLine label="Reason" value={request.decision_reason} /> : null}
      </div>

      {request.can_decide ? (
        <form className={styles.decisionForm} onSubmit={onSubmit}>
          <label>
            <span>Decision</span>
            <select
              disabled={actionState === "saving"}
              onChange={(event) => onDraftChange({ ...draft, status: event.target.value as DecisionStatus })}
              value={draft.status}
            >
              <option value="approved">Approve</option>
              <option value="waitlisted">Waitlist</option>
              <option value="rejected">Reject</option>
            </select>
          </label>
          <label>
            <span>Reason</span>
            <textarea
              disabled={actionState === "saving"}
              onChange={(event) => onDraftChange({ ...draft, reason: event.target.value })}
              placeholder="Decision reason"
              rows={3}
              value={draft.reason}
            />
          </label>
          <button className={styles.primaryButton} disabled={actionState === "saving"} type="submit">
            <Send size={17} aria-hidden />
            Apply decision
          </button>
        </form>
      ) : (
        <p className={styles.muted}>This request has already been decided.</p>
      )}
    </article>
  );
}

function RosterLearnerCard({
  actionState,
  confirmRemoval,
  learner,
  onRemove,
}: {
  actionState: ActionState;
  confirmRemoval: boolean;
  learner: TeacherCourseRosterLearner;
  onRemove: () => void;
}) {
  return (
    <article className={styles.enrollmentCard}>
      <div className={styles.courseTop}>
        <div>
          <p className={styles.eyebrow}>{learner.user.email}</p>
          <h3>{learner.user.name}</h3>
        </div>
        <span className={`${styles.statusPill} ${styles.good}`}>{statusLabel(learner.access_state)}</span>
      </div>
      <div className={styles.detailList}>
        <DetailLine label="Roles" value={learner.roles.join(", ") || "Learner"} />
        <DetailLine label="Latest request" value={learner.latest_join_request_status ? statusLabel(learner.latest_join_request_status) : "No request history"} />
        <DetailLine label="Progress" value={learner.progress_supported ? "Tracked" : "Not tracked yet"} />
        <DetailLine label="Reward eligibility" value={learner.reward_eligibility_supported ? "Tracked" : "Not tracked yet"} />
      </div>
      {learner.can_remove ? (
        <div className={styles.rosterAction}>
          {confirmRemoval ? (
            <p className={styles.muted}>Confirm removal to revoke course access for this learner.</p>
          ) : null}
          <button className={confirmRemoval ? styles.primaryButton : styles.secondaryButton} disabled={actionState === "saving"} type="button" onClick={onRemove}>
            <UserMinus size={17} aria-hidden />
            {confirmRemoval ? "Confirm removal" : "Remove"}
          </button>
        </div>
      ) : null}
    </article>
  );
}

function StudentProgressView({ students }: { students: TeacherCourseStudentsResponse }) {
  const pendingRewardCount = students.students.reduce(
    (total, student) => total + student.rewards.pending_teacher_count,
    0,
  );
  const approvedRewardCount = students.students.reduce(
    (total, student) => total + student.rewards.teacher_approved_count + student.rewards.completed_count,
    0,
  );
  return (
    <>
      <section className={styles.workspaceHero}>
        <Link className={styles.secondaryLink} href={`/teach/courses/${students.course.id}`}>
          <ArrowLeft size={17} aria-hidden />
          Course workspace
        </Link>
        <div className={styles.workspaceTitleBlock}>
          <p className={styles.eyebrow}>{statusLabel(students.course.lifecycle_status)}</p>
          <h2>Student progress</h2>
          <p className={styles.muted}>
            Review enrolled learners, progress support, and reward evidence without pretending lesson completion is stored.
          </p>
        </div>
      </section>

      <section className={styles.summaryGrid}>
        <SummaryCard icon={<Users size={20} aria-hidden />} label="Learners" value={students.total} tone={students.total ? "good" : "neutral"} />
        <SummaryCard icon={<FileText size={20} aria-hidden />} label="Course content" value={students.course.content.content_count} tone={students.course.content.has_content ? "good" : "warn"} />
        <SummaryCard icon={<Clock3 size={20} aria-hidden />} label="Pending rewards" value={pendingRewardCount} tone={pendingRewardCount ? "warn" : "neutral"} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Approved evidence" value={approvedRewardCount} tone={approvedRewardCount ? "good" : "neutral"} />
      </section>

      {!students.progress_supported ? (
        <section className={`${styles.warningPanel} ${styles.singlePanel}`}>
          <div className={styles.panelHeader}>
            <AlertCircle size={18} aria-hidden />
            <h2>Progress tracking</h2>
          </div>
          <p>
            Lesson completion is not persisted yet. This view shows enrolled learners and reward evidence that already exists.
          </p>
        </section>
      ) : null}

      <section className={styles.courseSection}>
        <div className={styles.sectionHeader}>
          <div>
            <h2>Learners</h2>
            <p className={styles.muted}>Each card separates access, progress support, and reward evidence.</p>
          </div>
          <span className={`${styles.statusPill} ${students.total ? styles.good : styles.neutral}`}>
            {students.total} total
          </span>
        </div>
        {students.students.length ? (
          <div className={styles.enrollmentList}>
            {students.students.map((student) => (
              <StudentProgressCard key={student.user.id} student={student} />
            ))}
          </div>
        ) : (
          <StatePanel
            detail="No enrolled learners are visible for this course."
            icon={<Users size={22} aria-hidden />}
            title="No students yet"
          />
        )}
      </section>
    </>
  );
}

function StudentProgressCard({ student }: { student: TeacherCourseStudentProgressItem }) {
  const latestCandidate = student.rewards.latest_candidate;
  return (
    <article className={styles.enrollmentCard}>
      <div className={styles.courseTop}>
        <div>
          <p className={styles.eyebrow}>{student.user.email}</p>
          <h3>{student.user.name}</h3>
        </div>
        <span className={`${styles.statusPill} ${styles.good}`}>{statusLabel(student.access_state)}</span>
      </div>

      <div className={styles.metricGrid}>
        <Metric label="Content total" value={student.progress.total_content_count} />
        <Metric label="Pending rewards" value={student.rewards.pending_teacher_count} tone={student.rewards.pending_teacher_count ? "warn" : "neutral"} />
        <Metric label="Approved" value={student.rewards.teacher_approved_count + student.rewards.completed_count} />
        <Metric label="Failed" value={student.rewards.failed_count} tone={student.rewards.failed_count ? "warn" : "neutral"} />
      </div>

      <div className={styles.detailList}>
        <DetailLine label="Roles" value={student.roles.join(", ") || "Learner"} />
        <DetailLine label="Latest request" value={student.latest_join_request_status ? statusLabel(student.latest_join_request_status) : "No request history"} />
        <DetailLine label="Lesson progress" value={student.progress.supported ? "Tracked" : student.progress.note} />
        <DetailLine label="Completion" value={student.progress.completion_percentage === null ? "Not tracked yet" : `${student.progress.completion_percentage}%`} />
        <DetailLine label="Reward candidates" value={String(student.rewards.reward_candidate_count)} />
        {latestCandidate ? (
          <>
            <DetailLine label="Latest evidence" value={`${statusLabel(latestCandidate.event_type)} - ${statusLabel(latestCandidate.status)}`} />
            <DetailLine label="Evidence updated" value={formatDateTime(latestCandidate.updated_at)} />
          </>
        ) : (
          <DetailLine label="Latest evidence" value="No reward evidence yet" />
        )}
      </div>
    </article>
  );
}

function RewardReviewView({
  actionMessage,
  actionState,
  candidates,
  decisionDrafts,
  onDecisionDraftChange,
  onRefresh,
  onStatusFilterChange,
  onSubmitDecision,
  statusFilter,
  students,
}: {
  actionMessage: string | null;
  actionState: ActionState;
  candidates: TeacherRewardCandidate[];
  decisionDrafts: Record<number, RewardDecisionDraft>;
  onDecisionDraftChange: (candidateId: number, draft: RewardDecisionDraft) => void;
  onRefresh: () => void;
  onStatusFilterChange: (status: TeacherRewardCandidateStatusFilter) => void;
  onSubmitDecision: (candidate: TeacherRewardCandidate, event: FormEvent<HTMLFormElement>) => void;
  statusFilter: TeacherRewardCandidateStatusFilter;
  students: TeacherCourseStudentsResponse;
}) {
  const learnersById = useMemo(
    () => new Map(students.students.map((student) => [student.user.id, student.user])),
    [students.students],
  );
  const canApprove = students.course.permissions.can_approve_reward_candidates;
  const canView = canApprove || students.course.permissions.can_view_reward_candidates;
  const pendingShown = candidates.filter((candidate) => candidate.status === "pending_teacher_approval").length;
  const decidedShown = candidates.filter(
    (candidate) => candidate.status === "teacher_approved" || candidate.status === "teacher_rejected",
  ).length;

  return (
    <>
      <section className={styles.workspaceHero}>
        <Link className={styles.secondaryLink} href={`/teach/courses/${students.course.id}`}>
          <ArrowLeft size={17} aria-hidden />
          Course workspace
        </Link>
        <div className={styles.workspaceTitleBlock}>
          <p className={styles.eyebrow}>{statusLabel(students.course.lifecycle_status)}</p>
          <h2>Reward review</h2>
          <p className={styles.muted}>
            Review learner evidence for this course and apply the teacher decision. Platform payout controls stay in admin workflows.
          </p>
        </div>
        <div className={styles.permissionRow} aria-label="Reward permissions">
          <PermissionChip enabled={canView} label="View rewards" />
          <PermissionChip enabled={canApprove} label="Teacher decision" />
        </div>
      </section>

      <section className={styles.summaryGrid}>
        <SummaryCard icon={<Clock3 size={20} aria-hidden />} label="Pending queue" value={students.course.reward_queue.pending_teacher_count} tone={students.course.reward_queue.pending_teacher_count ? "warn" : "neutral"} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Shown now" value={candidates.length} tone={candidates.length ? "good" : "neutral"} />
        <SummaryCard icon={<CheckCircle2 size={20} aria-hidden />} label="Decided shown" value={decidedShown} tone={decidedShown ? "good" : "neutral"} />
        <SummaryCard icon={<AlertCircle size={20} aria-hidden />} label="Failed queue" value={students.course.reward_queue.failed_count} tone={students.course.reward_queue.failed_count ? "warn" : "neutral"} />
      </section>

      <section className={`${styles.warningPanel} ${styles.singlePanel}`}>
        <div className={styles.panelHeader}>
          <ShieldCheck size={18} aria-hidden />
          <h2>Review boundary</h2>
        </div>
        <p>
          This page records teacher approval or rejection only. Financial review remains separated from the course workspace.
        </p>
      </section>

      {actionMessage ? (
        <section className={`${styles.warningPanel} ${styles.singlePanel}`} role="status">
          <div className={styles.panelHeader}>
            <AlertCircle size={18} aria-hidden />
            <h2>Reward update</h2>
          </div>
          <p>{actionMessage}</p>
        </section>
      ) : null}

      <section className={styles.filterPanel}>
        <label>
          <span>Status</span>
          <select
            value={statusFilter}
            onChange={(event) => onStatusFilterChange(event.target.value as TeacherRewardCandidateStatusFilter)}
          >
            {rewardStatusOptions.map((status) => (
              <option key={status} value={status}>
                {status === "all" ? "All statuses" : statusLabel(status)}
              </option>
            ))}
          </select>
        </label>
        <button className={styles.secondaryButton} type="button" onClick={onRefresh}>
          <RefreshCw size={17} aria-hidden />
          Refresh
        </button>
      </section>

      <section className={styles.courseSection}>
        <div className={styles.sectionHeader}>
          <div>
            <h2>Reward candidates</h2>
            <p className={styles.muted}>
              {statusFilter === "all" ? "Showing all visible candidates." : `Showing ${statusLabel(statusFilter)} candidates.`}
            </p>
          </div>
          <span className={`${styles.statusPill} ${pendingShown ? styles.warn : styles.neutral}`}>
            {pendingShown} pending shown
          </span>
        </div>
        {candidates.length ? (
          <div className={styles.enrollmentList}>
            {candidates.map((candidate) => (
              <RewardCandidateCard
                actionState={actionState}
                canApprove={canApprove}
                candidate={candidate}
                draft={decisionDrafts[candidate.id] || defaultRewardDecisionDraft}
                key={candidate.id}
                learner={learnersById.get(candidate.student_user_id) || null}
                onDraftChange={(draft) => onDecisionDraftChange(candidate.id, draft)}
                onSubmit={(event) => onSubmitDecision(candidate, event)}
              />
            ))}
          </div>
        ) : (
          <StatePanel
            detail="No reward candidates match this course filter."
            icon={<CheckCircle2 size={22} aria-hidden />}
            title="No matching candidates"
          />
        )}
      </section>
    </>
  );
}

function RewardCandidateCard({
  actionState,
  canApprove,
  candidate,
  draft,
  learner,
  onDraftChange,
  onSubmit,
}: {
  actionState: ActionState;
  canApprove: boolean;
  candidate: TeacherRewardCandidate;
  draft: RewardDecisionDraft;
  learner: TeacherEnrollmentUserSummary | null;
  onDraftChange: (draft: RewardDecisionDraft) => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
}) {
  const canDecide = canApprove && candidate.status === "pending_teacher_approval";
  return (
    <article className={styles.enrollmentCard}>
      <div className={styles.courseTop}>
        <div>
          <p className={styles.eyebrow}>{learner?.email || "Learner details unavailable"}</p>
          <h3>{learner?.name || "Learner unavailable"}</h3>
        </div>
        <span className={`${styles.statusPill} ${styles[rewardCandidateTone(candidate.status)]}`}>
          {statusLabel(candidate.status)}
        </span>
      </div>

      <div className={styles.detailList}>
        <DetailLine label="Event" value={statusLabel(candidate.event_type)} />
        <DetailLine label="Evidence" value={summarizeEvidence(candidate.evidence)} />
        <DetailLine label="Source" value={statusLabel(candidate.source_scope)} />
        <DetailLine label="Created" value={formatDateTime(candidate.created_at)} />
        <DetailLine label="Updated" value={formatDateTime(candidate.updated_at)} />
        {candidate.teacher_decided_at ? <DetailLine label="Teacher decided" value={formatDateTime(candidate.teacher_decided_at)} /> : null}
        {candidate.teacher_decision_reason ? <DetailLine label="Teacher reason" value={candidate.teacher_decision_reason} /> : null}
      </div>

      {canDecide ? (
        <form className={styles.decisionForm} onSubmit={onSubmit}>
          <label>
            <span>Teacher decision</span>
            <select
              disabled={actionState === "saving"}
              onChange={(event) =>
                onDraftChange({
                  ...draft,
                  status: event.target.value as TeacherRewardCandidateDecisionStatus,
                })
              }
              value={draft.status}
            >
              <option value="teacher_approved">Approve evidence</option>
              <option value="teacher_rejected">Reject evidence</option>
            </select>
          </label>
          <label>
            <span>Reason</span>
            <textarea
              disabled={actionState === "saving"}
              onChange={(event) => onDraftChange({ ...draft, reason: event.target.value })}
              placeholder="Decision reason"
              rows={3}
              value={draft.reason}
            />
          </label>
          <button className={styles.primaryButton} disabled={actionState === "saving"} type="submit">
            <Send size={17} aria-hidden />
            Apply teacher decision
          </button>
        </form>
      ) : (
        <p className={styles.muted}>
          {candidate.status === "pending_teacher_approval"
            ? "This session can view the candidate but cannot apply the teacher decision."
            : "Teacher review is already recorded or this candidate has moved to later processing."}
        </p>
      )}
    </article>
  );
}

function WorkspaceActionPanel({ workspace }: { workspace: TeacherCourseWorkspaceResponse }) {
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
            <span className={`${styles.smallIcon} ${action.enabled ? styles.good : styles.neutral}`}>{action.icon}</span>
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

function ChapterList({ chapters }: { chapters: TeacherCourseWorkspaceResponse["chapters"] }) {
  if (!chapters.length) {
    return (
      <section className={`${styles.panel} ${styles.singlePanel}`}>
        <div className={styles.panelHeader}>
          <BookOpen size={20} aria-hidden />
          <h2>No chapters</h2>
        </div>
        <p className={styles.muted}>Create the first chapter after the content-authoring route is built.</p>
      </section>
    );
  }

  return (
    <div className={styles.chapterList}>
      {chapters.map((chapter) => (
        <article className={styles.chapterCard} key={chapter.id}>
          <div className={styles.courseTop}>
            <div>
              <p className={styles.eyebrow}>Chapter {chapter.order}</p>
              <h3>{chapter.title}</h3>
            </div>
            <span className={`${styles.statusPill} ${styles.neutral}`}>{chapter.contents.length} item{chapter.contents.length === 1 ? "" : "s"}</span>
          </div>
          {chapter.contents.length ? (
            <div className={styles.contentList}>
              {chapter.contents.map((content) => (
                <ContentRow content={content} key={content.id} />
              ))}
            </div>
          ) : (
            <p className={styles.muted}>No content items in this chapter yet.</p>
          )}
        </article>
      ))}
    </div>
  );
}

function ContentRow({ content }: { content: TeacherCourseWorkspaceContent }) {
  const tone = content.display_state === "failed_processing" ? "warn" : content.display_state === "ready" ? "good" : "neutral";
  return (
    <article className={styles.contentRow}>
      <div>
        <strong>{statusLabel(content.content_type)}</strong>
        <p>
          Order {content.order} - {content.data_present ? "Data recorded" : "No stored data"} - {statusLabel(content.publication_status)}
        </p>
      </div>
      <span className={`${styles.statusPill} ${styles[tone]}`}>{statusLabel(content.display_state)}</span>
      {content.processing_status ? <small>{statusLabel(content.processing_status)}</small> : null}
      {content.processing_error ? <p className={styles.reviewNote}>{content.processing_error}</p> : null}
    </article>
  );
}

function ApplicationPanel({
  application,
  canSubmitApplication,
}: {
  application: TeacherApplication | null;
  canSubmitApplication: boolean;
}) {
  const config = applicationConfig(application, canSubmitApplication);
  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        {config.icon}
        <h2>{config.title}</h2>
      </div>
      <p className={styles.muted}>{config.detail}</p>
      {application?.decision_reason ? <p className={styles.reviewNote}>{application.decision_reason}</p> : null}
      {config.href ? (
        <Link className={styles.primaryLink} href={config.href}>
          <Send size={17} aria-hidden />
          {config.action}
        </Link>
      ) : null}
    </section>
  );
}

function PriorityList({ totals }: { totals: ReturnType<typeof dashboardTotals> }) {
  const items = [
    {
      detail: totals.pendingEnrollmentCount
        ? "Enrollment requests are waiting for a teacher decision."
        : "Enrollment queue is clear.",
      icon: <Users size={17} aria-hidden />,
      label: "Enrollment queue",
      tone: totals.pendingEnrollmentCount ? "warn" : "good",
      value: totals.pendingEnrollmentCount,
    },
    {
      detail: totals.pendingRewardCount
        ? "Reward candidates are waiting for course-scoped review."
        : "Reward review queue is clear.",
      icon: <Trophy size={17} aria-hidden />,
      label: "Reward review",
      tone: totals.pendingRewardCount ? "warn" : "good",
      value: totals.pendingRewardCount,
    },
    {
      detail: totals.unhealthyCourseCount
        ? "Archived, suspended, or empty courses need attention before learners rely on them."
        : "Visible courses have usable lifecycle and content signals.",
      icon: <ShieldCheck size={17} aria-hidden />,
      label: "Course health",
      tone: totals.unhealthyCourseCount ? "warn" : "good",
      value: totals.unhealthyCourseCount,
    },
  ];

  return (
    <div className={styles.priorityList}>
      {items.map((item) => (
        <article className={styles.priorityItem} key={item.label}>
          <span className={`${styles.smallIcon} ${styles[item.tone]}`}>{item.icon}</span>
          <div>
            <strong>{item.value} {item.label}</strong>
            <p>{item.detail}</p>
          </div>
        </article>
      ))}
    </div>
  );
}

function CourseGrid({
  courses,
  emptyDetail,
}: {
  courses: TeacherCourseDashboardItem[];
  emptyDetail: string;
}) {
  if (!courses.length) {
    return (
      <section className={`${styles.panel} ${styles.singlePanel}`}>
        <div className={styles.panelHeader}>
          <BookOpen size={20} aria-hidden />
          <h2>No courses</h2>
        </div>
        <p className={styles.muted}>{emptyDetail}</p>
      </section>
    );
  }

  return (
    <div className={styles.courseGrid}>
      {courses.map((course) => (
        <CourseCard course={course} key={course.id} />
      ))}
    </div>
  );
}

function CourseCard({ course }: { course: TeacherCourseDashboardItem }) {
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

function SummaryCard({
  icon,
  label,
  tone = "neutral",
  value,
}: {
  icon: ReactNode;
  label: string;
  tone?: "good" | "neutral" | "warn";
  value: number;
}) {
  return (
    <article className={styles.summaryCard}>
      <span className={`${styles.smallIcon} ${styles[tone]}`}>{icon}</span>
      <strong>{value}</strong>
      <span>{label}</span>
    </article>
  );
}

function Metric({
  label,
  tone = "neutral",
  value,
}: {
  label: string;
  tone?: "neutral" | "warn";
  value: number;
}) {
  return (
    <div className={`${styles.metricCard} ${styles[tone]}`}>
      <strong>{value}</strong>
      <span>{label}</span>
    </div>
  );
}

function DetailLine({ label, value }: { label: string; value: string }) {
  return (
    <div className={styles.detailLine}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function PermissionChip({ enabled, label }: { enabled: boolean; label: string }) {
  return <span className={`${styles.permissionChip} ${enabled ? styles.good : styles.neutral}`}>{label}</span>;
}

function StatePanel({
  action,
  detail,
  icon,
  title,
}: {
  action?: ReactNode;
  detail: string;
  icon: ReactNode;
  title: string;
}) {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`} aria-live="polite">
      <div className={styles.panelHeader}>
        {icon}
        <h2>{title}</h2>
      </div>
      <p className={styles.muted}>{detail}</p>
      {action}
    </section>
  );
}

function StatusLine({
  icon,
  label,
  tone,
}: {
  icon: ReactNode;
  label: string;
  tone: "good" | "neutral" | "warn";
}) {
  return (
    <span className={`${styles.statusLine} ${styles[tone]}`}>
      {icon}
      {label}
    </span>
  );
}

function routeNotice(error: RouteError | null): ShellNotice | null {
  if (!error) {
    return null;
  }

  return {
    actionHref: error.status === 401 ? "/login?redirect=/teach" : undefined,
    actionLabel: error.status === 401 ? "Sign in" : undefined,
    message: error.message,
    title: "Teaching workspace unavailable",
    tone: "error",
  };
}

function normalizeRouteError(error: unknown): RouteError {
  if (error instanceof TeacherRequestError || error instanceof SessionRequestError) {
    return {
      code: error.code,
      message: error.message,
      status: error.status,
    };
  }

  return {
    code: "unexpected_error",
    message: "The teaching workspace request failed before RustLearn could finish loading.",
    status: 0,
  };
}

function dashboardTotals(courses: TeacherCourseDashboardItem[]) {
  return courses.reduce(
    (totals, course) => {
      const pendingEnrollmentCount =
        course.roster.pending_join_request_count + course.roster.waitlisted_join_request_count;
      totals.courseCount += 1;
      totals.contentCount += course.content.content_count;
      totals.pendingEnrollmentCount += pendingEnrollmentCount;
      totals.pendingRewardCount += course.reward_queue.pending_teacher_count;
      if (!course.content.has_content || course.lifecycle_status === "archived" || course.lifecycle_status === "suspended") {
        totals.unhealthyCourseCount += 1;
      }
      return totals;
    },
    {
      contentCount: 0,
      courseCount: 0,
      pendingEnrollmentCount: 0,
      pendingRewardCount: 0,
      unhealthyCourseCount: 0,
    },
  );
}

function workspaceSummary(workspace: TeacherCourseWorkspaceResponse) {
  return workspace.chapters.reduce(
    (totals, chapter) => {
      totals.contentCount += chapter.contents.length;
      for (const content of chapter.contents) {
        if (content.display_state === "failed_processing") {
          totals.failedProcessingCount += 1;
        }
      }
      return totals;
    },
    {
      contentCount: 0,
      failedProcessingCount: 0,
    },
  );
}

function applicationConfig(application: TeacherApplication | null, canSubmitApplication: boolean) {
  if (!application) {
    return {
      action: "Apply to teach",
      detail: canSubmitApplication
        ? "No teacher application is currently on file for this account."
        : "No teacher application is visible for this account.",
      href: canSubmitApplication ? "/teach/apply" : null,
      icon: <Send size={20} aria-hidden />,
      title: "Teacher application",
    };
  }

  if (application.status === "approved") {
    return {
      action: "View application",
      detail: "Your teacher application is approved. Course-scoped teaching work appears in this dashboard.",
      href: "/teach/apply",
      icon: <CheckCircle2 size={20} aria-hidden />,
      title: "Application approved",
    };
  }

  if (application.status === "rejected") {
    return {
      action: canSubmitApplication ? "Start again" : "View application",
      detail: "The latest teacher application was rejected. Review the decision before starting a fresh application.",
      href: "/teach/apply",
      icon: <AlertCircle size={20} aria-hidden />,
      title: "Application rejected",
    };
  }

  if (application.status === "needs_changes") {
    return {
      action: "View feedback",
      detail: "A reviewer requested changes. The application route shows the reason and current resubmission limits.",
      href: "/teach/apply",
      icon: <FileText size={20} aria-hidden />,
      title: "Changes requested",
    };
  }

  return {
    action: "Track review",
    detail: "Your application is in review. Teaching courses will appear here after scope approval.",
    href: "/teach/apply",
    icon: <Clock3 size={20} aria-hidden />,
    title: "Application submitted",
  };
}

function lifecycleTone(status: string) {
  if (status === "published" || status === "approved") {
    return "good";
  }

  if (status === "archived" || status === "suspended" || status === "needs_changes") {
    return "warn";
  }

  return "neutral";
}

function joinRequestTone(status: string) {
  if (status === "approved") {
    return "good";
  }

  if (status === "pending" || status === "waitlisted") {
    return "warn";
  }

  return "neutral";
}

function rewardCandidateTone(status: string) {
  if (
    status === "teacher_approved" ||
    status === "amount_approved" ||
    status === "token_confirmed" ||
    status === "wallet_credited" ||
    status === "completed"
  ) {
    return "good";
  }

  if (
    status === "pending_teacher_approval" ||
    status === "teacher_rejected" ||
    status === "amount_rejected" ||
    status === "needs_reconciliation" ||
    status === "failed"
  ) {
    return "warn";
  }

  return "neutral";
}

function statusLabel(value: string) {
  return value.replace(/_/g, " ").replace(/\b\w/g, (letter) => letter.toUpperCase());
}

function summarizeEvidence(evidence: unknown) {
  if (evidence === null || typeof evidence === "undefined") {
    return "Evidence recorded";
  }

  if (typeof evidence === "string") {
    return evidence.trim() || "Evidence recorded";
  }

  if (typeof evidence === "number" || typeof evidence === "boolean") {
    return String(evidence);
  }

  if (Array.isArray(evidence)) {
    return evidence.length ? `${evidence.length} evidence item${evidence.length === 1 ? "" : "s"}` : "Evidence recorded";
  }

  if (typeof evidence === "object") {
    const entries = Object.entries(evidence as Record<string, unknown>)
      .filter(([key, value]) => isSafeEvidenceKey(key) && isSimpleEvidenceValue(value))
      .slice(0, 3)
      .map(([key, value]) => `${statusLabel(key)}: ${String(value)}`);
    return entries.length ? entries.join(", ") : "Evidence recorded";
  }

  return "Evidence recorded";
}

function isSafeEvidenceKey(key: string) {
  return !/(^id$|_id$|user_id|student|learner|candidate)/i.test(key);
}

function isSimpleEvidenceValue(value: unknown) {
  return typeof value === "string" || typeof value === "number" || typeof value === "boolean";
}

function formatDateTime(value: string) {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return "Date unavailable";
  }

  return date.toLocaleString(undefined, {
    dateStyle: "medium",
    timeStyle: "short",
  });
}
