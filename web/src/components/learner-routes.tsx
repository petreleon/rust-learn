"use client";

import {
  AlertTriangle,
  ArrowLeft,
  BookOpen,
  CheckCircle,
  CreditCard,
  FileText,
  Loader2,
  LogIn,
  RefreshCw,
  Search,
  Trophy,
} from "lucide-react";
import Link from "next/link";
import { type FormEvent, type ReactNode, useCallback, useEffect, useMemo, useState } from "react";
import { ProductShell, type ShellNotice } from "@/components/product-shell";
import {
  fetchCourseCatalog,
  fetchCourseDetail,
  fetchCourseLearning,
  fetchLearnerDashboard,
  fetchLearnerWallet,
  fetchRewardHistory,
  LearnerRequestError,
  linkMyWallet,
  requestCourseJoin,
  type CourseCatalogDetail,
  type CourseCatalogItem,
  type CourseCatalogResponse,
  type CourseLearningContent,
  type CourseLearningResponse,
  type LearnerDashboardSnapshot,
  type RewardHistoryEntry,
  type WalletSummary,
} from "@/lib/learner";
import {
  clearStoredSessionToken,
  fetchCurrentSession,
  readStoredSessionToken,
  type CurrentSession,
  SessionRequestError,
} from "@/lib/session";
import styles from "./learner-routes.module.css";

type LoadState = "idle" | "loading" | "success" | "error";
type LearnerProductRouteKind = "courses" | "rewards" | "wallet";
type LearnerRouteKind = "dashboard" | LearnerProductRouteKind;
type EnrollmentStatusFilter = "all" | "available" | "pending" | "waitlisted" | "enrolled" | "rejected" | "unavailable";
type RewardStatusFilter =
  | "all"
  | "pending_teacher_approval"
  | "amount_approved"
  | "token_pending"
  | "token_confirmed"
  | "wallet_credited"
  | "needs_reconciliation"
  | "failed";

type RouteError = {
  code: string;
  message: string;
  status: number;
};

export function LearnerDashboardRoute() {
  const [hasToken, setHasToken] = useState(false);
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [dashboard, setDashboard] = useState<LearnerDashboardSnapshot | null>(null);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [error, setError] = useState<RouteError | null>(null);

  const loadRoute = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setDashboard(null);
      setError(null);
      setLoadState("idle");
      return;
    }

    setHasToken(true);
    setLoadState("loading");
    setError(null);

    try {
      const [nextSession, nextDashboard] = await Promise.all([
        fetchCurrentSession({ token }),
        fetchLearnerDashboard({ token }),
      ]);
      setSession(nextSession);
      setDashboard(nextDashboard);
      setLoadState("success");
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (requestError.status === 401 || requestError.status === 404) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setSession(null);
      setDashboard(null);
      setError({
        code: requestError.code,
        message: requestError.message,
        status: requestError.status,
      });
      setLoadState("error");
    }
  }, []);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadRoute(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadRoute]);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setDashboard(null);
    setError(null);
    setLoadState("idle");
  }

  const statusLabel = loadState === "loading" ? "Loading" : session ? "Dashboard ready" : "Sign in required";

  return (
    <ProductShell
      activeNav="learn"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { label: "Learner" },
      ]}
      description="Continue learning, inspect rewards, and keep your wallet ready."
      eyebrow="Learner"
      isSignedIn={hasToken || Boolean(session)}
      notice={learnerNotice(error, "dashboard")}
      onSignOut={signOut}
      session={session}
      statusItems={<StatusPill label={statusLabel} tone={session ? "good" : "neutral"} />}
      title="Learner dashboard"
    >
      {loadState === "idle" && !session ? <SignedOutState redirect="/learn" /> : null}
      {loadState === "loading" ? <LoadingState /> : null}
      {error ? <ErrorState error={error} onRetry={loadRoute} redirect="/learn" /> : null}
      {loadState === "success" && session && dashboard ? (
        <LearnerDashboardContent dashboard={dashboard} onRefresh={loadRoute} session={session} />
      ) : null}
    </ProductShell>
  );
}

