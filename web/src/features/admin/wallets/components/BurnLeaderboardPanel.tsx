"use client";

import { Flame } from "lucide-react";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { PanelError } from "@/features/admin/shared/route-kit/PanelError";
import { PanelLoading } from "@/features/admin/shared/route-kit/PanelLoading";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import {
  type BurnLeaderboard,
  type BurnLeaderboardScope,
  type BurnLeaderboardWindow,
} from "../model/BurnLeaderboard";

export function BurnLeaderboardPanel({
  error,
  leaderboard,
  onRetry,
  onScopeChange,
  onWindowChange,
  scope,
  state,
  windowRange,
}: {
  error: RouteError | null;
  leaderboard: BurnLeaderboard | null;
  scope: BurnLeaderboardScope;
  state: LoadState;
  windowRange: BurnLeaderboardWindow;
  onRetry: () => void;
  onScopeChange: (scope: BurnLeaderboardScope) => void;
  onWindowChange: (windowRange: BurnLeaderboardWindow) => void;
}) {
  if (state === "loading" || state === "idle") {
    return <PanelLoading title="Loading burn leaderboard" />;
  }

  if (state === "error" || !leaderboard) {
    return <PanelError error={error} onRetry={onRetry} title="Burn leaderboard failed" />;
  }

  return (
    <section className={styles.panel} aria-label="Burn leaderboard">
      <div className={styles.panelHeader}>
        <Flame size={20} aria-hidden />
        <div>
          <h2>Burn leaderboard</h2>
          <p>Rolling LearnToken burns by users and organizations.</p>
        </div>
      </div>
      <div className={styles.filterActions}>
        <select
          aria-label="Burn leaderboard window"
          value={windowRange}
          onChange={(event) => onWindowChange(event.target.value as BurnLeaderboardWindow)}
        >
          <option value="7d">7 days</option>
          <option value="30d">30 days</option>
          <option value="365d">365 days</option>
        </select>
        <select
          aria-label="Burn leaderboard scope"
          value={scope}
          onChange={(event) => onScopeChange(event.target.value as BurnLeaderboardScope)}
        >
          <option value="all">All burners</option>
          <option value="users">Users</option>
          <option value="organizations">Organizations</option>
        </select>
      </div>
      <div className={styles.rowList}>
        {leaderboard.rows.length ? (
          leaderboard.rows.map((row) => (
            <article
              className={styles.compactRow}
              key={`${row.burner_type}-${row.user_id ?? row.organization_id}`}
            >
              <div>
                <strong>
                  #{row.rank} {burnerLabel(row.burner_type, row.user_id, row.organization_id)}
                </strong>
                <small>
                  {row.burn_count} burns in {leaderboard.window_days} days
                </small>
              </div>
              <div className={styles.rowMeta}>
                <StatusPill label={`${row.total_burned} burned`} tone="good" />
              </div>
            </article>
          ))
        ) : (
          <p className={styles.muted}>No burns found for this rolling window.</p>
        )}
      </div>
    </section>
  );
}

function burnerLabel(type: string, userId: number | null, organizationId: number | null) {
  if (type === "organization") return `Organization ${organizationId ?? "unknown"}`;
  return `User ${userId ?? "unknown"}`;
}
