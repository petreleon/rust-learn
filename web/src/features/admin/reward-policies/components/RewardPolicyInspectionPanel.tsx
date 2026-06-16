"use client";

import { FileSearch } from "lucide-react";
import { type RewardPolicyItem } from "@/lib/admin/RewardPolicyItem";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { EmptyState } from "@/features/admin/shared/route-kit/EmptyState";
import { formatDate } from "@/features/admin/shared/route-kit/formatDate";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { rewardPolicyInspectionRows } from "../model/rewardPolicyInspection";

export function RewardPolicyInspectionPanel({ policy }: { policy: RewardPolicyItem | null }) {
  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <FileSearch size={20} aria-hidden />
        <div>
          <h2>Policy inspection</h2>
          <p>Selected policy version, payout rules, scope identifiers, and ownership timestamps.</p>
        </div>
        {policy ? (
          <StatusPill label={policy.active ? "Active" : "Inactive"} tone={policy.active ? "good" : "neutral"} />
        ) : null}
      </div>
      {policy ? <PolicyInspectionDetails policy={policy} /> : <EmptyState text="Select a policy version to inspect." />}
    </section>
  );
}

function PolicyInspectionDetails({ policy }: { policy: RewardPolicyItem }) {
  return (
    <div className={styles.detailGrid}>
      <ContextRow label="Policy ID" value={`#${policy.id}`} />
      {rewardPolicyInspectionRows(policy).map((row) => (
        <ContextRow key={row.label} label={row.label} value={row.value} />
      ))}
      <ContextRow label="Created" value={formatDate(policy.created_at)} />
      <ContextRow label="Updated" value={formatDate(policy.updated_at)} />
    </div>
  );
}

function ContextRow({ label, value }: { label: string; value: string }) {
  return (
    <div className={styles.contextRow}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}
