"use client";

import { KeyRound, RefreshCw } from "lucide-react";
import styles from "../ops-console.module.css";
import { workflowNavItems } from "../model/workflowNavItems";
import { DelegationWorkflowPanel } from "./DelegationWorkflowPanel";
import { FraudWorkflowPanel } from "./FraudWorkflowPanel";
import { HistoryWorkflowPanel } from "./HistoryWorkflowPanel";
import { ReportWorkflowPanel } from "./ReportWorkflowPanel";
import { ResultPanel } from "./ResultPanel";
import { RewardWorkflowPanel } from "./RewardWorkflowPanel";
import { TeacherWorkflowPanel } from "./TeacherWorkflowPanel";
import { type OpsConsoleController } from "../route/useOpsConsoleController";

export function OpsWorkspace({ controller }: { controller: OpsConsoleController }) {
  const { auth } = controller;

  return (
    <>
      <header className={styles.topbar}>
        <div>
          <p className={styles.eyebrow}>Business console</p>
          <h1>Reward and teaching workflows</h1>
        </div>
        <button
          type="button"
          className={styles.iconButton}
          disabled={auth.apiState === "checking"}
          aria-label="Check API health"
          title="Check API health"
          onClick={auth.checkHealth}
        >
          <RefreshCw size={18} aria-hidden />
          <span>{auth.apiState === "checking" ? "Checking" : "Check API"}</span>
        </button>
      </header>
      <nav className={styles.workflowNav} aria-label="Workflow sections">
        {workflowNavItems.map((item) => (
          <a key={item.href} href={item.href}>
            {item.label}
          </a>
        ))}
      </nav>
      <section className={styles.metrics} aria-label="Workflow access">
        <div className={styles.metric}>
          <span>Teacher flow</span>
          <strong>{controller.teacherFlowStatus}</strong>
        </div>
        <div className={styles.metric}>
          <span>Reward flow</span>
          <strong>{controller.rewardFlowStatus}</strong>
        </div>
        <div className={styles.metric}>
          <span>Audit flow</span>
          <strong>{controller.auditFlowStatus}</strong>
        </div>
      </section>
      {!auth.hasSessionToken && (
        <section className={styles.workflowNotice} aria-label="Session requirement">
          <KeyRound size={18} aria-hidden />
          <div>
            <strong>JWT required</strong>
            <span>Protected workflow actions are locked until a session token is loaded.</span>
          </div>
        </section>
      )}
      <div className={styles.grid}>
        <TeacherWorkflowPanel controller={controller} />
        <RewardWorkflowPanel controller={controller} />
        <HistoryWorkflowPanel controller={controller} />
        <ReportWorkflowPanel controller={controller} />
        <FraudWorkflowPanel controller={controller} />
        <DelegationWorkflowPanel controller={controller} />
      </div>
      <ResultPanel auth={auth} />
    </>
  );
}
