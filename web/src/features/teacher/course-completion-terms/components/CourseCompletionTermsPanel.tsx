"use client";

import { CheckCircle2, Coins, RotateCcw, Send, Users, XCircle } from "lucide-react";
import { type FormEvent, type ReactNode } from "react";
import { type ActionState } from "@/shared/route-state/ActionState";
import styles from "@/features/teacher/shared/teacher-routes.module.css";
import {
  type CourseCompletionTerms,
  type CourseCompletionTermsHistory,
} from "../model/CourseCompletionTerms";
import { type TermsDraft } from "../model/TermsDraft";
import { statusLabel } from "../model/termsDisplay";
import { TermsAuditList } from "./TermsAuditList";

export function CourseCompletionTermsPanel({
  actionMessage,
  actionState,
  canDecide,
  canPropose,
  decisionNote,
  draft,
  history,
  loadState,
  onAccept,
  onCounter,
  onDecisionNoteChange,
  onDraftChange,
  onReject,
  onSubmit,
  onWithdraw,
  openTerms,
}: CourseCompletionTermsPanelProps) {
  function submitProposal(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    onSubmit();
  }

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <Coins size={20} aria-hidden />
        <h2>Completion terms</h2>
      </div>
      {actionMessage ? <p className={styles.muted} role="status">{actionMessage}</p> : null}
      {loadState === "loading" ? <p className={styles.muted}>Loading negotiated terms.</p> : null}
      {loadState === "error" ? <p className={styles.muted}>Completion terms could not be loaded.</p> : null}
      {history ? <TermsSummary activeTerms={history.active_terms} openTerms={openTerms} /> : null}

      <form className={styles.decisionForm} onSubmit={submitProposal}>
        <label>
          <span>Reward per completion</span>
          <input
            disabled={(!canPropose && !canDecide) || actionState === "saving"}
            inputMode="decimal"
            onChange={(event) => onDraftChange({ ...draft, completionRewardAmount: event.target.value })}
            placeholder="10"
            value={draft.completionRewardAmount}
          />
        </label>
        <label>
          <span>Max students</span>
          <input
            disabled={(!canPropose && !canDecide) || actionState === "saving"}
            inputMode="numeric"
            onChange={(event) => onDraftChange({ ...draft, maxEnrolledStudents: event.target.value })}
            placeholder="25"
            value={draft.maxEnrolledStudents}
          />
        </label>
        <label>
          <span>Negotiation note</span>
          <textarea
            disabled={(!canPropose && !canDecide) || actionState === "saving"}
            onChange={(event) => onDraftChange({ ...draft, note: event.target.value })}
            rows={2}
            value={draft.note}
          />
        </label>
        <div className={styles.actionRow}>
          <button className={styles.primaryButton} disabled={!canPropose || actionState === "saving"} type="submit">
            <Send size={17} aria-hidden />
            Submit proposal
          </button>
          <button
            className={styles.secondaryButton}
            disabled={!canDecide || !openTerms || actionState === "saving"}
            onClick={onCounter}
            type="button"
          >
            <RotateCcw size={17} aria-hidden />
            Counter
          </button>
        </div>
      </form>

      {openTerms ? (
        <div className={styles.decisionForm}>
          <label>
            <span>Decision note</span>
            <textarea
              disabled={actionState === "saving"}
              onChange={(event) => onDecisionNoteChange(event.target.value)}
              rows={2}
              value={decisionNote}
            />
          </label>
          <div className={styles.actionRow}>
            <button className={styles.primaryButton} disabled={!canDecide || actionState === "saving"} onClick={onAccept} type="button">
              <CheckCircle2 size={17} aria-hidden />
              Accept
            </button>
            <button className={styles.secondaryButton} disabled={!canDecide || actionState === "saving"} onClick={onReject} type="button">
              <XCircle size={17} aria-hidden />
              Reject
            </button>
            <button className={styles.secondaryButton} disabled={!canPropose || actionState === "saving"} onClick={onWithdraw} type="button">
              <RotateCcw size={17} aria-hidden />
              Withdraw
            </button>
          </div>
        </div>
      ) : null}

      {history ? <TermsAuditList history={history} /> : null}
    </section>
  );
}

type CourseCompletionTermsPanelProps = {
  actionMessage: string | null;
  actionState: ActionState;
  canDecide: boolean;
  canPropose: boolean;
  decisionNote: string;
  draft: TermsDraft;
  history: CourseCompletionTermsHistory | null;
  loadState: "idle" | "loading" | "success" | "error";
  onAccept: () => void;
  onCounter: () => void;
  onDecisionNoteChange: (note: string) => void;
  onDraftChange: (draft: TermsDraft) => void;
  onReject: () => void;
  onSubmit: () => void;
  onWithdraw: () => void;
  openTerms: CourseCompletionTerms | null;
};

function TermsSummary({
  activeTerms,
  openTerms,
}: {
  activeTerms: CourseCompletionTerms | null;
  openTerms: CourseCompletionTerms | null;
}) {
  return (
    <div className={styles.detailList}>
      <TermsLine icon={<Coins size={17} aria-hidden />} label="Active reward" terms={activeTerms} />
      <TermsLine icon={<Users size={17} aria-hidden />} label="Open negotiation" terms={openTerms} />
    </div>
  );
}

function TermsLine({ icon, label, terms }: { icon: ReactNode; label: string; terms: CourseCompletionTerms | null }) {
  const value = terms
    ? `${terms.completion_reward_amount} tokens, ${terms.max_enrolled_students} students, ${statusLabel(terms.status)}`
    : "None";
  return (
    <p className={styles.statusLine}>
      {icon}
      <strong>{label}</strong>
      <span>{value}</span>
    </p>
  );
}
