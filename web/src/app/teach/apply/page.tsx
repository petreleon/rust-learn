"use client";

import {
  AlertCircle,
  ArrowRight,
  BriefcaseBusiness,
  CheckCircle2,
  Clock3,
  ExternalLink,
  FileText,
  Loader2,
  LogIn,
  RefreshCw,
  RotateCcw,
  Send,
  ShieldCheck,
  XCircle,
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
  submitTeacherApplication,
  TeacherRequestError,
  type TeacherApplication,
  type TeacherApplicationAuditEvent,
  type TeacherApplicationScope,
  type TeacherApplicationSnapshot,
} from "@/lib/teacher";
import styles from "./page.module.css";

type LoadState = "idle" | "loading" | "success" | "error";
type SubmitState = "idle" | "submitting" | "success" | "error";

type RouteError = {
  code: string;
  message: string;
  status: number;
};

type ApplicationDraft = {
  experienceSummary: string;
  idempotencyKey: string;
  portfolioLinks: string;
  requestedCourseId: string;
  requestedOrganizationId: string;
  requestedScope: TeacherApplicationScope;
};

type ValidationResult = {
  errors: string[];
  portfolioLinks: string[];
};

const DRAFT_STORAGE_KEY = "rustlearn.teacher_application.draft.v1";
const submitPermission = "SUBMIT_TEACHER_APPLICATION";

const emptySnapshot: TeacherApplicationSnapshot = {
  application: null,
  audit_events: [],
};

