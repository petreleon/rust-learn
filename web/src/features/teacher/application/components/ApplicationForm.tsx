"use client";

import { AlertCircle, Loader2, RotateCcw, Send } from "lucide-react";
import { type FormEvent } from "react";
import { type CurrentSession } from "@/lib/session";
import styles from "@/app/teach/apply/page.module.css";
import { clearDraft } from "../model/clearDraft";
import { defaultDraft } from "../model/defaultDraft";
import { type ApplicationDraft } from "../model/ApplicationDraft";
import { type SubmitState } from "../model/SubmitState";
import { ScopeOption } from "./ScopeOption";

export function ApplicationForm({
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
