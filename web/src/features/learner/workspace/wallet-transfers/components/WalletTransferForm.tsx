"use client";

import { Loader2, Send } from "lucide-react";
import {
  type WalletTransferDraft,
  type WalletTransferOperation,
} from "../model/walletTransferTypes";
import { walletTransferDraftIsReady } from "../model/walletTransferPayload";
import styles from "../../learner-workspace.module.css";

export function WalletTransferForm({
  draft,
  enabled,
  isSubmitting,
  onSubmit,
  onUpdate,
  operation,
}: {
  draft: WalletTransferDraft;
  enabled: boolean;
  isSubmitting: boolean;
  operation: WalletTransferOperation;
  onSubmit: () => void;
  onUpdate: (patch: Partial<WalletTransferDraft>) => void;
}) {
  const disabled = !enabled || isSubmitting;
  const title = operation === "deposit" ? "Deposit intent" : "Retire tokens";

  return (
    <form className={styles.transferCard} onSubmit={(event) => {
      event.preventDefault();
      onSubmit();
    }}>
      <h3>{title}</h3>
      <label>
        Amount
        <input
          disabled={disabled}
          min="0"
          step="0.0001"
          type="number"
          value={draft.amount}
          onChange={(event) => onUpdate({ amount: event.target.value })}
        />
      </label>
      <label>
        Ethereum address
        <input
          disabled={disabled}
          value={draft.ethereumAddress}
          onChange={(event) => onUpdate({ ethereumAddress: event.target.value })}
        />
      </label>
      <label>
        Gas payer
        <select
          disabled={disabled}
          value={draft.gasPayer}
          onChange={(event) => onUpdate({ gasPayer: event.target.value as WalletTransferDraft["gasPayer"] })}
        >
          <option value="platform">Platform paid</option>
          <option value="user">User paid</option>
        </select>
      </label>
      <details>
        <summary>Chain details</summary>
        <label>
          Chain ID
          <input disabled={disabled} value={draft.chainId} onChange={(event) => onUpdate({ chainId: event.target.value })} />
        </label>
        <label>
          Contract address
          <input disabled={disabled} value={draft.contractAddress} onChange={(event) => onUpdate({ contractAddress: event.target.value })} />
        </label>
        <label>
          Transaction hash
          <input disabled={disabled} value={draft.transactionHash} onChange={(event) => onUpdate({ transactionHash: event.target.value })} />
        </label>
        <label>
          Log index
          <input disabled={disabled} value={draft.logIndex} onChange={(event) => onUpdate({ logIndex: event.target.value })} />
        </label>
        <label>
          Platform address
          <input disabled={disabled} value={draft.platformAddress} onChange={(event) => onUpdate({ platformAddress: event.target.value })} />
        </label>
      </details>
      <button className={styles.primaryLink} disabled={disabled || !walletTransferDraftIsReady(draft)} type="submit">
        {isSubmitting ? <Loader2 className={styles.spin} size={18} aria-hidden /> : <Send size={18} aria-hidden />}
        {operation === "deposit" ? "Create deposit" : "Create retirement"}
      </button>
    </form>
  );
}