export default function TeacherApplicationPage() {
  const [draft, setDraft] = useState<ApplicationDraft>(defaultDraft);
  const [draftReady, setDraftReady] = useState(false);
  const [error, setError] = useState<RouteError | null>(null);
  const [hasToken, setHasToken] = useState(false);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [showRejectedForm, setShowRejectedForm] = useState(false);
  const [snapshot, setSnapshot] = useState<TeacherApplicationSnapshot>(emptySnapshot);
  const [submitError, setSubmitError] = useState<RouteError | null>(null);
  const [submitState, setSubmitState] = useState<SubmitState>("idle");
  const [validationErrors, setValidationErrors] = useState<string[]>([]);

  const loadApplication = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setSnapshot(emptySnapshot);
      setError(null);
      setLoadState("idle");
      return;
    }

    setHasToken(true);
    setLoadState("loading");
    setError(null);
    setSubmitError(null);

    try {
      const nextSession = await fetchCurrentSession({ token });
      const nextSnapshot = await fetchMyTeacherApplication({ token });
      setSession(nextSession);
      setSnapshot(nextSnapshot);
      setLoadState("success");
      if (nextSnapshot.application?.status !== "rejected") {
        setShowRejectedForm(false);
      }
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      if (routeError.status === 401 || routeError.status === 404) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setSession(null);
      setSnapshot(emptySnapshot);
      setError(routeError);
      setLoadState("error");
    }
  }, []);

  useEffect(() => {
    const timeout = window.setTimeout(() => {
      setDraft(readDraft());
      setDraftReady(true);
    }, 0);
    return () => window.clearTimeout(timeout);
  }, []);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadApplication(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadApplication]);

  useEffect(() => {
    if (!draftReady) {
      return;
    }

    window.sessionStorage.setItem(DRAFT_STORAGE_KEY, JSON.stringify(draft));
  }, [draft, draftReady]);

  const application = snapshot.application;
  const canSubmitApplication = Boolean(session?.platform.effective_permissions.includes(submitPermission));
  const formVisible = Boolean(!application || (application.status === "rejected" && showRejectedForm));
  const workspaceSummary = useMemo(() => summarizeWorkspace(session), [session]);
  const selectedOrganization = session?.organizations.find(
    (organization) => String(organization.id) === draft.requestedOrganizationId,
  );
  const selectedCourse = session?.courses.find((course) => String(course.id) === draft.requestedCourseId);
  const statusConfig = applicationStatusConfig(application);
  const notice = routeNotice(error, submitError);

  useEffect(() => {
    const handleBeforeUnload = (e: BeforeUnloadEvent) => {
      const isDirty =
        draft.experienceSummary.trim() !== "" ||
        draft.portfolioLinks.trim() !== "" ||
        draft.requestedCourseId !== "" ||
        draft.requestedOrganizationId !== "" ||
        draft.requestedScope !== "platform";

      if (isDirty && formVisible && submitState !== "success") {
        e.preventDefault();
        e.returnValue = "";
      }
    };
    window.addEventListener("beforeunload", handleBeforeUnload);
    return () => window.removeEventListener("beforeunload", handleBeforeUnload);
  }, [draft, formVisible, submitState]);

  function signOut() {
    const isDirty =
      draft.experienceSummary.trim() !== "" ||
      draft.portfolioLinks.trim() !== "" ||
      draft.requestedCourseId !== "" ||
      draft.requestedOrganizationId !== "" ||
      draft.requestedScope !== "platform";

    if (isDirty && formVisible && submitState !== "success") {
      if (!window.confirm("You have unsaved changes in your teacher application. Are you sure you want to sign out?")) {
        return;
      }
    }

    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setSnapshot(emptySnapshot);
    setError(null);
    setSubmitError(null);
    setLoadState("idle");
  }

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const token = readStoredSessionToken();
    if (!token) {
      setSubmitError({
        code: "missing_token",
        message: "Sign in again before submitting your teacher application.",
        status: 401,
      });
      return;
    }

    const validation = validateDraft(draft, session);
    setValidationErrors(validation.errors);
    if (validation.errors.length) {
      return;
    }

    setSubmitState("submitting");
    setSubmitError(null);

    try {
      await submitTeacherApplication({
        payload: {
          experience_summary: draft.experienceSummary.trim(),
          idempotency_key: draft.idempotencyKey,
          portfolio_links: validation.portfolioLinks,
          requested_course_id:
            draft.requestedScope === "course" ? Number(draft.requestedCourseId) : undefined,
          requested_organization_id:
            draft.requestedScope === "organization" ? Number(draft.requestedOrganizationId) : undefined,
          requested_scope: draft.requestedScope,
        },
        token,
      });
      const nextSnapshot = await fetchMyTeacherApplication({ token });
      setSnapshot(nextSnapshot);
      setSubmitState("success");
      setShowRejectedForm(false);
      clearDraft();
      setDraft(defaultDraft());
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      setSubmitError(routeError);
      setSubmitState("error");
      if (routeError.code === "conflict") {
        try {
          setSnapshot(await fetchMyTeacherApplication({ token }));
        } catch {
          // Keep the visible conflict message; retry remains available.
        }
      }
    }
  }

  return (
    <ProductShell
      activeNav="teach"
      breadcrumbs={[
        { href: "/session", label: "Workspace" },
        { href: "/teach", label: "Teach" },
        { label: "Apply" },
      ]}
      description="Apply for teaching access, track review status, and see reviewer feedback without using the operations console."
      eyebrow="Teacher"
      isSignedIn={hasToken || Boolean(session)}
      notice={notice}
      onSignOut={signOut}
      session={session}
      statusItems={
        <>
          <StatusLine
            icon={<ShieldCheck size={16} aria-hidden />}
            label={
              canSubmitApplication
                ? "Application access"
                : application
                  ? "Application visible"
                  : loadState === "loading"
                    ? "Resolving access"
                    : "Access needed"
            }
            tone={canSubmitApplication ? "good" : application || loadState === "loading" ? "neutral" : "warn"}
          />
          <StatusLine icon={statusConfig.icon} label={statusConfig.label} tone={statusConfig.tone} />
        </>
      }
      title="Teacher application"
    >
      {loadState === "loading" ? (
        <StatePanel
          icon={<Loader2 className={styles.spin} size={22} aria-hidden />}
          title="Loading application"
          detail="Resolving your current session, teacher application state, and review history."
        />
      ) : null}

      {loadState === "idle" ? (
        <StatePanel
          action={
            <Link className={styles.primaryLink} href="/login?redirect=/teach/apply">
              <LogIn size={18} aria-hidden />
              Sign in
            </Link>
          }
          detail="Teacher applications use your current RustLearn account and saved session."
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
          <button className={styles.secondaryButton} type="button" onClick={() => void loadApplication()}>
            <RefreshCw size={17} aria-hidden />
            Retry
          </button>
        </section>
      ) : null}

      {loadState === "success" && session ? (
        <>
          <section className={styles.overviewGrid}>
            <article className={styles.statusPanel}>
              <div className={styles.panelHeader}>
                {statusConfig.icon}
                <h2>{statusConfig.heading}</h2>
              </div>
              <p className={styles.muted}>{statusConfig.detail}</p>
              {application ? <ApplicationSummary application={application} /> : null}
              {application?.status === "rejected" && canSubmitApplication && !showRejectedForm ? (
                <button className={styles.primaryButton} type="button" onClick={() => setShowRejectedForm(true)}>
                  <RotateCcw size={17} aria-hidden />
                  Start a new application
                </button>
              ) : null}
              {application?.status === "approved" ? (
                <Link className={styles.primaryLink} href="/teach">
                  <ArrowRight size={18} aria-hidden />
                  Open teaching workspace
                </Link>
              ) : null}
            </article>

            <article className={styles.statusPanel}>
              <div className={styles.panelHeader}>
                <BriefcaseBusiness size={22} aria-hidden />
                <h2>Scope context</h2>
              </div>
              <p className={styles.muted}>
                {workspaceSummary}. Organization and course choices use your session context, so you do not need internal ids.
              </p>
              <div className={styles.contextList}>
                <ContextRow label="Organizations" value={String(session.organizations.length)} />
                <ContextRow label="Courses" value={String(session.courses.length)} />
                <ContextRow label="Delegations" value={String(session.delegated_permissions.length)} />
              </div>
            </article>
          </section>

          {!canSubmitApplication && formVisible ? (
            <section className={`${styles.warningPanel} ${styles.singlePanel}`}>
              <div className={styles.panelHeader}>
                <AlertCircle size={20} aria-hidden />
                <h2>Application permission needed</h2>
              </div>
              <p className={styles.muted}>
                Your current session can view this page, but it does not include teacher application submission access.
                Ask a platform operator to enable application access or nominate you through an organization.
              </p>
            </section>
          ) : null}

          {formVisible && canSubmitApplication ? (
            <ApplicationForm
              draft={draft}
              onDraftChange={setDraft}
              onSubmit={handleSubmit}
              selectedCourseLabel={selectedCourse?.title || null}
              selectedOrganizationLabel={selectedOrganization?.name || null}
              session={session}
              submitState={submitState}
              validationErrors={validationErrors}
            />
          ) : null}

          {!formVisible && application?.status === "needs_changes" ? (
            <section className={`${styles.warningPanel} ${styles.singlePanel}`}>
              <div className={styles.panelHeader}>
                <FileText size={20} aria-hidden />
                <h2>Changes requested</h2>
              </div>
              <p className={styles.muted}>
                Reviewer feedback is visible below. Editing and resubmission are intentionally blocked until the
                application update contract exists, so the page does not pretend a new submission will replace this one.
              </p>
            </section>
          ) : null}

          {application ? (
            <AuditTimeline auditEvents={snapshot.audit_events} decisionReason={application.decision_reason} />
          ) : null}
        </>
      ) : null}
    </ProductShell>
  );
}

