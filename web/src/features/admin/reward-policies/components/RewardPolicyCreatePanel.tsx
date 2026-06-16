"use client";

import { ListPlus } from "lucide-react";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { type RewardPolicyDraft } from "../model/RewardPolicyDraft";
import {
  rewardPolicyCreateEventOptions,
  rewardPolicyPaymentOptions,
} from "../model/rewardPolicyDisplay";

export function RewardPolicyCreatePanel({
  draft,
  error,
  onDraftChange,
  onSubmit,
  state,
}: {
  draft: RewardPolicyDraft;
  error: RouteError | null;
  onDraftChange: (patch: Partial<RewardPolicyDraft>) => void;
  onSubmit: () => void;
  state: LoadState;
}) {
  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <ListPlus size={20} aria-hidden />
        <div>
          <h2>Create policy</h2>
          <p>New active policies version the matching scope and event while preserving older policy rows.</p>
        </div>
        <StatusPill label={draft.active ? "Active on create" : "Inactive on create"} tone={draft.active ? "good" : "neutral"} />
      </div>
      <form className={styles.decisionForm} onSubmit={(event) => {
        event.preventDefault();
        onSubmit();
      }}>
        <PolicyScopeFields draft={draft} onDraftChange={onDraftChange} />
        <label>
          <span>Event type</span>
          <select onChange={(event) => onDraftChange({ eventType: event.target.value })} value={draft.eventType}>
            {rewardPolicyCreateEventOptions.map((option) => (
              <option key={option.label} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </label>
        <label>
          <span>Payment strategy</span>
          <select onChange={(event) => onDraftChange({ paymentStrategy: event.target.value })} value={draft.paymentStrategy}>
            {rewardPolicyPaymentOptions.map((option) => (
              <option key={option.label} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </label>
        <AmountFields draft={draft} onDraftChange={onDraftChange} />
        <label>
          <span>Initial status</span>
          <select onChange={(event) => onDraftChange({ active: event.target.value === "true" })} value={String(draft.active)}>
            <option value="true">Active</option>
            <option value="false">Inactive</option>
          </select>
        </label>
        {error ? <p className={styles.inlineError}>{error.message}</p> : null}
        <button className={styles.primaryButton} disabled={state === "loading"} type="submit">
          <ListPlus size={16} aria-hidden />
          {state === "loading" ? "Creating..." : "Create policy"}
        </button>
      </form>
    </section>
  );
}

function PolicyScopeFields({
  draft,
  onDraftChange,
}: {
  draft: RewardPolicyDraft;
  onDraftChange: (patch: Partial<RewardPolicyDraft>) => void;
}) {
  return (
    <>
      <label>
        <span>Scope type</span>
        <select onChange={(event) => onDraftChange({ scopeType: event.target.value })} value={draft.scopeType}>
          <option value="platform">Platform</option>
          <option value="organization">Organization</option>
          <option value="course">Course</option>
        </select>
      </label>
      {draft.scopeType !== "platform" ? (
        <label>
          <span>Organization ID</span>
          <input onChange={(event) => onDraftChange({ organizationId: event.target.value })} placeholder="Required for organization scope" type="text" value={draft.organizationId} />
        </label>
      ) : null}
      {draft.scopeType === "course" ? (
        <label>
          <span>Course ID</span>
          <input onChange={(event) => onDraftChange({ courseId: event.target.value })} placeholder="Course id" type="text" value={draft.courseId} />
        </label>
      ) : null}
    </>
  );
}

function AmountFields({
  draft,
  onDraftChange,
}: {
  draft: RewardPolicyDraft;
  onDraftChange: (patch: Partial<RewardPolicyDraft>) => void;
}) {
  return (
    <>
      <label>
        <span>Token amount</span>
        <input onChange={(event) => onDraftChange({ tokenAmount: event.target.value })} placeholder="25" type="text" value={draft.tokenAmount} />
      </label>
      <label>
        <span>Multiplier</span>
        <input onChange={(event) => onDraftChange({ multiplier: event.target.value })} placeholder="1" type="text" value={draft.multiplier} />
      </label>
      <label>
        <span>Max payout</span>
        <input onChange={(event) => onDraftChange({ maxPayout: event.target.value })} placeholder="Optional" type="text" value={draft.maxPayout} />
      </label>
      <label>
        <span>Cooldown seconds</span>
        <input onChange={(event) => onDraftChange({ cooldownSeconds: event.target.value })} placeholder="0" type="text" value={draft.cooldownSeconds} />
      </label>
    </>
  );
}
