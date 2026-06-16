"use client";

import { Flame, Loader2 } from "lucide-react";
import { organizationBurnDraftIsReady, organizationBurnFeePath } from "../model/organizationBurnPayload";
import { type OrganizationBurnDraft } from "../model/organizationBurnTypes";
import styles from "@/features/organization/shared/organization-routes.module.css";

export function OrganizationBurnForm({
  draft,
  enabled,
  isSubmitting,
  onSubmit,
  onUpdate,
}: {
  draft: OrganizationBurnDraft;
  enabled: boolean;
  isSubmitting: boolean;
  onSubmit: () => void;
  onUpdate: (patch: Partial<OrganizationBurnDraft>) => void;
}) {
  const disabled = !enabled || isSubmitting;

  return (
    <form className={styles.filterPanel} onSubmit={(event) => {
      event.preventDefault();
      onSubmit();
    }}>
      <label>
        Amount
        <input disabled={disabled} min="0" onChange={(event) => onUpdate({ amount: event.target.value })} step="0.0001" type="number" value={draft.amount} />
      </label>
      <label>
        Source
        <select disabled={disabled} onChange={(event) => onUpdate({ source: event.target.value as OrganizationBurnDraft["source"] })} value={draft.source}>
          <option value="centralized_wallet">Centralized wallet</option>
          <option value="decentralized_direct">MetaMask direct</option>
          <option value="decentralized_platform_mediated">Platform mediated</option>
        </select>
      </label>
      <label>
        Fee path
        <input disabled value={organizationBurnFeePath(draft.source).replaceAll("_", " ")} />
      </label>
      <label>
        Ethereum address
        <input disabled={disabled} onChange={(event) => onUpdate({ ethereumAddress: event.target.value })} value={draft.ethereumAddress} />
      </label>
      <details>
        <summary>Chain evidence</summary>
        <label>
          Transaction hash
          <input disabled={disabled} onChange={(event) => onUpdate({ transactionHash: event.target.value })} value={draft.transactionHash} />
        </label>
        <label>
          Deposit intent ID
          <input disabled={disabled} onChange={(event) => onUpdate({ depositIntentId: event.target.value })} value={draft.depositIntentId} />
        </label>
        <label>
          Chain ID
          <input disabled={disabled} onChange={(event) => onUpdate({ chainId: event.target.value })} value={draft.chainId} />
        </label>
        <label>
          Contract address
          <input disabled={disabled} onChange={(event) => onUpdate({ contractAddress: event.target.value })} value={draft.contractAddress} />
        </label>
        <label>
          Log index
          <input disabled={disabled} onChange={(event) => onUpdate({ logIndex: event.target.value })} value={draft.logIndex} />
        </label>
      </details>
      <label>
        <input checked={draft.leaderboardVisible} disabled={disabled} onChange={(event) => onUpdate({ leaderboardVisible: event.target.checked })} type="checkbox" />
        Count on leaderboard
      </label>
      <button className={styles.primaryButton} disabled={disabled || !organizationBurnDraftIsReady(draft)} type="submit">
        {isSubmitting ? <Loader2 className={styles.spin} size={18} aria-hidden /> : <Flame size={18} aria-hidden />}
        Request burn
      </button>
    </form>
  );
}
