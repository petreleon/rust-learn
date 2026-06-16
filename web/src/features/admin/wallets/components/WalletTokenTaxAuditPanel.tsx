"use client";

import { History } from "lucide-react";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { MetricCard } from "@/features/admin/shared/route-kit/MetricCard";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import {
  type WalletTokenTaxAuditEvent,
  type WalletTokenTaxSettings,
} from "../model/WalletTokenTax";

const dateFormatter = new Intl.DateTimeFormat("en", {
  dateStyle: "medium",
  timeStyle: "short",
});

export function WalletTokenTaxAuditPanel({
  events,
  settings,
}: {
  events: WalletTokenTaxAuditEvent[];
  settings: WalletTokenTaxSettings;
}) {
  return (
    <section className={styles.panel} aria-label="Token tax history">
      <div className={styles.panelHeader}>
        <History size={20} aria-hidden />
        <div>
          <h2>Token tax history</h2>
          <p>Effective tax changes and wallet impact for platform-paid gas flows.</p>
        </div>
      </div>
      <div className={styles.metricGrid}>
        <MetricCard label="Deposit wallet impact" value={`Amount - ${settings.deposit.tax_amount}`} />
        <MetricCard label="Retirement wallet impact" value={`Amount + ${settings.retire.tax_amount}`} />
      </div>
      {events.length ? (
        <div className={styles.auditList}>
          {events.map((event) => (
            <article className={styles.compactRow} key={event.id}>
              <div>
                <strong>{event.operation === "deposit" ? "Deposit tax" : "Retirement tax"}</strong>
                <span>
                  {event.previous_tax_amount} to {event.new_tax_amount}
                </span>
                <small>
                  Actor {event.actor_user_id ?? "system"} · {formatDate(event.created_at)}
                </small>
              </div>
              <div className={styles.rowMeta}>
                <StatusPill label="Effective" tone="good" />
              </div>
            </article>
          ))}
        </div>
      ) : (
        <div className={styles.emptyState}>No token tax changes have been recorded yet.</div>
      )}
    </section>
  );
}

function formatDate(value: string) {
  const date = new Date(value);
  return Number.isNaN(date.getTime()) ? value : dateFormatter.format(date);
}
