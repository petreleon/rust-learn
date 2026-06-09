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
  fetchMyWallet,
  fetchRewardHistory,
  LearnerRequestError,
  linkMyWallet,
  requestCourseJoin,
  type CourseCatalogDetail,
  type CourseCatalogItem,
  type CourseCatalogResponse,
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
type LearnerRouteKind = "courses" | "rewards" | "wallet";
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

export function LearnerProductRoute({ kind }: { kind: LearnerRouteKind }) {
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
      const walletPromise = kind === "wallet" ? fetchMyWallet({ token }) : Promise.resolve(null);

      const [nextSession, nextRewards, nextCatalog, nextWallet] = await Promise.all([
        sessionPromise,
        rewardsPromise,
        catalogPromise,
        walletPromise,
      ]);

      setSession(nextSession);
      setRewards(nextRewards);
      setCatalog(nextCatalog);
      setWallet(nextWallet);
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
  joining,
  onRequestJoin,
}: {
  course: CourseCatalogItem;
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

  return (
    <article className={styles.itemCard}>
      <div className={styles.itemHeader}>
        <h3>{course.title}</h3>
        <StatusPill label={humanize(course.enrollment.state)} tone={enrollmentTone(course.enrollment.state)} />
      </div>
      <div className={styles.metaRow}>
        <span>{humanize(course.lifecycle_status)}</span>
        <span>{organizationLabel}</span>
        <span>{teacherLabel}</span>
      </div>
      <div className={styles.detailList}>
        <span>{contentLabel}</span>
        <span>{rewardLabel}</span>
        {course.enrollment.reason ? <span>{course.enrollment.reason}</span> : null}
      </div>
      <div className={styles.actionRow}>
        <Link className={styles.secondaryLink} href={`/courses/${course.id}`}>
          <BookOpen size={18} aria-hidden />
          Details
        </Link>
        {course.enrollment.can_request_join ? (
          <button className={styles.primaryLink} disabled={joining} type="button" onClick={() => onRequestJoin(course)}>
            {joining ? <Loader2 className={styles.spin} size={18} aria-hidden /> : <CheckCircle size={18} aria-hidden />}
            Request join
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
  wallet,
}: {
  linking: boolean;
  onLinkWallet: () => void;
  onRefresh: () => void;
  wallet: WalletSummary | null;
}) {
  return (
    <>
      <section className={styles.contentGrid}>
        <SummaryCard icon={<CreditCard size={20} aria-hidden />} label="Wallet state" value={wallet ? "Linked" : "Unlinked"} />
        <SummaryCard icon={<Trophy size={20} aria-hidden />} label="Available value" value={wallet?.value || "0"} />
        <SummaryCard icon={<BookOpen size={20} aria-hidden />} label="Owner" value={wallet?.owner_type || "User"} />
      </section>

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
              <h3>Wallet #{wallet.id}</h3>
              <StatusPill label={wallet.owner_type} tone="good" />
            </div>
            <p className={styles.muted}>Available value</p>
            <strong className={styles.walletValue}>{wallet.value}</strong>
            <div className={styles.metaRow}>
              <span>{wallet.user_id ? `User ${wallet.user_id}` : "No user owner"}</span>
              <span>{wallet.organization_id ? `Organization ${wallet.organization_id}` : "Personal wallet"}</span>
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
    </>
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
  return humanize(status);
}

function humanize(value: string) {
  return value.replaceAll("_", " ");
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
