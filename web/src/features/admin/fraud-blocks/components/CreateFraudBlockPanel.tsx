"use client";

import { AlertTriangle, Loader2, Send, ShieldAlert } from "lucide-react";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { type FraudBlockActionState } from "../model/FraudBlockActionState";
import {
  canSubmitFraudBlockCreate,
  type FraudBlockCreateDraft,
} from "../model/FraudBlockCreateDraft";
import { fraudBlockCreateScopeOptions } from "../model/fraudBlockDisplay";
import { type RewardPolicyOption } from "../model/RewardPolicyOption";

export function CreateFraudBlockPanel({
  canListRewardPolicies,
  draft,
  error,
  onDraftChange,
  onRewardPolicyRetry,
  onSubmit,
  rewardPolicyError,
  rewardPolicyOptions,
  rewardPolicyState,
  state,
}: {
  canListRewardPolicies: boolean;
  draft: FraudBlockCreateDraft;
  error: RouteError | null;
  onDraftChange: (patch: Partial<FraudBlockCreateDraft>) => void;
  onRewardPolicyRetry: () => void;
  onSubmit: () => void;
  rewardPolicyError: RouteError | null;
  rewardPolicyOptions: RewardPolicyOption[];
  rewardPolicyState: LoadState;
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
        <select
          onChange={(event) => onDraftChange({ scopeType: event.target.value, targetId: "" })}
          value={draft.scopeType}
        >
          {fraudBlockCreateScopeOptions.map((option) => (
            <option key={option.label} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </label>
      {draft.scopeType === "reward_policy" && canListRewardPolicies ? (
        <RewardPolicySelect
          error={rewardPolicyError}
          onChange={(targetId) => onDraftChange({ targetId })}
          onRetry={onRewardPolicyRetry}
          options={rewardPolicyOptions}
          state={rewardPolicyState}
          value={draft.targetId}
        />
      ) : (
        <TargetIdInput draft={draft} onDraftChange={onDraftChange} />
      )}
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

function RewardPolicySelect({
  error,
  onChange,
  onRetry,
  options,
  state,
  value,
}: {
  error: RouteError | null;
  onChange: (targetId: string) => void;
  onRetry: () => void;
  options: RewardPolicyOption[];
  state: LoadState;
  value: string;
}) {
  return (
    <>
      <label>
        <span>Reward policy</span>
        <select disabled={state === "loading"} onChange={(event) => onChange(event.target.value)} value={value}>
          <option value="">{state === "loading" ? "Loading policies..." : "Select active policy..."}</option>
          {options.map((option) => (
            <option key={option.id} value={String(option.id)}>
              {option.label}
            </option>
          ))}
        </select>
      </label>
      {state === "error" && error ? (
        <button className={styles.secondaryButton} type="button" onClick={onRetry}>
          Retry policy list
        </button>
      ) : null}
      {state === "success" && !options.length ? <span className={styles.muted}>No active reward policies.</span> : null}
    </>
  );
}

function TargetIdInput({
  draft,
  onDraftChange,
}: {
  draft: FraudBlockCreateDraft;
  onDraftChange: (patch: Partial<FraudBlockCreateDraft>) => void;
}) {
  return (
    <label>
      <span>{draft.scopeType === "reward_policy" ? "Reward policy ID" : "Target ID"}</span>
      <input
        onChange={(event) => onDraftChange({ targetId: event.target.value })}
        placeholder="Numeric ID for the scope target"
        type="text"
        value={draft.targetId}
      />
    </label>
  );
}
