"use client";

import { History } from "lucide-react";
import styles from "../ops-console.module.css";
import { PROTECTED_ACTIONS } from "../model/PROTECTED_ACTIONS";
import { optionLabel } from "../model/optionLabel";
import { rewardStatuses } from "../model/rewardStatuses";
import { OpsServerDeniedNotice } from "./OpsServerDeniedNotice";
import { PermissionNotice } from "./PermissionNotice";
import { type OpsConsoleController } from "../route/useOpsConsoleController";

export function HistoryWorkflowPanel({ controller }: { controller: OpsConsoleController }) {
  const { auth, reward } = controller;

  return (
    <section id="history-workflow" className={styles.panel} aria-labelledby="history-title">
      <div className={styles.panelHeader}>
        <div>
          <p className={styles.eyebrow}>Student</p>
          <h2 id="history-title">Reward history</h2>
        </div>
        <History size={22} aria-hidden />
      </div>
      {!reward.canViewCourseRewards && (
        <PermissionNotice
          title="Reward history permission disabled"
          detail="Enable course reward status permission to load student history."
        />
      )}
      {reward.canViewCourseRewards && (
        <>
          <OpsServerDeniedNotice auth={auth} actionKey={PROTECTED_ACTIONS.studentRewardHistory} action="Load history" />
          <div className={styles.actionStrip}>
            <select
              aria-label="Reward history status filter"
              value={reward.historyStatus}
              onChange={(event) => reward.setHistoryStatus(event.target.value)}
            >
              <option value="">All statuses</option>
              {rewardStatuses.map((status) => (
                <option key={status} value={status}>
                  {optionLabel(status)}
                </option>
              ))}
            </select>
            <button
              type="button"
              className={styles.secondaryButton}
              onClick={reward.loadStudentHistory}
              {...auth.actionState(true, true, "Required permission", PROTECTED_ACTIONS.studentRewardHistory)}
            >
              <History size={17} aria-hidden />
              <span>Load history</span>
            </button>
          </div>
        </>
      )}
    </section>
  );
}