export function LearnerProductRoute({ kind }: { kind: LearnerProductRouteKind }) {
  const config = routeConfig[kind];
  const [hasToken, setHasToken] = useState(false);
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [error, setError] = useState<RouteError | null>(null);
  const [actionNotice, setActionNotice] = useState<ShellNotice | null>(null);
  const [catalog, setCatalog] = useState<CourseCatalogResponse | null>(null);
  const [courseEnrollmentFilter, setCourseEnrollmentFilter] = useState<EnrollmentStatusFilter>("all");
  const [courseRewardOnly, setCourseRewardOnly] = useState(false);
  const [courseSearch, setCourseSearch] = useState("");
  const [courseSearchInput, setCourseSearchInput] = useState("");
  const [joiningCourseId, setJoiningCourseId] = useState<number | null>(null);
  const [rewards, setRewards] = useState<RewardHistoryEntry[]>([]);
  const [rewardStatus, setRewardStatus] = useState<RewardStatusFilter>("all");
  const [wallet, setWallet] = useState<WalletSummary | null>(null);
  const [walletLinking, setWalletLinking] = useState(false);

  const loadRoute = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setError(null);
      setCatalog(null);
      setRewards([]);
      setWallet(null);
      setLoadState("idle");
      return;
    }

    setHasToken(true);
    setLoadState("loading");
    setError(null);

    try {
      const sessionPromise = fetchCurrentSession({ token });
      const rewardsPromise =
        kind === "rewards"
          ? fetchRewardHistory({
              status: rewardStatus === "all" ? undefined : rewardStatus,
              token,
            })
          : Promise.resolve([]);
      const catalogPromise =
        kind === "courses"
          ? fetchCourseCatalog({
              enrollmentStatus: courseEnrollmentFilter === "all" ? undefined : courseEnrollmentFilter,
              rewardAvailable: courseRewardOnly ? true : undefined,
              search: courseSearch,
              token,
            })
          : Promise.resolve(null);
      const walletPromise = kind === "wallet" ? fetchLearnerWallet({ token }) : Promise.resolve(null);

      const [nextSession, nextRewards, nextCatalog, nextWalletSnapshot] = await Promise.all([
        sessionPromise,
        rewardsPromise,
        catalogPromise,
        walletPromise,
      ]);

      setSession(nextSession);
      setRewards(kind === "wallet" ? nextWalletSnapshot?.reward_history || [] : nextRewards);
      setCatalog(nextCatalog);
      setWallet(nextWalletSnapshot?.wallet || null);
      setLoadState("success");
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (requestError.status === 401 || requestError.status === 404) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setSession(null);
      setError({
        code: requestError.code,
        message: requestError.message,
        status: requestError.status,
      });
      setLoadState("error");
    }
  }, [courseEnrollmentFilter, courseRewardOnly, courseSearch, kind, rewardStatus]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadRoute(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadRoute]);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setError(null);
    setActionNotice(null);
    setCatalog(null);
    setRewards([]);
    setWallet(null);
    setLoadState("idle");
  }

  async function linkWallet() {
    const token = readStoredSessionToken();
    if (!token) {
      setLoadState("idle");
      return;
    }

    setWalletLinking(true);
    setActionNotice(null);
    try {
      const result = await linkMyWallet({ token });
      setWallet(result.wallet);
      setActionNotice({
        message: result.created
          ? "Approved rewards can now be credited to this wallet."
          : "This account already had a RustLearn wallet.",
        title: result.created ? "Wallet linked" : "Wallet ready",
        tone: "success",
      });
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
      setWalletLinking(false);
    }
  }

  async function requestJoin(course: CourseCatalogItem) {
    const token = readStoredSessionToken();
    if (!token) {
      setLoadState("idle");
      return;
    }

    setJoiningCourseId(course.id);
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
      if (requestError.status === 401 || requestError.status === 404) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setActionNotice({
        message: requestError.message,
        title: requestError.code,
        tone: requestError.code === "timeout" || requestError.code === "network_error" ? "warn" : "error",
      });
    } finally {
      setJoiningCourseId(null);
    }
  }

  function applyCourseSearch(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setCourseSearch(courseSearchInput.trim());
  }

  function clearCourseFilters() {
    setCourseEnrollmentFilter("all");
    setCourseRewardOnly(false);
    setCourseSearch("");
    setCourseSearchInput("");
  }

  const notice = actionNotice || learnerNotice(error, kind);
  const statusLabel = loadState === "loading" ? "Loading" : session ? "Learner access" : "Sign in required";

  return (
    <ProductShell
      activeNav="learn"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/learn", label: "Learner" },
        { label: config.title },
      ]}
      description={config.description}
      eyebrow="Learner"
      isSignedIn={hasToken || Boolean(session)}
      notice={notice}
      onSignOut={signOut}
      session={session}
      statusItems={<StatusPill label={statusLabel} tone={session ? "good" : "neutral"} />}
      title={config.title}
    >
      {loadState === "idle" && !session ? <SignedOutState redirect={config.href} /> : null}
      {loadState === "loading" ? <LoadingState /> : null}
      {error ? <ErrorState error={error} onRetry={loadRoute} redirect={config.href} /> : null}
      {loadState === "success" && session && kind === "courses" ? (
        <CoursesContent
          catalog={catalog}
          enrollmentFilter={courseEnrollmentFilter}
          joiningCourseId={joiningCourseId}
          onApplySearch={applyCourseSearch}
          onChangeEnrollmentFilter={setCourseEnrollmentFilter}
          onChangeRewardOnly={setCourseRewardOnly}
          onChangeSearchInput={setCourseSearchInput}
          onClearFilters={clearCourseFilters}
          onRefresh={loadRoute}
          onRequestJoin={requestJoin}
          rewardOnly={courseRewardOnly}
          search={courseSearch}
          searchInput={courseSearchInput}
          session={session}
        />
      ) : null}
      {loadState === "success" && session && kind === "rewards" ? (
        <RewardsContent
          filter={rewardStatus}
          onChangeFilter={(nextFilter) => setRewardStatus(nextFilter)}
          rewards={rewards}
        />
      ) : null}
      {loadState === "success" && session && kind === "wallet" ? (
        <WalletContent
          linking={walletLinking}
          onLinkWallet={linkWallet}
          onRefresh={loadRoute}
          rewards={rewards}
          wallet={wallet}
        />
      ) : null}
    </ProductShell>
  );
}

export function LearnerCourseDetailRoute({ courseId }: { courseId: string }) {
  const numericCourseId = Number(courseId);
  const validCourseId = Number.isInteger(numericCourseId) && numericCourseId > 0;
  const [hasToken, setHasToken] = useState(false);
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [detail, setDetail] = useState<CourseCatalogDetail | null>(null);
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
      setError(null);
      setLoadState("idle");
      return;
    }

    if (!validCourseId) {
      setHasToken(true);
      setSession(null);
      setDetail(null);
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

      const nextDetail = await fetchCourseDetail({ courseId: numericCourseId, token });
      setDetail(nextDetail);
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
        <CourseDetailContent detail={detail} joining={joining} onRequestJoin={requestJoin} />
      ) : null}
    </ProductShell>
  );
}

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
      setSelectedContentId(nextLearning.active_content_id);
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
          learning={learning}
          selectedContent={selectedContent}
          selectedContentId={selectedContentId}
          onSelectContent={setSelectedContentId}
        />
      ) : null}
    </ProductShell>
  );
}

