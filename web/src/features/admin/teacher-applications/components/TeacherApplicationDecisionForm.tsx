"use client";

import { AlertTriangle, Loader2, Send } from "lucide-react";
import { type PlatformTeacherApplicationItem } from "@/lib/admin/PlatformTeacherApplicationItem";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { StatusPill } from "@/components/admin-routes/StatusPill";
import { type RouteError } from "@/shared/route-state/RouteError";
import {
  canSubmitTeacherApplicationDecision,
  type TeacherApplicationDecisionDraft,
  type TeacherApplicationDecisionStatus,
} from "../model/TeacherApplicationDecisionDraft";
import { type TeacherApplicationDecisionState } from "../model/TeacherApplicationDecisionState";
import { isFinalTeacherApplicationStatus } from "../model/teacherApplicationDisplay";

export function TeacherApplicationDecisionForm({
  application,
  canApprove,
  canReject,
  draft,
  error,
  onDraftChange,
  onSubmit,
  state,
}: {
  application: PlatformTeacherApplicationItem;
  canApprove: boolean;
  canReject: boolean;
  draft: TeacherApplicationDecisionDraft;
  error: RouteError | null;
  onDraftChange: (patch: Partial<TeacherApplicationDecisionDraft>) => void;
  onSubmit: () => void;
  state: TeacherApplicationDecisionState;
}) {
  const isFinal = isFinalTeacherApplicationStatus(application.status);

  return (
    <form
      className={styles.decisionForm}
      onSubmit={(event) => {
        event.preventDefault();
        onSubmit();
      }}
    >
      <div className={styles.subsectionHeader}>
        <h3>Decision</h3>
        {isFinal ? <StatusPill label="Final state" tone="neutral" /> : null}
      </div>
      <label>
        <span>Decision status</span>
        <select
          disabled={isFinal}
          onChange={(event) => onDraftChange({ status: event.target.value as TeacherApplicationDecisionStatus })}
          value={draft.status}
        >
          <option value="needs_changes">Needs changes</option>
          <option disabled={!canApprove} value="approved">
            Approve
          </option>
          <option disabled={!canReject} value="rejected">
            Reject
          </option>
        </select>
      </label>
      {!canApprove || !canReject ? (
        <p className={styles.muted}>
          Approve and reject require the matching platform permissions; requesting changes remains available to reviewers.
        </p>
      ) : null}
      <label>
        <span>Decision reason</span>
        <textarea
          disabled={isFinal}
          onChange={(event) => onDraftChange({ reason: event.target.value })}
          placeholder="Explain what changed, what is missing, or why the application is approved."
          rows={4}
          value={draft.reason}
        />
      </label>
      {!isFinal && !draft.reason.trim() ? (
        <p className={styles.muted}>A decision reason is required before saving.</p>
      ) : null}
      {error ? (
        <div className={styles.inlineError} role="alert">
          <AlertTriangle size={16} aria-hidden />
          <span>{error.message}</span>
        </div>
      ) : null}
      <button
        className={styles.primaryButton}
        disabled={!canSubmitTeacherApplicationDecision({ application, canApprove, canReject, draft, state })}
        type="submit"
      >
        {state === "submitting" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <Send size={16} aria-hidden />}
        Save decision
      </button>
      {isFinal ? (
        <p className={styles.muted}>Approved and rejected applications are final in the current backend contract.</p>
      ) : null}
    </form>
  );
}
