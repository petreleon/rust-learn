"use client";

import {
  AlertCircle,
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
  fetchMyTeacherApplication,
  fetchTeachingCourses,
  TeacherRequestError,
  type TeacherApplication,
  type TeacherApplicationSnapshot,
  type TeacherCourseDashboardItem,
  type TeacherCoursesResponse,
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
        <PermissionChip enabled={course.permissions.can_approve_reward_candidates} label="Rewards" />
        <PermissionChip enabled={course.permissions.can_manage_settings} label="Settings" />
      </div>

      <div className={styles.actionRow}>
        <button className={styles.secondaryButton} disabled type="button" title="Course workspace route is next">
          <BriefcaseBusiness size={16} aria-hidden />
          Workspace
        </button>
        <button className={styles.secondaryButton} disabled type="button" title="Enrollment queue route is next">
          <Users size={16} aria-hidden />
          Enrollments
        </button>
        <button className={styles.secondaryButton} disabled type="button" title="Reward review route is next">
          <Trophy size={16} aria-hidden />
          Rewards
        </button>
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

function statusLabel(value: string) {
  return value.replace(/_/g, " ").replace(/\b\w/g, (letter) => letter.toUpperCase());
}