function LearnerDashboardContent({
  dashboard,
  onRefresh,
  session,
}: {
  dashboard: LearnerDashboardSnapshot;
  onRefresh: () => void;
  session: CurrentSession;
}) {
  const enrolledCourses = dashboard.enrolled_catalog.courses;
  const recommendedCourses = dashboard.recommended_catalog.courses;
  const continueCourse = enrolledCourses.find((course) => course.access.can_view_content && course.content.has_content) || null;
  const rewardSummary = useMemo(() => summarizeRewards(dashboard.reward_history), [dashboard.reward_history]);
  const progressLabel = continueCourse ? "Not tracked" : "No lesson";

  return (
    <>
      <section className={styles.dashboardGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Enrolled courses" value={dashboard.enrolled_catalog.total} />
        <SummaryCard icon={<CheckCircle size={20} aria-hidden />} label="Progress" value={progressLabel} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Recent rewards" value={dashboard.reward_history.length} />
        <SummaryCard icon={<CreditCard size={20} aria-hidden />} label="Wallet" value={dashboard.wallet ? "Linked" : "Unlinked"} />
      </section>

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Next step</h2>
          <button className={styles.iconAction} aria-label="Refresh learner dashboard" type="button" onClick={onRefresh}>
            <RefreshCw size={18} aria-hidden />
          </button>
        </div>
        {continueCourse ? (
          <article className={styles.itemCard}>
            <div className={styles.itemHeader}>
              <h3>{continueCourse.title}</h3>
              <StatusPill label="Continue" tone="good" />
            </div>
            <p className={styles.muted}>
              Progress tracking is not available yet. You can still open the next available lesson from the course outline.
            </p>
            <div className={styles.metaRow}>
              <span>{courseOrganizationLabel(continueCourse)}</span>
              <span>{courseContentLabel(continueCourse)}</span>
              <span>{continueCourse.rewards.available ? "Rewards available" : "No active rewards"}</span>
            </div>
            <div className={styles.actionRow}>
              <Link className={styles.primaryLink} href={`/courses/${continueCourse.id}/learn`}>
                <BookOpen size={18} aria-hidden />
                Continue learning
              </Link>
              <Link className={styles.secondaryLink} href={`/courses/${continueCourse.id}`}>
                Course details
              </Link>
            </div>
          </article>
        ) : (
          <EmptyState
            actionHref="/courses"
            actionLabel={enrolledCourses.length ? "Open courses" : "Find courses"}
            detail={
              enrolledCourses.length
                ? "Your current courses do not have viewable content yet."
                : "Choose a course from the catalog to start building your learner workspace."
            }
            title={enrolledCourses.length ? "No lesson ready" : "Start with a course"}
          />
        )}
      </section>

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Enrolled courses</h2>
          <StatusPill label={`${enrolledCourses.length} shown`} tone="neutral" />
        </div>
        {enrolledCourses.length ? (
          <div className={styles.itemGrid}>
            {enrolledCourses.map((course) => (
              <DashboardCourseCard course={course} key={course.id} />
            ))}
          </div>
        ) : (
          <EmptyState
            actionHref="/courses"
            actionLabel="Browse catalog"
            detail="Enrolled courses appear here after course staff approve access or assign you directly."
            title="No enrolled courses"
          />
        )}
      </section>

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Rewards and wallet</h2>
          <StatusPill label={rewardSummary.needsHelp ? "Needs attention" : "Ready"} tone={rewardSummary.needsHelp ? "warn" : "neutral"} />
        </div>
        <div className={styles.itemGrid}>
          <DashboardRewardPanel rewardCount={dashboard.reward_history.length} summary={rewardSummary} />
          <DashboardWalletPanel wallet={dashboard.wallet} />
        </div>
      </section>

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Recommended courses</h2>
          <StatusPill label={`${recommendedCourses.length} available`} tone="neutral" />
        </div>
        {recommendedCourses.length ? (
          <div className={styles.itemGrid}>
            {recommendedCourses.map((course) => (
              <DashboardCourseCard course={course} key={course.id} />
            ))}
          </div>
        ) : (
          <EmptyState
            detail={
              session.courses.length
                ? "No additional available courses are visible right now."
                : "Published courses will appear here when they are visible to learners."
            }
            title="No recommendations yet"
          />
        )}
      </section>
    </>
  );
}

function DashboardCourseCard({ course }: { course: CourseCatalogItem }) {
  return (
    <article className={styles.itemCard}>
      <div className={styles.itemHeader}>
        <h3>{course.title}</h3>
        <StatusPill label={humanize(course.enrollment.state)} tone={enrollmentTone(course.enrollment.state)} />
      </div>
      <div className={styles.metaRow}>
        <StatusPill label={humanize(course.lifecycle_status)} tone={lifecycleTone(course.lifecycle_status)} />
        <span>{courseOrganizationLabel(course)}</span>
        <span>{courseTeacherLabel(course)}</span>
        <span>{courseContentLabel(course)}</span>
      </div>
      <p className={styles.muted}>{course.enrollment.reason || "Open course details for enrollment and reward requirements."}</p>
      <div className={styles.actionRow}>
        {course.access.can_view_content && course.content.has_content ? (
          <Link className={styles.primaryLink} href={`/courses/${course.id}/learn`}>
            <BookOpen size={18} aria-hidden />
            Learn
          </Link>
        ) : null}
        <Link className={styles.secondaryLink} href={`/courses/${course.id}`}>
          Details
        </Link>
      </div>
    </article>
  );
}

function DashboardRewardPanel({
  rewardCount,
  summary,
}: {
  rewardCount: number;
  summary: ReturnType<typeof summarizeRewards>;
}) {
  return (
    <article className={styles.itemCard}>
      <div className={styles.itemHeader}>
        <h3>Reward status</h3>
        <StatusPill label={`${rewardCount} recent`} tone={rewardCount ? "good" : "neutral"} />
      </div>
      <div className={styles.detailList}>
        <span>{summary.pendingTeacher} waiting for teacher review</span>
        <span>{summary.processing} processing or amount-approved</span>
        <span>{summary.credited} credited to wallet</span>
        <span>{summary.needsHelp} need help</span>
      </div>
      <div className={styles.actionRow}>
        <Link className={styles.secondaryLink} href="/rewards">
          View rewards
        </Link>
      </div>
    </article>
  );
}

function DashboardWalletPanel({ wallet }: { wallet: WalletSummary | null }) {
  return (
    <article className={styles.itemCard}>
      <div className={styles.itemHeader}>
        <h3>Wallet readiness</h3>
        <StatusPill label={wallet ? "Linked" : "Unlinked"} tone={wallet ? "good" : "warn"} />
      </div>
      {wallet ? (
        <>
          <p className={styles.muted}>Approved rewards can be credited to this wallet.</p>
          <strong className={styles.walletValue}>{wallet.value}</strong>
          <div className={styles.metaRow}>
            <span>{wallet.owner_type}</span>
            <span>{wallet.organization_id ? "Organization wallet" : "Personal wallet"}</span>
          </div>
        </>
      ) : (
        <p className={styles.muted}>Link your RustLearn wallet before approved rewards can be credited.</p>
      )}
      <div className={styles.actionRow}>
        <Link className={wallet ? styles.secondaryLink : styles.primaryLink} href="/wallet">
          {wallet ? "Open wallet" : "Link wallet"}
        </Link>
      </div>
    </article>
  );
}

function CourseLearningContentView({
  learning,
  onSelectContent,
  selectedContent,
  selectedContentId,
}: {
  learning: CourseLearningResponse;
  onSelectContent: (contentId: number) => void;
  selectedContent: CourseLearningContent | null;
  selectedContentId: number | null;
}) {
  const allContents = learning.chapters.flatMap((chapter) => chapter.contents);
  const selectedIndex = selectedContent
    ? allContents.findIndex((content) => content.id === selectedContent.id)
    : -1;
  const previousContent = selectedIndex > 0 ? allContents[selectedIndex - 1] : null;
  const nextContent = selectedIndex >= 0 && selectedIndex < allContents.length - 1 ? allContents[selectedIndex + 1] : null;

  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Chapters" value={learning.chapters.length} />
        <SummaryCard icon={<FileText size={20} aria-hidden />} label="Lessons" value={allContents.length} />
        <SummaryCard
          icon={<CheckCircle size={20} aria-hidden />}
          label="Progress"
          value={learning.progress_supported ? "Tracked" : "Local only"}
        />
      </section>

      <section className={styles.learningLayout}>
        <aside className={styles.lessonOutline} aria-label="Course outline">
          <div className={styles.panelHeader}>
            <BookOpen size={20} aria-hidden />
            <h2>Outline</h2>
          </div>
          {learning.chapters.length ? (
            learning.chapters.map((chapter) => (
              <div className={styles.lessonChapter} key={chapter.id}>
                <h3>{chapter.title}</h3>
                {chapter.contents.length ? (
                  <div className={styles.lessonList}>
                    {chapter.contents.map((content) => (
                      <button
                        aria-pressed={selectedContentId === content.id}
                        className={`${styles.lessonButton} ${selectedContentId === content.id ? styles.activeLesson : ""}`}
                        key={content.id}
                        type="button"
                        onClick={() => onSelectContent(content.id)}
                      >
                        <span>{content.order + 1}. {humanize(content.content_type)}</span>
                        <StatusPill label={humanize(content.display_state)} tone={contentStateTone(content.display_state)} />
                      </button>
                    ))}
                  </div>
                ) : (
                  <p className={styles.muted}>No lessons in this chapter yet.</p>
                )}
              </div>
            ))
          ) : (
            <p className={styles.muted}>No course outline is available yet.</p>
          )}
        </aside>

        <section className={styles.lessonReader}>
          {selectedContent ? (
            <>
              <div className={styles.sectionHeader}>
                <h2>{humanize(selectedContent.content_type)}</h2>
                <StatusPill label={humanize(selectedContent.display_state)} tone={contentStateTone(selectedContent.display_state)} />
              </div>
              <LessonContentBody content={selectedContent} />
              <div className={styles.actionRow}>
                <button
                  className={styles.secondaryButton}
                  disabled={!previousContent}
                  type="button"
                  onClick={() => previousContent ? onSelectContent(previousContent.id) : undefined}
                >
                  <ArrowLeft size={18} aria-hidden />
                  Previous
                </button>
                <button
                  className={styles.secondaryButton}
                  disabled={!nextContent}
                  type="button"
                  onClick={() => nextContent ? onSelectContent(nextContent.id) : undefined}
                >
                  Next
                </button>
              </div>
            </>
          ) : (
            <EmptyState detail="Course content has not been published yet." title="No lesson selected" />
          )}
        </section>
      </section>
    </>
  );
}

