"use client";

import { ShieldAlert } from "lucide-react";
import { type FraudBlockItem } from "@/lib/admin/FraudBlockItem";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { EmptyState } from "@/features/admin/shared/route-kit/EmptyState";
import { formatDate } from "@/features/admin/shared/route-kit/formatDate";
import { formatUnderscoreLabel } from "@/features/admin/shared/route-kit/formatUnderscoreLabel";
import { PanelError } from "@/features/admin/shared/route-kit/PanelError";
import { PanelLoading } from "@/features/admin/shared/route-kit/PanelLoading";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { type FraudBlockListResult } from "../api/fraudBlocksApi";
import { type FraudBlockFilters } from "../model/FraudBlockFilters";
import { fraudBlockStatus, fraudBlockTargetLabel } from "../model/fraudBlockDisplay";

export function FraudBlockListPanel({
  blocks,
  error,
  filters,
  onPageOffset,
  onRefresh,
  onSelect,
  selectedBlockId,
  state,
}: {
  blocks: FraudBlockListResult | null;
  error: RouteError | null;
  filters: FraudBlockFilters;
  onPageOffset: (offset: number) => void;
  onRefresh: () => void;
  onSelect: (blockId: number) => void;
  selectedBlockId: number | null;
  state: LoadState;
}) {
  if (state === "loading" || state === "idle") return <PanelLoading title="Loading fraud blocks" />;
  if (state === "error") return <PanelError error={error} onRetry={onRefresh} title="Fraud blocks failed" />;
  if (!blocks) return null;

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <ShieldAlert size={20} aria-hidden />
        <div>
          <h2>Fraud block list</h2>
          <p>{blocks.total} block{blocks.total === 1 ? "" : "s"} match the current filters.</p>
        </div>
      </div>
      {blocks.blocks.length ? (
        <div className={styles.rowList}>
          {blocks.blocks.map((block) => (
            <FraudBlockListRow
              block={block}
              isSelected={selectedBlockId === block.id}
              key={block.id}
              onSelect={onSelect}
            />
          ))}
        </div>
      ) : (
        <EmptyState text="No fraud blocks match these filters." />
      )}
      <FraudBlockPagination blocks={blocks} filters={filters} onPageOffset={onPageOffset} />
    </section>
  );
}

function FraudBlockListRow({
  block,
  isSelected,
  onSelect,
}: {
  block: FraudBlockItem;
  isSelected: boolean;
  onSelect: (blockId: number) => void;
}) {
  const status = fraudBlockStatus(block);

  return (
    <article className={`${styles.compactRow} ${isSelected ? styles.selectedRow : ""}`}>
      <div>
        <strong>Block #{block.id}</strong>
        <span>{block.reason}</span>
        <small>
          {formatUnderscoreLabel(block.scope_type)} · {fraudBlockTargetLabel(block)} · {formatDate(block.created_at)}
        </small>
      </div>
      <div className={styles.rowMeta}>
        <StatusPill label={formatUnderscoreLabel(status)} tone={status === "active" ? "warn" : "neutral"} />
        <span>{formatDate(block.updated_at)}</span>
        <button
          aria-label={`Inspect block ${block.id}`}
          className={styles.secondaryButton}
          onClick={() => onSelect(block.id)}
          type="button"
        >
          Inspect
        </button>
      </div>
    </article>
  );
}

function FraudBlockPagination({
  blocks,
  filters,
  onPageOffset,
}: {
  blocks: FraudBlockListResult;
  filters: FraudBlockFilters;
  onPageOffset: (offset: number) => void;
}) {
  const start = blocks.blocks.length ? filters.offset + 1 : 0;
  const end = Math.min(filters.offset + blocks.blocks.length, blocks.total);

  return (
    <div className={styles.paginationRow}>
      <button
        className={styles.secondaryButton}
        disabled={filters.offset <= 0}
        onClick={() => onPageOffset(filters.offset - blocks.limit)}
        type="button"
      >
        Previous
      </button>
      <span>
        Showing {start}-{end} of {blocks.total}
      </span>
      <button
        className={styles.secondaryButton}
        disabled={filters.offset + blocks.limit >= blocks.total}
        onClick={() => onPageOffset(filters.offset + blocks.limit)}
        type="button"
      >
        Next
      </button>
    </div>
  );
}
