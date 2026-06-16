"use client";

import { Landmark } from "lucide-react";
import { type PlatformRewardCandidateItem } from "@/lib/admin/PlatformRewardCandidateItem";
import { type RewardAuditEvent } from "@/lib/admin/RewardAuditEvent";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { ContextRow } from "@/components/admin-routes/ContextRow";
import { formatDate } from "@/components/admin-routes/formatDate";
import { formatUnderscoreLabel } from "@/components/admin-routes/formatUnderscoreLabel";
import { StatusPill } from "@/components/admin-routes/StatusPill";
import { type RewardAmountDecisionDraft } from "../model/RewardAmountDecisionDraft";
import { type RewardAmountDecisionState } from "../model/RewardAmountDecisionState";
import { rewardCandidateStatusTone } from "../model/rewardCandidateDisplay";
import { RewardAmountDecisionForm } from "./RewardAmountDecisionForm";
import { RewardCandidateAuditPanel } from "./RewardCandidateAuditPanel";

export function RewardCandidateDetailPanel({
  auditError,
  auditEvents,
  auditState,
  canApprove,
  candidate,
  decisionDraft,
  decisionError,
  decisionState,
  onDecisionDraftChange,
  onRefreshAudit,
  onSubmitDecision,
}: {
  auditError: RouteError | null;
  auditEvents: RewardAuditEvent[];
  auditState: LoadState;
  canApprove: boolean;
  candidate: PlatformRewardCandidateItem | null;
  decisionDraft: RewardAmountDecisionDraft;
  decisionError: RouteError | null;
  decisionState: RewardAmountDecisionState;
  onDecisionDraftChange: (patch: Partial<RewardAmountDecisionDraft>) => void;
  onRefreshAudit: () => void;
  onSubmitDecision: () => void;
}) {
  if (!candidate) return <EmptyRewardCandidateDetail />;

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <Landmark size={20} aria-hidden />
        <div>
          <h2>Candidate {candidate.id}</h2>
          <p>{candidate.student.name} · {candidate.student.email}</p>
        </div>
        <StatusPill
          label={formatUnderscoreLabel(candidate.status)}
          tone={rewardCandidateStatusTone(candidate.status)}
        />
      </div>
      <RewardCandidateContext candidate={candidate} />
      <RewardCandidateAuditPanel
        error={auditError}
        events={auditEvents}
        onRefresh={onRefreshAudit}
        state={auditState}
      />
      <RewardAmountDecisionForm
        canApprove={canApprove}
        candidate={candidate}
        draft={decisionDraft}
        error={decisionError}
        onDraftChange={onDecisionDraftChange}
        onSubmit={onSubmitDecision}
        state={decisionState}
      />
    </section>
  );
}

function EmptyRewardCandidateDetail() {
  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <Landmark size={20} aria-hidden />
        <div>
          <h2>Candidate detail</h2>
          <p>Select a candidate from the queue to inspect context and audit history.</p>
        </div>
      </div>
    </section>
  );
}

function RewardCandidateContext({ candidate }: { candidate: PlatformRewardCandidateItem }) {
  return (
    <div className={styles.detailGrid}>
      <ContextRow label="Student" value={candidate.student.name} />
      <ContextRow label="Course" value={candidate.course.title} />
      <ContextRow label="Event type" value={formatUnderscoreLabel(candidate.event_type)} />
      <ContextRow label="Submitter" value={candidate.submitter.name} />
      <ContextRow label="Teacher approver" value={candidate.teacher_approver?.name || "No approver yet"} />
      <ContextRow label="Teacher decision" value={candidate.teacher_decision_reason || "No reason recorded"} />
      <ContextRow label="Approved amount" value={candidate.approved_amount ?? "Not set"} />
      <ContextRow label="Created" value={formatDate(candidate.created_at)} />
      <ContextRow label="Updated" value={formatDate(candidate.updated_at)} />
    </div>
  );
}