function LessonContentBody({ content }: { content: CourseLearningContent }) {
  if (content.display_state === "ready" && isReadableTextContent(content.content_type) && content.data) {
    return <article className={styles.lessonText}>{content.data}</article>;
  }

  const stateCopy = lessonStateCopy(content);
  return (
    <article className={styles.statePanel}>
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h3>{stateCopy.title}</h3>
      </div>
      <p className={styles.muted}>{stateCopy.detail}</p>
      {content.processing_error ? <p className={styles.muted}>Last error: {content.processing_error}</p> : null}
    </article>
  );
}

function CourseDetailContent({
  detail,
  joining,
  onRequestJoin,
}: {
  detail: CourseCatalogDetail;
  joining: boolean;
  onRequestJoin: (course: CourseCatalogItem) => void;
}) {
  const course = detail.course;
  const organizationLabel = course.organizations.map((organization) => organization.name).join(", ") || "Independent";
  const teacherLabel = course.teachers.map((teacher) => teacher.name).join(", ") || "Teacher pending";
  const contentTypes = course.content.content_types.map(humanize).join(", ") || "Content pending";
  const rewardLabel = course.rewards.available
    ? `${course.rewards.active_policy_count} active ${plural(course.rewards.active_policy_count)}`
    : "No active policy";

  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Chapters" value={course.content.chapter_count} />
        <SummaryCard icon={<FileText size={20} aria-hidden />} label="Content items" value={course.content.content_count} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Rewards" value={rewardLabel} />
      </section>

      <section className={styles.catalogPanel}>
        <div className={styles.panelHeader}>
          <BookOpen size={20} aria-hidden />
          <h2>Overview</h2>
          <StatusPill label={humanize(course.lifecycle_status)} tone={course.lifecycle_status === "published" ? "good" : "neutral"} />
        </div>
        <div className={styles.metaRow}>
          <span>{organizationLabel}</span>
          <span>{teacherLabel}</span>
          <span>{contentTypes}</span>
        </div>
        <div className={styles.detailList}>
          <span>{course.enrollment.reason || humanize(course.enrollment.state)}</span>
          {course.rewards.available ? (
            <span>{course.rewards.event_types.map(humanize).join(", ")} rewards</span>
          ) : null}
          {detail.prerequisites.length ? <span>{detail.prerequisites.join(", ")}</span> : null}
        </div>
        <div className={styles.actionRow}>
          <Link className={styles.secondaryLink} href="/courses">
            <ArrowLeft size={18} aria-hidden />
            Courses
          </Link>
          {course.enrollment.can_request_join ? (
            <button className={styles.primaryLink} disabled={joining} type="button" onClick={() => onRequestJoin(course)}>
              {joining ? <Loader2 className={styles.spin} size={18} aria-hidden /> : <CheckCircle size={18} aria-hidden />}
              Request join
            </button>
          ) : null}
          {course.access.can_view_content && course.content.has_content ? (
            <Link className={styles.primaryLink} href={`/courses/${course.id}/learn`}>
              <BookOpen size={18} aria-hidden />
              Start learning
            </Link>
          ) : null}
          {course.access.can_view_rewards ? (
            <Link className={styles.secondaryLink} href="/rewards">
              <Trophy size={18} aria-hidden />
              Rewards
            </Link>
          ) : null}
        </div>
      </section>

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Syllabus</h2>
          <StatusPill label={`${detail.chapters.length} chapters`} tone="neutral" />
        </div>
        {detail.chapters.length ? (
          <div className={styles.itemGrid}>
            {detail.chapters.map((chapter) => (
              <article className={styles.itemCard} key={chapter.id}>
                <div className={styles.itemHeader}>
                  <h3>{chapter.title}</h3>
                  <StatusPill label={`#${chapter.order + 1}`} tone="neutral" />
                </div>
                {chapter.contents.length ? (
                  <div className={styles.detailList}>
                    {chapter.contents.map((content) => (
                      <span key={content.id}>
                        {content.order + 1}. {humanize(content.content_type)}
                      </span>
                    ))}
                  </div>
                ) : (
                  <p className={styles.muted}>No content items yet.</p>
                )}
              </article>
            ))}
          </div>
        ) : (
          <EmptyState detail="Syllabus content has not been added yet." title="No syllabus yet" />
        )}
      </section>
    </>
  );
}

