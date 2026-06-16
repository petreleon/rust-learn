"use client";

import { AlertTriangle, Coins, Loader2, Save } from "lucide-react";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { MetricCard } from "@/features/admin/shared/route-kit/MetricCard";
import { PanelError } from "@/features/admin/shared/route-kit/PanelError";
import { PanelLoading } from "@/features/admin/shared/route-kit/PanelLoading";
import {
  type WalletTokenTaxDraft,
  type WalletTokenTaxOperation,
  type WalletTokenTaxSettings,
} from "../model/WalletTokenTax";

export function WalletTokenTaxPanel({
  canSetDeposit,
  canSetRetire,
  draft,
  error,
  onChange,
  onRetry,
  onSave,
  savingOperation,
  settings,
  state,
}: {
  canSetDeposit: boolean;
  canSetRetire: boolean;
  draft: WalletTokenTaxDraft;
  error: RouteError | null;
  onChange: (operation: WalletTokenTaxOperation, value: string) => void;
  onRetry: () => void;
  onSave: (operation: WalletTokenTaxOperation) => void;
  savingOperation: WalletTokenTaxOperation | null;
  settings: WalletTokenTaxSettings | null;
  state: LoadState;
}) {
  if (state === "loading" || state === "idle") {
    return <PanelLoading title="Loading token tax configuration" />;
  }

  if (state === "error" || !settings) {
    return <PanelError error={error} onRetry={onRetry} title="Token tax configuration failed" />;
  }

  return (
    <section className={styles.panel} aria-label="Token tax configuration">
      <div className={styles.panelHeader}>
        <Coins size={20} aria-hidden />
        <div>
          <h2>Token tax configuration</h2>
          <p>Platform-paid gas tax amounts applied when learner deposits or retirements use platform gas.</p>
        </div>
      </div>
      {error ? (
        <div className={styles.inlineError} role="alert">
          <AlertTriangle size={16} aria-hidden />
          <span>{error.message}</span>
        </div>
      ) : null}
      <div className={styles.metricGrid}>
        <MetricCard label="Deposit tax" value={settings.deposit.tax_amount} />
        <MetricCard label="Retirement tax" value={settings.retire.tax_amount} />
      </div>
      <form className={styles.decisionForm} onSubmit={(event) => event.preventDefault()}>
        <TokenTaxInput
          canSave={canSetDeposit}
          label="Deposit tax"
          operation="deposit"
          savingOperation={savingOperation}
          value={draft.deposit}
          onChange={onChange}
          onSave={onSave}
        />
        <TokenTaxInput
          canSave={canSetRetire}
          label="Retirement tax"
          operation="retire"
          savingOperation={savingOperation}
          value={draft.retire}
          onChange={onChange}
          onSave={onSave}
        />
      </form>
    </section>
  );
}

function TokenTaxInput({
  canSave,
  label,
  operation,
  onChange,
  onSave,
  savingOperation,
  value,
}: {
  canSave: boolean;
  label: string;
  operation: WalletTokenTaxOperation;
  onChange: (operation: WalletTokenTaxOperation, value: string) => void;
  onSave: (operation: WalletTokenTaxOperation) => void;
  savingOperation: WalletTokenTaxOperation | null;
  value: string;
}) {
  const saving = savingOperation === operation;

  return (
    <div className={styles.contextRow}>
      <label htmlFor={`wallet-token-tax-${operation}`}>{label}</label>
      <input
        disabled={!canSave || saving}
        id={`wallet-token-tax-${operation}`}
        inputMode="decimal"
        min="0"
        step="0.00000001"
        type="number"
        value={value}
        onChange={(event) => onChange(operation, event.target.value)}
      />
      <button
        className={styles.secondaryButton}
        disabled={!canSave || saving || !value.trim()}
        type="button"
        onClick={() => onSave(operation)}
      >
        {saving ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <Save size={16} aria-hidden />}
        Save {operation} tax
      </button>
      {!canSave ? <span className={styles.muted}>Requires {operation === "deposit" ? "SET_DEPOSIT_TAX" : "SET_RETIRE_TAX"}.</span> : null}
    </div>
  );
}
