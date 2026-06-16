"use client";

import { AlertTriangle, Loader2, ShieldCheck, XCircle } from "lucide-react";
import { type DelegationItem } from "@/lib/admin/DelegationItem";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/components/admin-routes.module.css";
import { ContextRow } from "@/components/admin-routes/ContextRow";
import { formatDate } from "@/components/admin-routes/formatDate";
import { formatUnderscoreLabel } from "@/components/admin-routes/formatUnderscoreLabel";
import { StatusPill } from "@/components/admin-routes/StatusPill";
import { type DelegationActionState } from "../model/DelegationActionState";
import { delegationScopeLabel, delegationStatus } from "../model/delegationDisplay";

export function DelegationDetailPanel({
  canRevoke,
  delegation,
  onRevoke,
  onRevokeReasonChange,
  revokeError,
  revokeReason,
  revokeState,
}: {
  canRevoke: boolean;
  delegation: DelegationItem;
  onRevoke: (delegationId: number) => void;
  onRevokeReasonChange: (reason: string) => void;
  revokeError: RouteError | null;
  revokeReason: string;
  revokeState: DelegationActionState;
}) {
  const status = delegationStatus(delegation);

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <ShieldCheck size={20} aria-hidden />
        <div>
          <h2>Delegation {delegation.id}</h2>
          <p>
            {formatUnderscoreLabel(delegation.permission)} · {delegationScopeLabel(delegation)}
          </p>
        </div>
        <StatusPill label={status} tone={status === "active" ? "good" : "neutral"} />
      </div>
      <div className={styles.detailGrid}>
        <ContextRow label="Permission" value={formatUnderscoreLabel(delegation.permission)} />
        <ContextRow label="Scope" value={delegationScopeLabel(delegation)} />
        <ContextRow label="Grantee" value={`User ${delegation.grantee_user_id}`} />
        <ContextRow label="Grantor" value={`User ${delegation.grantor_user_id}`} />
        <ContextRow label="Reason" value={delegation.reason || "None"} />
        <ContextRow label="Created" value={formatDate(delegation.created_at)} />
        <ContextRow label="Updated" value={formatDate(delegation.updated_at)} />
        <ContextRow label="Expires" value={delegation.expires_at ? formatDate(delegation.expires_at) : "No expiration"} />
        {delegation.revoked_at ? (
          <>
            <ContextRow label="Revoked" value={formatDate(delegation.revoked_at)} />
            <ContextRow label="Revoked by" value={`User ${delegation.revoked_by_user_id}`} />
            <ContextRow label="Revoke reason" value={delegation.revoke_reason || "None"} />
          </>
        ) : null}
      </div>
      {!delegation.revoked_at && canRevoke ? (
        <div className={styles.textBlock}>
          <h3>Revoke delegation</h3>
          <label>
            <span>Revoke reason</span>
            <textarea
              onChange={(event) => onRevokeReasonChange(event.target.value)}
              placeholder="Optional reason for revocation"
              rows={3}
              value={revokeReason}
            />
          </label>
          {revokeError ? (
            <div className={styles.inlineError} role="alert">
              <AlertTriangle size={16} aria-hidden />
              <span>{revokeError.message}</span>
            </div>
          ) : null}
          <button
            className={styles.primaryButton}
            disabled={revokeState === "submitting"}
            onClick={() => onRevoke(delegation.id)}
            type="button"
          >
            {revokeState === "submitting" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <XCircle size={16} aria-hidden />}
            Revoke delegation
          </button>
        </div>
      ) : null}
      {delegation.revoked_at ? <p className={styles.muted}>This delegation was revoked and cannot be changed.</p> : null}
      {!delegation.revoked_at && !canRevoke ? (
        <p className={styles.muted}>Revoke requires the MANAGE_ROLE_PERMISSIONS permission.</p>
      ) : null}
    </section>
  );
}
