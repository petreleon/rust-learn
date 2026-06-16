"use client";

import { AlertTriangle, Loader2, Send } from "lucide-react";
import { type PlatformRewardCandidateItem } from "@/lib/admin/PlatformRewardCandidateItem";
import { type RewardCandidateAmountDecisionStatus } from "@/lib/admin/RewardCandidateAmountDecisionStatus";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { type RewardAmountDecisionDraft } from "../model/RewardAmountDecisionDraft";
import { canSubmitRewardAmountDecision } from "../model/RewardAmountDecisionDraft";
import { type RewardAmountDecisionState } from "../model/RewardAmountDecisionState";
import { candidateIsFinal } from "../model/rewardCandidateDisplay";

export function RewardAmountDecisionForm({
  canApprove,
  candidate,
  draft,
  error,
  onDraftChange,
  onSubmit,
  state,
}: {
  canApprove: boolean;
  candidate: PlatformRewardCandidateItem;
  draft: RewardAmountDecisionDraft;
  error: RouteError | null;
  onDraftChange: (patch: Partial<RewardAmountDecisionDraft>) => void;
  onSubmit: () => void;
  state: RewardAmountDecisionState;
}) {
  const isFinal = candidateIsFinal(candidate);

  return (
    <form
      className={styles.decisionForm}
      onSubmit={(event) => {
        event.preventDefault();
        onSubmit();
      }}
    >
      <div className={styles.subsectionHeader}>
        <h3>Amount decision</h3>
        {isFinal ? <StatusPill label="Final state" tone="neutral" /> : null}
      </div>
      <label>
        <span>Decision status</span>
        <select
          disabled={isFinal}
          onChange={(event) =>
            onDraftChange({ status: event.target.value as RewardCandidateAmountDecisionStatus })
          }
          value={draft.status}
        >
          <option disabled={!canApprove} value="approved">
            Approve amount
          </option>
          <option value="rejected">Reject</option>
        </select>
      </label>
      {!canApprove ? (
        <p className={styles.muted}>
          Approve amount requires the APPROVE_REWARD_AMOUNT platform permission.
        </p>
      ) : null}
      {draft.status === "approved" ? (
        <label>
          <span>Approved amount</span>
          <input
            disabled={isFinal}
            onChange={(event) => onDraftChange({ amount: event.target.value })}
            placeholder="0.00"
            type="text"
            value={draft.amount}
          />
        </label>
      ) : null}
      <label>
        <span>Decision reason</span>
        <textarea
          disabled={isFinal}
          onChange={(event) => onDraftChange({ reason: event.target.value })}
          placeholder="Explain the amount decision or rejection reason."
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
        disabled={!canSubmitRewardAmountDecision({ canApprove, candidate, draft, state })}
        type="submit"
      >
        {state === "submitting" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <Send size={16} aria-hidden />}
        Save decision
      </button>
      {isFinal ? (
        <p className={styles.muted}>This candidate is in a final state and cannot be changed from this screen.</p>
      ) : null}
    </form>
  );
}
