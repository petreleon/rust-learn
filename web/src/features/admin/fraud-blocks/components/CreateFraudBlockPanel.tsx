"use client";

import { AlertTriangle, Loader2, Send, ShieldAlert } from "lucide-react";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { type FraudBlockActionState } from "../model/FraudBlockActionState";
import {
  canSubmitFraudBlockCreate,
  type FraudBlockCreateDraft,
} from "../model/FraudBlockCreateDraft";
import { fraudBlockCreateScopeOptions } from "../model/fraudBlockDisplay";

export function CreateFraudBlockPanel({
  draft,
  error,
  onDraftChange,
  onSubmit,
  state,
}: {
  draft: FraudBlockCreateDraft;
  error: RouteError | null;
  onDraftChange: (patch: Partial<FraudBlockCreateDraft>) => void;
  onSubmit: () => void;
  state: FraudBlockActionState;
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
        <ShieldAlert size={20} aria-hidden />
        <div>
          <h2>Create block</h2>
          <p>Add a new fraud block.</p>
        </div>
      </div>
      <label>
        <span>Scope type</span>
        <select onChange={(event) => onDraftChange({ scopeType: event.target.value })} value={draft.scopeType}>
          {fraudBlockCreateScopeOptions.map((option) => (
            <option key={option.label} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </label>
      <label>
        <span>Target ID</span>
        <input
          onChange={(event) => onDraftChange({ targetId: event.target.value })}
          placeholder="Optional numeric ID for the scope target"
          type="text"
          value={draft.targetId}
        />
      </label>
      <label>
        <span>Reason</span>
        <textarea
          onChange={(event) => onDraftChange({ reason: event.target.value })}
          placeholder="Explain why this fraud block is being created."
          rows={4}
          value={draft.reason}
        />
      </label>
      <label>
        <span>Evidence reference</span>
        <input
          onChange={(event) => onDraftChange({ evidence: event.target.value })}
          placeholder="Optional case or ticket reference"
          type="text"
          value={draft.evidence}
        />
      </label>
      {error ? (
        <div className={styles.inlineError} role="alert">
          <AlertTriangle size={16} aria-hidden />
          <span>{error.message}</span>
        </div>
      ) : null}
      <button className={styles.primaryButton} disabled={!canSubmitFraudBlockCreate(draft, state)} type="submit">
        {state === "submitting" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <Send size={16} aria-hidden />}
        Create block
      </button>
    </form>
  );
}