function CoursesContent({
  catalog,
  enrollmentFilter,
  joiningCourseId,
  onApplySearch,
  onChangeEnrollmentFilter,
  onChangeRewardOnly,
  onChangeSearchInput,
  onClearFilters,
  onRefresh,
  onRequestJoin,
  rewardOnly,
  search,
  searchInput,
  session,
}: {
  catalog: CourseCatalogResponse | null;
  enrollmentFilter: EnrollmentStatusFilter;
  joiningCourseId: number | null;
  onApplySearch: (event: FormEvent<HTMLFormElement>) => void;
  onChangeEnrollmentFilter: (filter: EnrollmentStatusFilter) => void;
  onChangeRewardOnly: (checked: boolean) => void;
  onChangeSearchInput: (value: string) => void;
  onClearFilters: () => void;
  onRefresh: () => void;
  onRequestJoin: (course: CourseCatalogItem) => void;
  rewardOnly: boolean;
  search: string;
  searchInput: string;
  session: CurrentSession;
}) {
  const courses = catalog?.courses || [];
  const rewardCourseCount = courses.filter((course) => course.rewards.available).length;
  const activeFilterCount = [search ? 1 : 0, enrollmentFilter !== "all" ? 1 : 0, rewardOnly ? 1 : 0].reduce(
    (total, value) => total + value,
    0,
  );

  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Catalog matches" value={catalog?.total ?? 0} />
        <SummaryCard icon={<CheckCircle size={20} aria-hidden />} label="Current courses" value={session.courses.length} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Reward-ready" value={rewardCourseCount} />
      </section>

      <section className={styles.catalogPanel}>
        <div className={styles.panelHeader}>
          <Search size={20} aria-hidden />
          <h2>Course catalog</h2>
          <button className={styles.iconAction} aria-label="Refresh courses" type="button" onClick={onRefresh}>
            <RefreshCw size={18} aria-hidden />
          </button>
        </div>

        <form className={styles.searchForm} onSubmit={onApplySearch}>
          <label className={styles.searchField}>
            <Search size={18} aria-hidden />
            <span className={styles.srOnly}>Search courses</span>
            <input
              placeholder="Search courses"
              type="search"
              value={searchInput}
              onChange={(event) => onChangeSearchInput(event.target.value)}
            />
          </label>
          <button className={styles.primaryLink} type="submit">
            <Search size={18} aria-hidden />
            Search
          </button>
        </form>

        <div className={styles.filterBar} aria-label="Course filters">
          {enrollmentFilterOptions.map((option) => (
            <button
              aria-pressed={enrollmentFilter === option.value}
              className={`${styles.filterButton} ${enrollmentFilter === option.value ? styles.activeFilter : ""}`}
              key={option.value}
              type="button"
              onClick={() => onChangeEnrollmentFilter(option.value)}
            >
              {option.label}
            </button>
          ))}
          <label className={`${styles.filterToggle} ${rewardOnly ? styles.activeToggle : ""}`}>
            <input
              checked={rewardOnly}
              type="checkbox"
              onChange={(event) => onChangeRewardOnly(event.target.checked)}
            />
            <Trophy size={17} aria-hidden />
            Rewards
          </label>
          {activeFilterCount ? (
            <button className={styles.secondaryButton} type="button" onClick={onClearFilters}>
              Clear
            </button>
          ) : null}
        </div>
      </section>

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Courses</h2>
          <StatusPill label={`${courses.length} shown`} tone="neutral" />
        </div>
        {courses.length ? (
          <div className={styles.itemGrid}>
            {courses.map((course) => (
              <CourseCatalogCard
                course={course}
                emailVerified={session.user.email_verified}
                joining={joiningCourseId === course.id}
                key={course.id}
                onRequestJoin={onRequestJoin}
              />
            ))}
          </div>
        ) : (
          <EmptyState
            action={
              activeFilterCount ? (
                <button className={styles.secondaryButton} type="button" onClick={onClearFilters}>
                  Clear filters
                </button>
              ) : undefined
            }
            detail={
              activeFilterCount
                ? "No visible courses match these filters."
                : "Visible courses will appear after they are published or assigned to you."
            }
            title={activeFilterCount ? "No matching courses" : "No courses visible"}
          />
        )}
      </section>

      {session.courses.length ? (
        <section className={styles.section}>
          <div className={styles.sectionHeader}>
            <h2>Current access</h2>
            <StatusPill label={`${session.courses.length} scopes`} tone="neutral" />
          </div>
          <div className={styles.compactList}>
            {session.courses.slice(0, 4).map((course) => (
              <span key={course.id}>
                {course.title} - {course.roles.length ? course.roles.join(", ") : "Direct access"}
              </span>
            ))}
          </div>
        </section>
      ) : null}
    </>
  );
}

function CourseCatalogCard({
  course,
  emailVerified,
  joining,
  onRequestJoin,
}: {
  course: CourseCatalogItem;
  emailVerified?: boolean;
  joining: boolean;
  onRequestJoin: (course: CourseCatalogItem) => void;
}) {
  const organizationLabel = course.organizations.map((organization) => organization.name).join(", ") || "Independent";
  const teacherLabel = course.teachers.map((teacher) => teacher.name).join(", ") || "Teacher pending";
  const contentLabel = course.content.has_content
    ? `${course.content.chapter_count} chapters, ${course.content.content_count} items`
    : "Content pending";
  const rewardLabel = course.rewards.available
    ? `${course.rewards.active_policy_count} reward ${plural(course.rewards.active_policy_count)}`
    : "No active rewards";
  const emailBlocked = course.enrollment.can_request_join && emailVerified === false;

  return (
    <article className={styles.itemCard}>
      <div className={styles.itemHeader}>
        <h3>{course.title}</h3>
        <StatusPill label={humanize(course.enrollment.state)} tone={enrollmentTone(course.enrollment.state)} />
      </div>
      <div className={styles.metaRow}>
        <StatusPill label={humanize(course.lifecycle_status)} tone={lifecycleTone(course.lifecycle_status)} />
        <span>{organizationLabel}</span>
        <span>{teacherLabel}</span>
      </div>
      <div className={styles.detailList}>
        <span>{contentLabel}</span>
        <span>{rewardLabel}</span>
        {course.enrollment.reason ? <span>{course.enrollment.reason}</span> : null}
        {emailBlocked ? <span className={styles.muted}>Verify your email address before requesting enrollment.</span> : null}
      </div>
      <div className={styles.actionRow}>
        <Link className={styles.secondaryLink} href={`/courses/${course.id}`}>
          <BookOpen size={18} aria-hidden />
          Details
        </Link>
        {course.enrollment.can_request_join ? (
          <button
            className={styles.primaryLink}
            disabled={joining || emailBlocked}
            type="button"
            onClick={() => onRequestJoin(course)}
          >
            {joining ? <Loader2 className={styles.spin} size={18} aria-hidden /> : <CheckCircle size={18} aria-hidden />}
            {emailBlocked ? "Verify email first" : "Request join"}
          </button>
        ) : null}
      </div>
    </article>
  );
}

