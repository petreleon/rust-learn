"use client";

import { WalletCards } from "lucide-react";
import {
  type WalletTransferDraft,
  type WalletTransferOperation,
  type WalletTransferResult,
} from "../model/walletTransferTypes";
import { WalletTransferForm } from "../components/WalletTransferForm";
import { WalletTransferResultCard } from "../components/WalletTransferResultCard";
import { StatusPill } from "../../components/StatusPill";
import styles from "../../learner-workspace.module.css";

export function WalletTransferView({
  drafts,
  enabled,
  error,
  onSubmit,
  onUpdateDraft,
  pendingOperation,
  results,
  walletLinked,
}: {
  drafts: Record<WalletTransferOperation, WalletTransferDraft>;
  enabled: boolean;
  error: string | null;
  pendingOperation: WalletTransferOperation | null;
  results: WalletTransferResult[];
  walletLinked: boolean;
  onSubmit: (operation: WalletTransferOperation) => void;
  onUpdateDraft: (operation: WalletTransferOperation, patch: Partial<WalletTransferDraft>) => void;
}) {
  return (
    <section className={styles.section}>
      <div className={styles.sectionHeader}>
        <h2>Deposit and retire tokens</h2>
        <StatusPill label={enabled && walletLinked ? "Available" : "Locked"} tone={enabled && walletLinked ? "good" : "warn"} />
      </div>
      <p className={styles.muted}>
        Create a wallet intent, review the tax and wallet delta, then complete the required external wallet action.
      </p>
      {error ? <p className={styles.errorPanel}>{error}</p> : null}
      <div className={styles.transferGrid}>
        {(["deposit", "retire"] as const).map((operation) => (
          <WalletTransferForm
            draft={drafts[operation]}
            enabled={enabled && walletLinked}
            isSubmitting={pendingOperation === operation}
            key={operation}
            operation={operation}
            onSubmit={() => onSubmit(operation)}
            onUpdate={(patch) => onUpdateDraft(operation, patch)}
          />
        ))}
      </div>
      <div className={styles.sectionHeader}>
        <h3>Recent wallet intents</h3>
        <WalletCards size={18} aria-hidden />
      </div>
      {results.length ? (
        <div className={styles.transferResultGrid}>
          {results.map((result) => (
            <WalletTransferResultCard key={`${result.operation}-${transferResultId(result)}`} result={result} />
          ))}
        </div>
      ) : (
        <p className={styles.muted}>Created deposit and retirement intents will appear here during this session.</p>
      )}
    </section>
  );
}

function transferResultId(result: WalletTransferResult) {
  return result.operation === "deposit" ? result.id : result.transaction_id;
}
