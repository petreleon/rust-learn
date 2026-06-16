"use client";

import { Loader2, RefreshCw, Search } from "lucide-react";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RewardPolicyFilters } from "../model/RewardPolicyFilters";
import {
  rewardPolicyActiveOptions,
  rewardPolicyEventOptions,
  rewardPolicyScopeOptions,
} from "../model/rewardPolicyDisplay";

export function RewardPolicyFiltersPanel({
  filters,
  onApply,
  onRefresh,
  onReset,
  onUpdate,
  state,
}: {
  filters: RewardPolicyFilters;
  onApply: () => void;
  onRefresh: () => void;
  onReset: () => void;
  onUpdate: (patch: Partial<RewardPolicyFilters>) => void;
  state: LoadState;
}) {
  return (
    <section className={styles.filterPanel} aria-label="Reward policy filters">
      <label>
        <span>Scope</span>
        <select onChange={(event) => onUpdate({ offset: 0, scopeType: event.target.value })} value={filters.scopeType}>
          {rewardPolicyScopeOptions.map((option) => (
            <option key={option.label} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </label>
      <label>
        <span>Event</span>
        <select onChange={(event) => onUpdate({ eventType: event.target.value, offset: 0 })} value={filters.eventType}>
          {rewardPolicyEventOptions.map((option) => (
            <option key={option.label} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </label>
      <label>
        <span>Status</span>
        <select onChange={(event) => onUpdate({ active: activeValue(event.target.value), offset: 0 })} value={filters.active === true ? "true" : filters.active === false ? "false" : ""}>
          {rewardPolicyActiveOptions.map((option) => (
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