function RewardsContent({
  filter,
  onChangeFilter,
  rewards,
}: {
  filter: RewardStatusFilter;
  onChangeFilter: (filter: RewardStatusFilter) => void;
  rewards: RewardHistoryEntry[];
}) {
  const summary = useMemo(() => {
    return {
      credited: rewards.filter((reward) => reward.wallet_credit).length,
      needsHelp: rewards.filter((reward) => reward.status === "failed" || reward.status === "needs_reconciliation").length,
      processing: rewards.filter((reward) => reward.status.includes("token") || reward.status === "amount_approved").length,
    };
  }, [rewards]);

  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Reward records" value={rewards.length} />
        <SummaryCard icon={<CreditCard size={20} aria-hidden />} label="Wallet credits" value={summary.credited} />
        <SummaryCard icon={<AlertTriangle size={20} aria-hidden />} label="Need help" value={summary.needsHelp} />
      </section>

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Reward history</h2>
          <StatusPill label={`${summary.processing} processing`} tone="neutral" />
        </div>
        <div className={styles.filterBar} aria-label="Reward status filters">
          {rewardFilterOptions.map((option) => (
            <button
              aria-pressed={filter === option.value}
              className={`${styles.filterButton} ${filter === option.value ? styles.activeFilter : ""}`}
              key={option.value}
              type="button"
              onClick={() => onChangeFilter(option.value)}
            >
              {option.label}
            </button>
          ))}
        </div>

        {rewards.length ? (
          <div className={styles.itemGrid}>
            {rewards.map((reward) => (
              <RewardCard key={reward.reward_candidate_id} reward={reward} />
            ))}
          </div>
        ) : (
          <EmptyState
            action={
              filter === "all" ? undefined : (
                <button className={styles.secondaryButton} type="button" onClick={() => onChangeFilter("all")}>
                  Clear filter
                </button>
              )
            }
            actionHref={filter === "all" ? "/courses" : undefined}
            actionLabel={filter === "all" ? "Check courses" : undefined}
            detail={
              filter === "all"
                ? "Rewards appear after eligible course activity is submitted and reviewed."
                : "There are no rewards in this status right now."
            }
            title={filter === "all" ? "No reward history yet" : "No matching rewards"}
          />
        )}
      </section>
    </>
  );
}

function WalletContent({
  linking,
  onLinkWallet,
  onRefresh,
  rewards,
  wallet,
}: {
  linking: boolean;
  onLinkWallet: () => void;
  onRefresh: () => void;
  rewards: RewardHistoryEntry[];
  wallet: WalletSummary | null;
}) {
  const summary = useMemo(() => summarizeRewards(rewards), [rewards]);
  const pendingCreditCount = rewards.filter(isWalletCreditPending).length;
  const walletScope = wallet?.organization_id ? "Organization wallet" : "Personal wallet";

  const metricsSection = (
    <section className={`${styles.dashboardGrid} ${styles.walletGrid}`}>
      <SummaryCard icon={<CreditCard size={20} aria-hidden />} label="Wallet state" value={wallet ? "Linked" : "Unlinked"} />
      <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Available value" value={wallet?.value || "0"} />
      <SummaryCard icon={<CheckCircle size={20} aria-hidden />} label="Wallet credits" value={summary.credited} />
      <SummaryCard icon={<AlertTriangle size={20} aria-hidden />} label="Pending credits" value={pendingCreditCount} />
    </section>
  );

  const walletSummarySection = (
    <section className={styles.section}>
      <div className={styles.sectionHeader}>
        <h2>Wallet summary</h2>
        <button className={styles.iconAction} aria-label="Refresh wallet" type="button" onClick={onRefresh}>
          <RefreshCw size={18} aria-hidden />
        </button>
      </div>
      {wallet ? (
        <article className={styles.itemCard}>
          <div className={styles.itemHeader}>
            <h3>{walletScope}</h3>
            <StatusPill label="Ready for credits" tone="good" />
          </div>
          <p className={styles.muted}>Approved rewards can be credited here. Deposits and retirements are not available in this UI yet.</p>
          <strong className={styles.walletValue}>{wallet.value}</strong>
          <div className={styles.metaRow}>
            <span>{wallet.owner_type}</span>
            <span>{walletScope}</span>
            <span>{summary.needsHelp ? "Needs review" : "No wallet issues shown"}</span>
          </div>
          <div className={styles.actionRow}>
            <Link className={styles.secondaryLink} href="/rewards">
              View rewards
            </Link>
          </div>
        </article>
      ) : (
        <EmptyState
          action={
            <button className={styles.primaryLink} disabled={linking} onClick={onLinkWallet} type="button">
              {linking ? <Loader2 className={styles.spin} size={18} aria-hidden /> : <CreditCard size={18} aria-hidden />}
              Link wallet
            </button>
          }
          detail="A RustLearn wallet is required before approved rewards can be credited."
          title="Wallet not linked"
        />
      )}
    </section>
  );

  return (
    <>
      {wallet ? metricsSection : walletSummarySection}
      {wallet ? walletSummarySection : metricsSection}

      <section className={styles.section}>
        <div className={styles.sectionHeader}>
          <h2>Reward credit history</h2>
          <StatusPill label={`${rewards.length} recent`} tone={rewards.length ? "good" : "neutral"} />
        </div>
        {rewards.length ? (
          <div className={styles.itemGrid}>
            {rewards.slice(0, 6).map((reward) => (
              <WalletActivityCard key={reward.reward_candidate_id} reward={reward} />
            ))}
          </div>
        ) : (
          <EmptyState
            actionHref="/courses"
            actionLabel="Open courses"
            detail="Reward credits appear after eligible course activity is reviewed and credited."
            title="No wallet activity yet"
          />
        )}
      </section>
    </>
  );
}