function ApplicationForm({
  draft,
  onDraftChange,
  onSubmit,
  selectedCourseLabel,
  selectedOrganizationLabel,
  session,
  submitState,
  validationErrors,
}: {
  draft: ApplicationDraft;
  onDraftChange: (nextDraft: ApplicationDraft) => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
  selectedCourseLabel: string | null;
  selectedOrganizationLabel: string | null;
  session: CurrentSession;
  submitState: SubmitState;
  validationErrors: string[];
}) {
  const hasOrganizations = session.organizations.length > 0;
  const hasCourses = session.courses.length > 0;

  function updateDraft(patch: Partial<ApplicationDraft>) {
    onDraftChange({
      ...draft,
      ...patch,
    });
  }

  return (
    <form className={styles.formPanel} onSubmit={onSubmit}>
      <div className={styles.panelHeader}>
        <Send size={22} aria-hidden />
        <h2>Application details</h2>
      </div>

      {validationErrors.length ? (
        <div className={styles.validationBox} role="status">
          <AlertCircle size={18} aria-hidden />
          <span>
            <strong>Check the application</strong>
            {validationErrors.join(" ")}
          </span>
        </div>
      ) : null}

      <fieldset className={styles.scopeFieldset}>
        <legend>Requested teaching scope</legend>
        <div className={styles.scopeGrid}>
          <ScopeOption
            checked={draft.requestedScope === "platform"}
            detail="Teach across the platform after central review."
            label="Platform"
            name="requested-scope"
            onChange={() => updateDraft({ requestedScope: "platform" })}
            value="platform"
          />
          <ScopeOption
            checked={draft.requestedScope === "organization"}
            detail={hasOrganizations ? "Use an organization from your session context." : "Requires organization context or nomination."}
            disabled={!hasOrganizations}
            label="Organization"
            name="requested-scope"
            onChange={() =>
              updateDraft({
                requestedOrganizationId: draft.requestedOrganizationId || String(session.organizations[0]?.id || ""),
                requestedScope: "organization",
              })
            }
            value="organization"
          />
          <ScopeOption
            checked={draft.requestedScope === "course"}
            detail={hasCourses ? "Request teaching access for a course visible to your session." : "Requires a visible course context."}
            disabled={!hasCourses}
            label="Course"
            name="requested-scope"
            onChange={() =>
              updateDraft({
                requestedCourseId: draft.requestedCourseId || String(session.courses[0]?.id || ""),
                requestedScope: "course",
              })
            }
            value="course"
          />
        </div>
      </fieldset>

      {draft.requestedScope === "organization" ? (
        <label className={styles.fieldLabel}>
          Organization
          <select
            value={draft.requestedOrganizationId}
            onChange={(event) => updateDraft({ requestedOrganizationId: event.target.value })}
          >
            <option value="">Choose organization</option>
            {session.organizations.map((organization) => (
              <option key={organization.id} value={organization.id}>
                {organization.name}
              </option>
            ))}
          </select>
          <small>{selectedOrganizationLabel ? `${selectedOrganizationLabel} will be attached to the request.` : "No raw organization id is required."}</small>
        </label>
      ) : null}

      {draft.requestedScope === "course" ? (
        <label className={styles.fieldLabel}>
          Course
          <select value={draft.requestedCourseId} onChange={(event) => updateDraft({ requestedCourseId: event.target.value })}>
            <option value="">Choose course</option>
            {session.courses.map((course) => (
              <option key={course.id} value={course.id}>
                {course.title}
              </option>
            ))}
          </select>
          <small>{selectedCourseLabel ? `${selectedCourseLabel} will be attached to the request.` : "No raw course id is required."}</small>
        </label>
      ) : null}

      <label className={styles.fieldLabel}>
        Experience summary
        <textarea
          rows={6}
          value={draft.experienceSummary}
          onChange={(event) => updateDraft({ experienceSummary: event.target.value })}
          placeholder="Describe what you teach, where you have taught, and how you support learner progress."
        />
      </label>

      <label className={styles.fieldLabel}>
        Portfolio links
        <textarea
          rows={4}
          value={draft.portfolioLinks}
          onChange={(event) => updateDraft({ portfolioLinks: event.target.value })}
          placeholder="One link per line"
        />
        <small>Links are optional and stay in your browser draft until you submit or clear the draft.</small>
      </label>

      <div className={styles.actionRow}>
        <button className={styles.primaryButton} disabled={submitState === "submitting"} type="submit">
          {submitState === "submitting" ? <Loader2 className={styles.spin} size={17} aria-hidden /> : <Send size={17} aria-hidden />}
          Submit application
        </button>
        <button
          className={styles.secondaryButton}
          type="button"
          onClick={() => {
            clearDraft();
            onDraftChange(defaultDraft());
          }}
        >
          <RotateCcw size={17} aria-hidden />
          Clear draft
        </button>
      </div>
    </form>
  );
}

