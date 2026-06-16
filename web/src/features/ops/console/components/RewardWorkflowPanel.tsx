"use client";

import { ClipboardList, FileCheck, Send } from "lucide-react";
import styles from "../ops-console.module.css";
import { PROTECTED_ACTIONS } from "../model/PROTECTED_ACTIONS";
import { positiveIntegerInputProps } from "../model/positiveIntegerInputProps";
import { OpsServerDeniedNotice } from "./OpsServerDeniedNotice";
import { PermissionNotice } from "./PermissionNotice";
import { RequirementNotice } from "./RequirementNotice";
import { AmountDecisionControls, RewardSharedFields, TeacherRewardDecisionControls } from "./RewardWorkflowControls";
import { type OpsConsoleController } from "../route/useOpsConsoleController";

export function RewardWorkflowPanel({ controller }: { controller: OpsConsoleController }) {
  const { auth, reward } = controller;

  return (
    <section id="reward-workflow" className={styles.panel} aria-labelledby="reward-title">
      <div className={styles.panelHeader}>
        <div>
          <p className={styles.eyebrow}>Course rewards</p>
          <h2 id="reward-title">Candidate approval</h2>
        </div>
        <FileCheck size={22} aria-hidden />
      </div>
      {reward.showRewardSharedFields && <RewardSharedFields controller={controller} />}
      {!reward.canUseRewardWorkflow && (
        <PermissionNotice
          title="Reward permissions disabled"
          detail="Enable course reward permissions to submit, load, or approve candidates."
        />
      )}
      {reward.canSubmitReward && (
        <>
          {auth.hasSessionToken && reward.hasSubmitRewardCandidateDraft && (
            <RequirementNotice action="Submit candidate" fields={reward.submitRewardCandidateMissingFields} />
          )}
          <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.submitRewardCandidate} action="Submit candidate" />
          <div className={styles.actionStrip}>
            <input
              aria-label="Reward student user id"
              {...positiveIntegerInputProps}
              placeholder="Student user id"
              value={reward.rewardStudentId}
              onChange={(event) => reward.setRewardStudentId(event.target.value)}
            />
            <button
              type="button"
              className={styles.primaryButton}
              onClick={reward.submitRewardCandidate}
              {...auth.actionState(
                reward.canSubmitRewardCandidateForm,
                true,
                "Required permission",
                PROTECTED_ACTIONS.submitRewardCandidate,
              )}
            >
              <Send size={17} aria-hidden />
              <span>Submit candidate</span>
            </button>
          </div>
        </>
      )}
      {reward.canViewCourseRewards && (
        <>
          {auth.hasSessionToken && reward.hasLoadRewardCandidatesDraft && (
            <RequirementNotice action="Load candidates" fields={reward.loadRewardCandidatesMissingFields} />
          )}
          <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.courseRewardCandidates} action="Load candidates" />
          <button
            type="button"
            className={styles.secondaryButton}
            onClick={reward.loadRewardCandidates}
            {...auth.actionState(
              reward.canLoadRewardCandidatesForm,
              true,
              "Required permission",
              PROTECTED_ACTIONS.courseRewardCandidates,
            )}
          >
            <ClipboardList size={17} aria-hidden />
            <span>Load candidates</span>
          </button>
        </>
      )}
      {reward.canTeacherApproveReward && <TeacherRewardDecisionControls controller={controller} />}
      {reward.canApproveAmount && <AmountDecisionControls controller={controller} />}
    </section>
  );
}