function WalletActivityCard({ reward }: { reward: RewardHistoryEntry }) {
  return (
    <article className={styles.itemCard}>
      <div className={styles.itemHeader}>
        <h3>{reward.course_title}</h3>
        <StatusPill label={reward.wallet_credit ? "Wallet credited" : humanRewardStatus(reward.status)} tone={rewardTone(reward.status)} />
      </div>
      <p className={styles.muted}>{rewardNextStep(reward)}</p>
      <div className={styles.metaRow}>
        <span>{humanize(reward.event_type)}</span>
        <span>{reward.approved_amount ? `${reward.approved_amount} approved` : "Amount pending"}</span>
        <span>{reward.wallet_credit ? `Credited ${reward.wallet_credit.amount}` : "Wallet pending"}</span>
      </div>
      <div className={styles.detailList}>
        <span>Updated {formatDate(reward.updated_at)}</span>
        {reward.wallet_credit ? <span>Credited {formatDate(reward.wallet_credit.credited_at)}</span> : null}
        {reward.token_transaction?.transaction_hash ? (
          <span>Token tx {reward.token_transaction.transaction_hash.slice(0, 12)}</span>
        ) : null}
      </div>
    </article>
  );
}

function RewardCard({ reward }: { reward: RewardHistoryEntry }) {
  return (
    <article className={styles.itemCard}>
      <div className={styles.itemHeader}>
        <h3>{reward.course_title}</h3>
        <StatusPill label={humanRewardStatus(reward.status)} tone={rewardTone(reward.status)} />
      </div>
      <p className={styles.muted}>{rewardNextStep(reward)}</p>
      <div className={styles.metaRow}>
        <span>{humanize(reward.event_type)}</span>
        <span>{reward.approved_amount ? `${reward.approved_amount} approved` : "Amount pending"}</span>
        <span>{reward.wallet_credit ? "Wallet credited" : "Wallet pending"}</span>
      </div>
      <div className={styles.detailList}>
        <span>Updated {formatDate(reward.updated_at)}</span>
        {reward.wallet_credit ? <span>Credited {reward.wallet_credit.amount}</span> : null}
        {reward.token_transaction?.transaction_hash ? (
          <span>Token tx {reward.token_transaction.transaction_hash.slice(0, 12)}</span>
        ) : null}
      </div>
    </article>
  );
}

function SignedOutState({ redirect }: { redirect: string }) {
  return (
    <section className={styles.statePanel}>
      <div className={styles.panelHeader}>
        <LogIn size={20} aria-hidden />
        <h2>Sign in required</h2>
      </div>
      <p className={styles.muted}>Learner routes load after RustLearn resolves your current session.</p>
      <Link className={styles.primaryLink} href={loginHref(redirect)}>
        <LogIn size={18} aria-hidden />
        Sign in
      </Link>
    </section>
  );
}

function LoadingState() {
  return (
    <section className={styles.statePanel} aria-live="polite">
      <div className={styles.panelHeader}>
        <Loader2 className={styles.spin} size={20} aria-hidden />
        <h2>Loading learner route</h2>
      </div>
      <div className={styles.skeletonGrid} aria-hidden>
        <div className={styles.skeleton} />
        <div className={styles.skeleton} />
        <div className={styles.skeleton} />
      </div>
    </section>
  );
}

function ErrorState({
  error,
  onRetry,
  redirect,
}: {
  error: RouteError;
  onRetry: () => void;
  redirect: string;
}) {
  const needsSignIn = error.code === "unauthorized" || error.code === "missing_token" || error.code === "missing_user";
  const needsVerification = error.code === "unverified_email";

  return (
    <section className={styles.errorPanel} role="status">
      <div className={styles.panelHeader}>
        <AlertTriangle size={20} aria-hidden />
        <h2>{error.code}</h2>
      </div>
      <p>{error.message}</p>
      {needsVerification ? (
        <Link className={styles.secondaryLink} href="/verify-email">
          Verify email
        </Link>
      ) : needsSignIn ? (
        <Link className={styles.secondaryLink} href={loginHref(redirect)}>
          Return to login
        </Link>
      ) : (
        <button className={styles.secondaryButton} type="button" onClick={onRetry}>
          <RefreshCw size={18} aria-hidden />
          Retry
        </button>
      )}
    </section>
  );
}

function EmptyState({
  action,
  actionHref,
  actionLabel,
  detail,
  title,
}: {
  action?: ReactNode;
  actionHref?: string;
  actionLabel?: string;
  detail: string;
  title: string;
}) {
  return (
    <section className={styles.statePanel}>
      <h3>{title}</h3>
      <p className={styles.muted}>{detail}</p>
      {action}
      {actionHref && actionLabel ? (
        <Link className={styles.secondaryLink} href={actionHref}>
          {actionLabel}
        </Link>
      ) : null}
    </section>
  );
}

function SummaryCard({
  icon,
  label,
  value,
}: {
  icon: ReactNode;
  label: string;
  value: number | string;
}) {
  return (
    <article className={styles.summaryCard}>
      {icon}
      <strong>{value}</strong>
      <span className={styles.muted}>{label}</span>
    </article>
  );
}

function StatusPill({
  label,
  tone,
}: {
  label: string;
  tone: "bad" | "good" | "neutral" | "warn";
}) {
  return <span className={`${styles.statusPill} ${styles[tone]}`}>{label}</span>;
}

function summarizeRewards(rewards: RewardHistoryEntry[]) {
  return rewards.reduce(
    (summary, reward) => {
      if (reward.status === "pending_teacher_approval") {
        summary.pendingTeacher += 1;
      }
      if (reward.status.includes("token") || reward.status === "amount_approved") {
        summary.processing += 1;
      }
      if (reward.wallet_credit) {
        summary.credited += 1;
      }
      if (reward.status === "failed" || reward.status === "needs_reconciliation") {
        summary.needsHelp += 1;
      }
      return summary;
    },
    {
      credited: 0,
      needsHelp: 0,
      pendingTeacher: 0,
      processing: 0,
    },
  );
}

function isWalletCreditPending(reward: RewardHistoryEntry) {
  return !reward.wallet_credit && reward.status !== "failed" && reward.status !== "needs_reconciliation" && !reward.status.endsWith("_rejected");
}

function courseOrganizationLabel(course: CourseCatalogItem) {
  return course.organizations.map((organization) => organization.name).join(", ") || "Independent";
}

function courseTeacherLabel(course: CourseCatalogItem) {
  return course.teachers.map((teacher) => teacher.name).join(", ") || "Teacher pending";
}

function courseContentLabel(course: CourseCatalogItem) {
  return course.content.has_content
    ? `${course.content.chapter_count} chapters, ${course.content.content_count} items`
    : "Content pending";
}