function ScopeOption({
  checked,
  detail,
  disabled = false,
  label,
  name,
  onChange,
  value,
}: {
  checked: boolean;
  detail: string;
  disabled?: boolean;
  label: string;
  name: string;
  onChange: () => void;
  value: TeacherApplicationScope;
}) {
  return (
    <label className={`${styles.scopeOption} ${checked ? styles.scopeOptionActive : ""} ${disabled ? styles.scopeOptionDisabled : ""}`}>
      <input checked={checked} disabled={disabled} name={name} onChange={onChange} type="radio" value={value} />
      <strong>{label}</strong>
      <span>{detail}</span>
    </label>
  );
}

function ApplicationSummary({ application }: { application: TeacherApplication }) {
  const portfolioLinks = normalizePortfolioLinks(application.portfolio_links);

  return (
    <div className={styles.applicationSummary}>
      <ContextRow label="Scope" value={scopeLabel(application.requested_scope)} />
      <ContextRow label="Submitted" value={formatDate(application.created_at)} />
      <ContextRow label="Updated" value={formatDate(application.updated_at)} />
      {application.decision_reason ? <ContextRow label="Reviewer note" value={application.decision_reason} /> : null}
      {portfolioLinks.length ? (
        <div className={styles.portfolioBlock}>
          <strong>Portfolio</strong>
          <div className={styles.portfolioLinks}>
            {portfolioLinks.map((link) => (
              <PortfolioLink key={link} link={link} />
            ))}
          </div>
        </div>
      ) : null}
    </div>
  );
}

