"use client";

import { Loader2, ShieldAlert } from "lucide-react";
import { type FraudBlockAuditEvent } from "@/lib/admin/FraudBlockAuditEvent";
import { type FraudBlockItem } from "@/lib/admin/FraudBlockItem";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { ContextRow } from "@/components/admin-routes/ContextRow";
import { formatDate } from "@/components/admin-routes/formatDate";
import { formatUnderscoreLabel } from "@/components/admin-routes/formatUnderscoreLabel";
import { StatusPill } from "@/components/admin-routes/StatusPill";
import { type FraudBlockActionState } from "../model/FraudBlockActionState";
import { fraudBlockStatus, fraudBlockTargetLabel } from "../model/fraudBlockDisplay";
import { FraudBlockAuditPanel } from "./FraudBlockAuditPanel";

export function FraudBlockDetailPanel({
  auditError,
  auditEvents,
  auditState,
  block,
  canRevoke,
  onRefreshAudit,
  onRevoke,
  revokeState,
}: {
  auditError: RouteError | null;
  auditEvents: FraudBlockAuditEvent[];
  auditState: LoadState;
  block: FraudBlockItem;
  canRevoke: boolean;
  onRefreshAudit: () => void;
  onRevoke: (blockId: number) => void;
  revokeState: FraudBlockActionState;
}) {
  const status = fraudBlockStatus(block);

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <ShieldAlert size={20} aria-hidden />
        <div>
          <h2>Block {block.id}</h2>
          <p>Inspect context and audit history.</p>
        </div>
        <StatusPill label={formatUnderscoreLabel(status)} tone={status === "revoked" ? "neutral" : "warn"} />
      </div>
      <div className={styles.detailGrid}>
        <ContextRow label="Scope type" value={formatUnderscoreLabel(block.scope_type)} />
        <ContextRow label="Target" value={fraudBlockTargetLabel(block)} />
        <ContextRow label="Reason" value={block.reason} />
        <ContextRow label="Evidence" value={block.evidence_reference || "None"} />
        <ContextRow label="Created" value={formatDate(block.created_at)} />
        <ContextRow label="Updated" value={formatDate(block.updated_at)} />
      </div>
      <FraudBlockAuditPanel
        error={auditError}
        events={auditEvents}
        onRefresh={onRefreshAudit}
        state={auditState}
      />
      <FraudBlockRevokeSection block={block} canRevoke={canRevoke} onRevoke={onRevoke} state={revokeState} />
    </section>
  );
}

function FraudBlockRevokeSection({
  block,
  canRevoke,
  onRevoke,
  state,
}: {
  block: FraudBlockItem;
  canRevoke: boolean;
  onRevoke: (blockId: number) => void;
  state: FraudBlockActionState;
}) {
  if (block.revoked_at) {
    return <p className={styles.muted}>This block was revoked and cannot be changed.</p>;
  }
  if (!canRevoke) {
    return <p className={styles.muted}>Revoke requires the MANAGE_REWARD_FRAUD_BLOCKS permission.</p>;
  }

  return (
    <div className={styles.textBlock}>
      <h3>Revoke block</h3>
      <button
        className={styles.primaryButton}
        disabled={state === "submitting"}
        onClick={() => onRevoke(block.id)}
        type="button"
      >
        {state === "submitting" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : null}
        Revoke
      </button>
    </div>
  );
}