function learnerNotice(error: RouteError | null, kind: LearnerRouteKind): ShellNotice | null {
  if (!error) {
    return null;
  }

  if (error.code === "unauthorized" || error.code === "missing_user" || error.code === "missing_token") {
    return {
      actionHref: loginHref(routeConfig[kind].href),
      actionLabel: "Sign in",
      message: "Your stored session is no longer valid. Sign in again to continue.",
      title: "Session expired",
      tone: "error",
    };
  }

  if (error.code === "unverified_email") {
    return {
      actionHref: "/verify-email",
      actionLabel: "Verify email",
      message: "Verify this email address before using learner routes.",
      title: "Email verification required",
      tone: "warn",
    };
  }

  return {
    message: error.message,
    title: "Learner route status",
    tone: error.code === "timeout" || error.code === "network_error" ? "warn" : "error",
  };
}

function normalizeRouteError(error: unknown) {
  if (error instanceof SessionRequestError) {
    return error;
  }

  if (error instanceof LearnerRequestError) {
    return error;
  }

  return new LearnerRequestError("Learner route failed before the API responded.", 0, "network_error");
}

function rewardTone(status: string): "bad" | "good" | "neutral" | "warn" {
  if (status === "wallet_credited" || status === "completed" || status === "notified") {
    return "good";
  }
  if (status === "failed") {
    return "bad";
  }
  if (status === "needs_reconciliation" || status.endsWith("_rejected")) {
    return "warn";
  }
  return "neutral";
}

function enrollmentTone(status: string): "bad" | "good" | "neutral" | "warn" {
  if (status === "enrolled" || status === "available") {
    return "good";
  }
  if (status === "pending" || status === "waitlisted") {
    return "warn";
  }
  if (status === "rejected" || status === "unavailable") {
    return "bad";
  }
  return "neutral";
}

function lifecycleTone(status: string): "bad" | "good" | "neutral" | "warn" {
  if (status === "published") return "good";
  if (status === "suspended") return "bad";
  if (status === "archived") return "warn";
  if (status === "submitted" || status === "needs_changes") return "warn";
  return "neutral";
}

function contentStateTone(status: string): "bad" | "good" | "neutral" | "warn" {
  if (status === "ready") {
    return "good";
  }
  if (status === "processing" || status === "uploaded" || status === "unprocessed_upload") {
    return "warn";
  }
  if (status === "failed_processing" || status === "unavailable") {
    return "bad";
  }
  return "neutral";
}

function findLearningContent(learning: CourseLearningResponse | null, contentId: number | null) {
  if (!learning || contentId === null) {
    return null;
  }

  return learning.chapters
    .flatMap((chapter) => chapter.contents)
    .find((content) => content.id === contentId) || null;
}

function isReadableTextContent(contentType: string) {
  const normalized = contentType.trim().toLowerCase();
  return normalized === "text" || normalized === "article" || normalized === "markdown";
}

function lessonStateCopy(content: CourseLearningContent) {
  switch (content.display_state) {
    case "processing":
      return {
        detail: "This media is still being processed. Check back after the worker finishes.",
        title: "Processing media",
      };
    case "failed_processing":
      return {
        detail: "The media processor failed. Course staff need to retry or replace this upload.",
        title: "Processing failed",
      };
    case "unprocessed_upload":
      return {
        detail: "The upload has not been attached or queued for processing yet.",
        title: "Upload not ready",
      };
    case "uploaded":
      return {
        detail: "The file is uploaded, but this viewer does not yet stream the stored object.",
        title: "Uploaded file",
      };
    case "unavailable":
      return {
        detail: "This lesson has no readable content data yet.",
        title: "Content unavailable",
      };
    default:
      return {
        detail: "This content type is not rendered inline yet.",
        title: "Content preview",
      };
  }
}

function plural(count: number) {
  return count === 1 ? "policy" : "policies";
}

function rewardNextStep(reward: RewardHistoryEntry) {
  switch (reward.status) {
    case "pending_teacher_approval":
      return "Waiting for teacher review.";
    case "teacher_approved":
      return "Teacher approved this activity; amount review is next.";
    case "teacher_rejected":
      return "Teacher rejected this activity. Check the course context before resubmitting.";
    case "amount_approved":
    case "adjusted":
      return "Amount is approved and token processing can continue.";
    case "amount_rejected":
      return "Amount review rejected this reward.";
    case "token_pending":
      return "Token transaction is being processed.";
    case "token_confirmed":
      return "Token transaction is confirmed; wallet credit is next.";
    case "wallet_credited":
    case "notified":
    case "completed":
      return "Reward was credited to the wallet.";
    case "needs_reconciliation":
      return "Reward needs reconciliation before it is final.";
    case "failed":
      return "Reward failed and needs support review.";
    default:
      return "Reward status changed after this page loaded.";
  }
}

function humanRewardStatus(status: string) {
  return sentenceCase(humanize(status));
}

function humanize(value: string) {
  return value.replaceAll("_", " ");
}

function sentenceCase(value: string) {
  return value ? `${value[0].toUpperCase()}${value.slice(1)}` : value;
}

function formatDate(value: string) {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return value;
  }

  return new Intl.DateTimeFormat(undefined, {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(date);
}

function loginHref(redirect: string) {
  return `/login?redirect=${encodeURIComponent(redirect)}`;
}

const rewardFilterOptions: Array<{ label: string; value: RewardStatusFilter }> = [
  { label: "All", value: "all" },
  { label: "Pending", value: "pending_teacher_approval" },
  { label: "Approved", value: "amount_approved" },
  { label: "Processing", value: "token_pending" },
  { label: "Confirmed", value: "token_confirmed" },
  { label: "Credited", value: "wallet_credited" },
  { label: "Needs help", value: "needs_reconciliation" },
  { label: "Failed", value: "failed" },
];

const enrollmentFilterOptions: Array<{ label: string; value: EnrollmentStatusFilter }> = [
  { label: "All", value: "all" },
  { label: "Available", value: "available" },
  { label: "Pending", value: "pending" },
  { label: "Waitlisted", value: "waitlisted" },
  { label: "Enrolled", value: "enrolled" },
  { label: "Rejected", value: "rejected" },
  { label: "Unavailable", value: "unavailable" },
];

const routeConfig = {
  dashboard: {
    description: "Continue learning, inspect rewards, and keep your wallet ready.",
    href: "/learn",
    title: "Learner dashboard",
  },
  courses: {
    description: "Search courses, review enrollment state, and request access.",
    href: "/courses",
    title: "Courses",
  },
  rewards: {
    description: "Reward history with human status, wallet credit, and course context.",
    href: "/rewards",
    title: "Rewards",
  },
  wallet: {
    description: "Wallet link state, balance, and reward-credit readiness.",
    href: "/wallet",
    title: "Wallet",
  },
} as const;
