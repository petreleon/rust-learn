"use client";

import { Flame } from "lucide-react";
import { StatusPill } from "@/features/organization/shared/route-kit/StatusPill";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { OrganizationBurnForm } from "../components/OrganizationBurnForm";
import { OrganizationBurnResultCard } from "../components/OrganizationBurnResultCard";
import {
  type OrganizationBurnDraft,
  type OrganizationBurnPermissions,
  type OrganizationTokenBurn,
} from "../model/organizationBurnTypes";

export function OrganizationBurnView({
  burns,
  draft,
  enabled,
  error,
  loading,
  onSubmit,
  onUpdateDraft,
  permissions,
  submitting,
}: {
  burns: OrganizationTokenBurn[];
  draft: OrganizationBurnDraft;
  enabled: boolean;
  error: string | null;
  loading: boolean;
  permissions: OrganizationBurnPermissions | null;
  submitting: boolean;
  onSubmit: () => void;
  onUpdateDraft: (patch: Partial<OrganizationBurnDraft>) => void;
}) {
  const ready = enabled && Boolean(permissions?.can_request_burn);

  return (
    <section className={styles.panel}>
      <div className={styles.sectionHeader}>
        <div>
          <h2>Token burns</h2>
          <p className={styles.muted}>Centralized, direct MetaMask, and platform-mediated burn records.</p>
        </div>
        <StatusPill label={burnStatusLabel(enabled, permissions, loading)} tone={ready ? "good" : "warn"} />
      </div>
      <div className={styles.permissionRow}>
        <span className={styles.permissionChip}>{permissions?.required_permission ?? "BURN_ORGANIZATION_TOKENS"}</span>
        <span className={styles.permissionChip}>{permissions?.kyc_verified ? "KYC verified" : "KYC required"}</span>
      </div>
      {error ? <p className={styles.inlineError}>{error}</p> : null}
      <OrganizationBurnForm
        draft={draft}
        enabled={ready}
        isSubmitting={submitting}
        onSubmit={onSubmit}
        onUpdate={onUpdateDraft}
      />
      <div className={styles.sectionHeader}>
        <h3>Recent burns</h3>
        <Flame size={18} aria-hidden />
      </div>
      {burns.length ? (
        <div className={styles.compactList}>
          {burns.slice(0, 6).map((burn) => (
            <OrganizationBurnResultCard burn={burn} key={burn.id} />
          ))}
        </div>
      ) : (
        <p className={styles.muted}>No organization token burns recorded yet.</p>
      )}
    </section>
  );
}

function burnStatusLabel(
  enabled: boolean,
  permissions: OrganizationBurnPermissions | null,
  loading: boolean,
) {
  if (!enabled) return "Wallet required";
  if (loading || !permissions) return "Checking";
  if (permissions.can_request_burn) return "Available";
  if (!permissions.kyc_verified) return "KYC required";
  return "Permission required";
}
