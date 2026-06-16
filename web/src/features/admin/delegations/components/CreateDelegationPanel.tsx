"use client";

import { AlertTriangle, Loader2, Send, ShieldCheck } from "lucide-react";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { type DelegationActionState } from "../model/DelegationActionState";
import {
  canSubmitDelegationCreate,
  type DelegationCreateDraft,
} from "../model/DelegationCreateDraft";

export function CreateDelegationPanel({
  draft,
  error,
  onDraftChange,
  onSubmit,
  state,
}: {
  draft: DelegationCreateDraft;
  error: RouteError | null;
  onDraftChange: (patch: Partial<DelegationCreateDraft>) => void;
  onSubmit: () => void;
  state: DelegationActionState;
}) {
  return (
    <form
      className={styles.decisionForm}
      onSubmit={(event) => {
        event.preventDefault();
        onSubmit();
      }}
    >
      <div className={styles.panelHeader}>
        <ShieldCheck size={20} aria-hidden />
        <div>
          <h2>Create delegation</h2>
          <p>Grant a delegated permission with scope and expiration.</p>
        </div>
      </div>
      <label>
        <span>Scope type</span>
        <select onChange={(event) => onDraftChange({ scopeType: event.target.value })} value={draft.scopeType}>
          <option value="">Select scope...</option>
          <option value="platform">Platform</option>
          <option value="organization">Organization</option>
          <option value="course">Course</option>
        </select>
      </label>
      <label>
        <span>Permission</span>
        <input
          onChange={(event) => onDraftChange({ permission: event.target.value })}
          placeholder="e.g. APPROVE_REWARD_AMOUNT"
          type="text"
          value={draft.permission}
        />
      </label>
      <label>
        <span>Grantee user ID</span>
        <input
          onChange={(event) => onDraftChange({ granteeUserId: event.target.value })}
          placeholder="Numeric user ID"
          type="text"
          value={draft.granteeUserId}
        />
      </label>
      {draft.scopeType === "organization" ? (
        <label>
          <span>Organization ID</span>
          <input
            onChange={(event) => onDraftChange({ organizationId: event.target.value })}
            placeholder="Numeric organization ID"
            type="text"
            value={draft.organizationId}
          />
        </label>
      ) : null}
      {draft.scopeType === "course" ? (
        <label>
          <span>Course ID</span>
          <input
            onChange={(event) => onDraftChange({ courseId: event.target.value })}
            placeholder="Numeric course ID"
            type="text"
            value={draft.courseId}
          />
        </label>
      ) : null}
      <label>
        <span>Reason</span>
        <textarea
          onChange={(event) => onDraftChange({ reason: event.target.value })}
          placeholder="Optional reason for this delegation"
          rows={3}
          value={draft.reason}
        />
      </label>
      <label>
        <span>Expires at</span>
        <input
          onChange={(event) => onDraftChange({ expiresAt: event.target.value })}
          placeholder="Optional ISO date"
          type="text"
          value={draft.expiresAt}
        />
      </label>
      {error ? (
        <div className={styles.inlineError} role="alert">
          <AlertTriangle size={16} aria-hidden />
          <span>{error.message}</span>
        </div>
      ) : null}
      <button className={styles.primaryButton} disabled={!canSubmitDelegationCreate(draft, state)} type="submit">
        {state === "submitting" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <Send size={16} aria-hidden />}
        Create delegation
      </button>
    </form>
  );
}
