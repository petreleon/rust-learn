"use client";

import { Loader2, RefreshCw, Search } from "lucide-react";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type FraudBlockFilters } from "../model/FraudBlockFilters";
import { fraudBlockActiveOptions, fraudBlockScopeOptions } from "../model/fraudBlockDisplay";

export function FraudBlockFiltersPanel({
  filters,
  onApply,
  onRefresh,
  onReset,
  onUpdate,
  state,
}: {
  filters: FraudBlockFilters;
  onApply: () => void;
  onRefresh: () => void;
  onReset: () => void;
  onUpdate: (patch: Partial<FraudBlockFilters>) => void;
  state: LoadState;
}) {
  return (
    <section className={styles.filterPanel} aria-label="Fraud block filters">
      <label>
        <span>Search</span>
        <span className={styles.inputWithIcon}>
          <Search size={17} aria-hidden />
          <input
            onChange={(event) => onUpdate({ searchInput: event.target.value })}
            onKeyDown={(event) => {
              if (event.key === "Enter") {
                event.preventDefault();
                onApply();
              }
            }}
            placeholder="Block ID, reason"
            type="search"
            value={filters.searchInput}
          />
        </span>
      </label>
      <label>
        <span>Scope</span>
        <select
          onChange={(event) => onUpdate({ offset: 0, scopeType: event.target.value })}
          value={filters.scopeType}
        >
          {fraudBlockScopeOptions.map((option) => (
            <option key={option.label} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </label>
      <label>
        <span>Status</span>
        <select
          onChange={(event) => onUpdate({ active: activeValue(event.target.value), offset: 0 })}
          value={filters.active === true ? "true" : filters.active === false ? "false" : ""}
        >
          {fraudBlockActiveOptions.map((option) => (
            <option key={option.label} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </label>
      <div className={styles.filterActions}>
        <button className={styles.primaryButton} onClick={onApply} type="button">
          <Search size={16} aria-hidden />
          Apply
        </button>
        <button className={styles.secondaryButton} onClick={onReset} type="button">
          Reset
        </button>
        <button className={styles.secondaryButton} disabled={state === "loading"} onClick={onRefresh} type="button">
          {state === "loading" ? <Loader2 className={styles.spin} size={16} aria-hidden /> : <RefreshCw size={16} aria-hidden />}
          Refresh
        </button>
      </div>
    </section>
  );
}

function activeValue(value: string) {
  if (value === "") return null;
  return value === "true";
}
