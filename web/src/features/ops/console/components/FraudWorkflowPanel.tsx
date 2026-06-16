"use client";

import { ShieldAlert } from "lucide-react";
import styles from "../ops-console.module.css";
import { FraudCreateControls, FraudUseControls } from "./FraudWorkflowControls";
import { PermissionNotice } from "./PermissionNotice";
import { type OpsConsoleController } from "../route/useOpsConsoleController";

export function FraudWorkflowPanel({ controller }: { controller: OpsConsoleController }) {
  const { fraud } = controller;

  return (
    <section id="fraud-workflow" className={styles.panel} aria-labelledby="fraud-title">
      <div className={styles.panelHeader}>
        <div>
          <p className={styles.eyebrow}>Fraud controls</p>
          <h2 id="fraud-title">Reward blocks</h2>
        </div>
        <ShieldAlert size={22} aria-hidden />
      </div>
      {!fraud.canUseFraudWorkflow && (
        <PermissionNotice
          title="Fraud permissions disabled"
          detail="Enable fraud audit or management permissions to inspect reward blocks."
        />
      )}
      {fraud.canManageFraud && <FraudCreateControls controller={controller} />}
      {fraud.canUseFraudBlockActions && <FraudUseControls controller={controller} />}
    </section>
  );
}
