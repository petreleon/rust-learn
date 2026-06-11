"use client";

import { CheckCircle2, Database, XCircle } from "lucide-react";
import { type PlatformSystemStatus } from "@/lib/admin";
import styles from "../admin-routes.module.css";
import { MetricCard } from "./MetricCard";
import { PanelError } from "./PanelError";
import { PanelLoading } from "./PanelLoading";
import { StatusPill } from "./StatusPill";
import { dependencyLabel } from "./dependencyLabel";
import { formatUnderscoreLabel } from "./formatUnderscoreLabel";
import { type RouteError } from "./RouteError";
import { type SectionState } from "./SectionState";

export function SystemPanel({
  error,
  onRetry,
  state,
  status,
}: {
  error: RouteError | null;
  onRetry: () => void;
  state: SectionState;
  status: PlatformSystemStatus | null;
}) {
  if (state === "loading" || state === "idle") {
    return <PanelLoading title="Loading system status" />;
  }

  if (state === "error" || !status) {
    return <PanelError error={error} onRetry={onRetry} title="System status failed" />;
  }

  const ready = status.readiness.status === "ready";

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <Database size={20} aria-hidden />
        <div>
          <h2>System status</h2>
          <p>Readiness checks cover API dependencies instead of only shallow liveness.</p>
        </div>
        <StatusPill label={formatUnderscoreLabel(status.readiness.status)} tone={ready ? "good" : "warn"} />
      </div>
      <div className={styles.metricGrid}>
        <MetricCard label="API liveness" value={formatUnderscoreLabel(status.liveness.status)} tone={status.liveness.status === "ok" ? "good" : "warn"} />
        <MetricCard label="Readiness" value={formatUnderscoreLabel(status.readiness.status)} tone={ready ? "good" : "warn"} />
      </div>
      <div className={styles.rowList}>
        {status.readiness.checks.map((check) => (
          <article className={styles.compactRow} key={check.name}>
            <div>
              <strong>{dependencyLabel(check.name)}</strong>
              <span>{check.message || "Dependency check passed."}</span>
            </div>
            <StatusPill
              icon={check.status === "ok" ? <CheckCircle2 size={15} aria-hidden /> : <XCircle size={15} aria-hidden />}
              label={formatUnderscoreLabel(check.status)}
              tone={check.status === "ok" ? "good" : "warn"}
            />
          </article>
        ))}
      </div>
    </section>
  );
}
