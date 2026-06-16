"use client";

import { CheckCircle2, WalletCards } from "lucide-react";
import styles from "../ops-console.module.css";
import { PROTECTED_ACTIONS } from "../model/PROTECTED_ACTIONS";
import { decimalInputProps } from "../model/decimalInputProps";
import { optionLabel } from "../model/optionLabel";
import { positiveIntegerInputProps } from "../model/positiveIntegerInputProps";
import { rewardStatuses } from "../model/rewardStatuses";
import { OpsServerDeniedNotice } from "./OpsServerDeniedNotice";
import { RequirementNotice } from "./RequirementNotice";
import { type OpsConsoleController } from "../route/useOpsConsoleController";

export function RewardSharedFields({ controller }: { controller: OpsConsoleController }) {
  const { reward } = controller;

  return (
    <div className={styles.actionStrip}>
      {reward.showRewardCourseId && (
        <input
          aria-label="Reward course id"
          {...positiveIntegerInputProps}
          placeholder="Course id"
          value={reward.rewardCourseId}
          onChange={(event) => reward.setRewardCourseId(event.target.value)}
        />
      )}
      {reward.showRewardCandidateId && (
        <input
          aria-label="Reward candidate id"
          {...positiveIntegerInputProps}
          placeholder="Candidate id"
          value={reward.rewardCandidateId}
          onChange={(event) => reward.setRewardCandidateId(event.target.value)}
        />
      )}
      {reward.showRewardStatusFilter && (
        <select
          aria-label="Reward candidate status filter"
          value={reward.rewardStatus}
          onChange={(event) => reward.setRewardStatus(event.target.value)}
        >
          {rewardStatuses.map((status) => (
            <option key={status} value={status}>
              {optionLabel(status)}
            </option>
          ))}
        </select>
      )}
    </div>
  );
}

export function TeacherRewardDecisionControls({ controller }: { controller: OpsConsoleController }) {
  const { auth, reward } = controller;

  return (
    <>
      {auth.hasSessionToken && reward.hasTeacherRewardDecisionDraft && (
        <RequirementNotice action="Teacher decision" fields={reward.teacherRewardDecisionMissingFields} />
      )}
      <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.courseRewardDecision} action="Teacher decision" />
      <div className={styles.actionStrip}>
        <select
          aria-label="Teacher reward decision status"
          value={reward.teacherRewardDecision.status}
          onChange={(event) => reward.setTeacherRewardDecision((current) => ({ ...current, status: event.target.value }))}
        >
          <option value="approved">{optionLabel("approved")}</option>
          <option value="rejected">{optionLabel("rejected")}</option>
        </select>
        <input
          aria-label="Teacher reward decision reason"
          placeholder="Teacher reason"
          value={reward.teacherRewardDecision.decision_reason}
          onChange={(event) => reward.setTeacherRewardDecision((current) => ({ ...current, decision_reason: event.target.value }))}
        />
        <button
          type="button"
          className={styles.secondaryButton}
          onClick={reward.decideStudentReward}
          {...auth.actionState(
            reward.canDecideStudentRewardForm,
            true,
            "Required permission",
            PROTECTED_ACTIONS.courseRewardDecision,
          )}
        >
          <CheckCircle2 size={17} aria-hidden />
          <span>Teacher decision</span>
        </button>
      </div>
    </>
  );
}

export function AmountDecisionControls({ controller }: { controller: OpsConsoleController }) {
  const { auth, reward } = controller;

  return (
    <>
      {auth.hasSessionToken && reward.hasAmountDecisionDraft && (
        <RequirementNotice action="Set amount" fields={reward.amountDecisionMissingFields} />
      )}
      <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.rewardAmountDecision} action="Set amount" />
      <div className={styles.actionStrip}>
        <select
          aria-label="Reward amount decision status"
          value={reward.amountDecision.status}
          onChange={(event) => reward.setAmountDecision((current) => ({ ...current, status: event.target.value }))}
        >
          <option value="approved">{optionLabel("approved")}</option>
          <option value="rejected">{optionLabel("rejected")}</option>
        </select>
        {reward.amountDecision.status === "approved" && (
          <input
            aria-label="Approved reward amount"
            {...decimalInputProps}
            placeholder="Amount"
            value={reward.amountDecision.approved_amount}
            onChange={(event) => reward.setAmountDecision((current) => ({ ...current, approved_amount: event.target.value }))}
          />
        )}
        <input
          aria-label="Reward amount decision reason"
          placeholder="Amount reason"
          value={reward.amountDecision.decision_reason}
          onChange={(event) => reward.setAmountDecision((current) => ({ ...current, decision_reason: event.target.value }))}
        />
        <button
          type="button"
          className={styles.secondaryButton}
          onClick={reward.decideRewardAmount}
          {...auth.actionState(
            reward.canDecideRewardAmountForm,
            true,
            "Required permission",
            PROTECTED_ACTIONS.rewardAmountDecision,
          )}
        >
          <WalletCards size={17} aria-hidden />
          <span>Set amount</span>
        </button>
      </div>
    </>
  );
}