function AuditTimeline({
  auditEvents,
  decisionReason,
}: {
  auditEvents: TeacherApplicationAuditEvent[];
  decisionReason: string | null;
}) {
  return (
    <section className={styles.timelinePanel}>
      <div className={styles.panelHeader}>
        <Clock3 size={20} aria-hidden />
        <h2>Review history</h2>
      </div>
      {decisionReason ? <p className={styles.reviewReason}>{decisionReason}</p> : null}
      {auditEvents.length ? (
        <ol className={styles.timelineList}>
          {auditEvents.map((event) => (
            <li key={event.id}>
              <span className={styles.timelineDot} aria-hidden />
              <div>
                <strong>{statusLabel(event.to_status)}</strong>
                <small>{formatDate(event.created_at)}</small>
                {event.reason ? <p>{event.reason}</p> : null}
              </div>
            </li>
          ))}
        </ol>
      ) : (
        <p className={styles.muted}>Review events will appear after the application is submitted.</p>
      )}
    </section>
  );
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
    <section className={`${styles.statusPanel} ${styles.singlePanel}`} aria-live="polite">
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
  tone: "good" | "neutral" | "warn" | "bad";
}) {
  return (
    <span className={`${styles.statusLine} ${styles[tone]}`}>
      {icon}
      {label}
    </span>
  );
}

function ContextRow({ label, value }: { label: string; value: string }) {
  return (
    <div className={styles.contextRow}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function PortfolioLink({ link }: { link: string }) {
  if (!isSafeHttpUrl(link)) {
    return <span className={styles.portfolioText}>{link}</span>;
  }

  return (
    <a className={styles.portfolioLink} href={link} rel="noreferrer" target="_blank">
      <ExternalLink size={15} aria-hidden />
      {link}
    </a>
  );
}

function applicationStatusConfig(application: TeacherApplication | null): {
  detail: string;
  heading: string;
  icon: ReactNode;
  label: string;
  tone: "good" | "neutral" | "warn" | "bad";
} {
  if (!application) {
    return {
      detail: "No teacher application is on file yet. Submit one when you are ready for platform review.",
      heading: "Ready to apply",
      icon: <FileText size={20} aria-hidden />,
      label: "No application",
      tone: "neutral",
    };
  }

  switch (application.status) {
    case "approved":
      return {
        detail: "Your application has been approved. Teaching permissions are assigned from the approved scope.",
        heading: "Application approved",
        icon: <CheckCircle2 size={20} aria-hidden />,
        label: "Approved",
        tone: "good",
      };
    case "needs_changes":
      return {
        detail: "A reviewer asked for changes. This page shows the reason and blocks fake resubmission until the edit contract exists.",
        heading: "Changes requested",
        icon: <AlertCircle size={20} aria-hidden />,
        label: "Needs changes",
        tone: "warn",
      };
    case "rejected":
      return {
        detail: "The latest application was rejected. You can start a fresh application with a stronger summary or portfolio.",
        heading: "Application rejected",
        icon: <XCircle size={20} aria-hidden />,
        label: "Rejected",
        tone: "bad",
      };
    case "submitted":
    default:
      return {
        detail: "Your application is in the review queue. Duplicate submissions are blocked so reviewer state stays clear.",
        heading: "Application submitted",
        icon: <Clock3 size={20} aria-hidden />,
        label: "Submitted",
        tone: "neutral",
      };
  }
}

function validateDraft(draft: ApplicationDraft, session: CurrentSession | null): ValidationResult {
  const errors: string[] = [];
  const portfolioLinks = parsePortfolioLinks(draft.portfolioLinks);

  if (!draft.experienceSummary.trim()) {
    errors.push("Add an experience summary.");
  }

  if (draft.requestedScope === "organization") {
    const hasOrganization = session?.organizations.some(
      (organization) => String(organization.id) === draft.requestedOrganizationId,
    );
    if (!hasOrganization) {
      errors.push("Choose an organization from your session context.");
    }
  }

  if (draft.requestedScope === "course") {
    const hasCourse = session?.courses.some((course) => String(course.id) === draft.requestedCourseId);
    if (!hasCourse) {
      errors.push("Choose a course from your session context.");
    }
  }

  return { errors, portfolioLinks };
}

function routeNotice(error: RouteError | null, submitError: RouteError | null): ShellNotice | null {
  if (submitError) {
    return {
      message:
        submitError.code === "conflict"
          ? "Another application state now exists. Review the refreshed status before submitting again."
          : submitError.message,
      title: submitError.code === "conflict" ? "Application state changed" : "Submission failed",
      tone: submitError.code === "conflict" ? "warn" : "error",
    };
  }

  if (!error) {
    return null;
  }

  return {
    actionHref: error.status === 401 ? "/login?redirect=/teach/apply" : undefined,
    actionLabel: error.status === 401 ? "Sign in" : undefined,
    message: error.message,
    title: "Teacher application unavailable",
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
    message: "The request failed before RustLearn could finish loading the application state.",
    status: 0,
  };
}

function summarizeWorkspace(session: CurrentSession | null) {
  if (!session) {
    return "No workspace is loaded yet";
  }

  const organizationCount = session.organizations.length;
  const courseCount = session.courses.length;
  return `${organizationCount} organization${organizationCount === 1 ? "" : "s"} and ${courseCount} course${courseCount === 1 ? "" : "s"} are visible`;
}

function statusLabel(status: string) {
  return status.replace(/_/g, " ").replace(/\b\w/g, (letter) => letter.toUpperCase());
}

function scopeLabel(scope: TeacherApplicationScope) {
  if (scope === "platform") {
    return "Platform";
  }

  if (scope === "organization") {
    return "Organization";
  }

  return "Course";
}

function formatDate(value: string) {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return value;
  }

  return new Intl.DateTimeFormat(undefined, {
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
    month: "short",
    year: "numeric",
  }).format(date);
}

function parsePortfolioLinks(value: string) {
  return value
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean);
}

