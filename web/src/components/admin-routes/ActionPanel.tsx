"use client";

import { ShieldCheck } from "lucide-react";
import { platformCapabilityEnabled, type PlatformAdminWorkspace, type PlatformCapabilityKey, type PlatformFraudDashboard, type PlatformRewardDashboard, type PlatformSystemStatus } from "@/lib/admin";
import styles from "../admin-routes.module.css";
import { StatusPill } from "./StatusPill";
import { actionIcons } from "./actionIcons";
import { capabilityLabel } from "./capabilityLabel";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";

export function ActionPanel({
  canApproveRewardAmount,
  canExportData,
  canManageFraud,
  canViewRewardAudit,
  fraudDashboard,
  rewardDashboard,
  systemStatus,
  workspace,
}: {
  canApproveRewardAmount: boolean;
  canExportData: boolean;
  canManageFraud: boolean;
  canViewRewardAudit: boolean;
  fraudDashboard: PlatformFraudDashboard | null;
  rewardDashboard: PlatformRewardDashboard | null;
  systemStatus: PlatformSystemStatus | null;
  workspace: PlatformAdminWorkspace;
}) {
  const readinessTone = systemStatus?.readiness.status === "ready" ? "good" : systemStatus ? "warn" : "neutral";
  const cards: Array<{
    detail: string;
    key: PlatformCapabilityKey;
    label: string;
    state: string;
    tone: "good" | "neutral" | "warn";
    value: string | number;
  }> = [
    {
      detail: "Submitted teacher applications waiting for platform review.",
      key: "teacher_applications",
      label: "Teacher review",
      state: capabilityLabel(workspace, "teacher_applications"),
      tone: rewardDashboard?.teacher_applications.submitted ? "warn" : "neutral",
      value: rewardDashboard?.teacher_applications.submitted ?? "Gated",
    },
    {
      detail: "Teacher-approved candidates waiting for platform amount decision.",
      key: "reward_amount_review",
      label: "Amount review",
      state: canApproveRewardAmount ? "Approval enabled" : canViewRewardAudit ? "Audit only" : "Missing reward audit",
      tone: rewardDashboard?.pending_amount_approval_count ? "warn" : "neutral",
      value: rewardDashboard?.pending_amount_approval_count ?? "Gated",
    },
    {
      detail: "Active reward fraud blocks across teachers, organizations, courses, and policies.",
      key: "fraud_blocks",
      label: "Fraud controls",
      state: canManageFraud ? "Manage enabled" : canViewRewardAudit ? "Audit only" : "Missing reward audit",
      tone: fraudDashboard?.active_total ? "warn" : "neutral",
      value: fraudDashboard?.active_total ?? "Gated",
    },
    {
      detail: "Delegated permissions with scope, expiration, and revocation audit.",
      key: "delegations",
      label: "Delegations",
      state: platformCapabilityEnabled(workspace, "delegations") ? "Available" : "Gated",
      tone: platformCapabilityEnabled(workspace, "delegations") ? "good" : "neutral",
      value: platformCapabilityEnabled(workspace, "delegations") ? "Ready" : "Gated",
    },
    {
      detail: "CSV exports available for reports, reward operations, wallets, and delegations.",
      key: "exports",
      label: "Exports",
      state: canExportData ? "CSV enabled" : "Missing EXPORT_DATA",
      tone: canExportData ? "good" : "neutral",
      value: canExportData ? "Ready" : "Gated",
    },
    {
      detail: "Wallet and transaction audit permissions visible in the current platform scope.",
      key: "wallets",
      label: "Wallet audit",
      state: capabilityLabel(workspace, "wallets"),
      tone: platformCapabilityEnabled(workspace, "wallets") ? "good" : "neutral",
      value: platformCapabilityEnabled(workspace, "wallets") ? "Available" : "Gated",
    },
    {
      detail: "API liveness and dependency readiness from the runtime health endpoints.",
      key: "system",
      label: "System",
      state: systemStatus?.readiness.status ? formatUnderscoreLabel(systemStatus.readiness.status) : "Loading",
      tone: readinessTone,
      value: systemStatus?.liveness.status || "Checking",
    },
  ];

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <ShieldCheck size={20} aria-hidden />
        <div>
          <h2>Review lanes</h2>
          <p>Counts and gated lanes follow resolved platform permissions from the current session.</p>
        </div>
      </div>
      <div className={styles.actionGrid}>
        {cards.map((card) => {
          const Icon = actionIcons[card.key];
          return (
            <article className={styles.actionCard} key={card.key}>
              <div className={styles.actionTop}>
                <span className={styles.smallIcon}>
                  <Icon size={19} aria-hidden />
                </span>
                <StatusPill label={card.state} tone={card.tone} />
              </div>
              <strong>{card.value}</strong>
              <span>{card.label}</span>
              <p>{card.detail}</p>
            </article>
          );
        })}
      </div>
    </section>
  );
}