function normalizePortfolioLinks(value: unknown) {
  if (!Array.isArray(value)) {
    return [];
  }

  return value.filter((item): item is string => typeof item === "string" && item.trim().length > 0);
}

function isSafeHttpUrl(value: string) {
  try {
    const url = new URL(value);
    return url.protocol === "http:" || url.protocol === "https:";
  } catch {
    return false;
  }
}

function readDraft(): ApplicationDraft {
  if (typeof window === "undefined") {
    return defaultDraft();
  }

  const raw = window.sessionStorage.getItem(DRAFT_STORAGE_KEY);
  if (!raw) {
    return defaultDraft();
  }

  try {
    const parsed = JSON.parse(raw) as Partial<ApplicationDraft>;
    return {
      experienceSummary: parsed.experienceSummary || "",
      idempotencyKey: parsed.idempotencyKey || newIdempotencyKey(),
      portfolioLinks: parsed.portfolioLinks || "",
      requestedCourseId: parsed.requestedCourseId || "",
      requestedOrganizationId: parsed.requestedOrganizationId || "",
      requestedScope: isTeacherScope(parsed.requestedScope) ? parsed.requestedScope : "platform",
    };
  } catch {
    return defaultDraft();
  }
}

function clearDraft() {
  if (typeof window !== "undefined") {
    window.sessionStorage.removeItem(DRAFT_STORAGE_KEY);
  }
}

function defaultDraft(): ApplicationDraft {
  return {
    experienceSummary: "",
    idempotencyKey: newIdempotencyKey(),
    portfolioLinks: "",
    requestedCourseId: "",
    requestedOrganizationId: "",
    requestedScope: "platform",
  };
}

function isTeacherScope(value: unknown): value is TeacherApplicationScope {
  return value === "platform" || value === "organization" || value === "course";
}

function newIdempotencyKey() {
  if (typeof crypto !== "undefined" && "randomUUID" in crypto) {
    return `teacher-application-${crypto.randomUUID()}`;
  }

  return `teacher-application-${Date.now()}-${Math.random().toString(36).slice(2)}`;
}
