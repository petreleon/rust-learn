"use client";

import { Landmark } from "lucide-react";
import { type PlatformRewardCandidateItem } from "@/lib/admin/PlatformRewardCandidateItem";
import { type PlatformRewardCandidatesResponse } from "@/lib/admin/PlatformRewardCandidatesResponse";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { EmptyState } from "@/features/admin/shared/route-kit/EmptyState";
import { formatDate } from "@/features/admin/shared/route-kit/formatDate";
import { formatUnderscoreLabel } from "@/features/admin/shared/route-kit/formatUnderscoreLabel";
import { PanelError } from "@/features/admin/shared/route-kit/PanelError";
import { PanelLoading } from "@/features/admin/shared/route-kit/PanelLoading";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { type RewardCandidateFilters } from "../model/RewardCandidateFilters";
import { rewardCandidateStatusTone } from "../model/rewardCandidateDisplay";

export function RewardCandidateListPanel({
  candidates,
  error,
  filters,
  onPageOffset,
  onRefresh,
  onSelect,
  selectedCandidateId,
  state,
}: {
  candidates: PlatformRewardCandidatesResponse | null;
  error: RouteError | null;
  filters: RewardCandidateFilters;
  onPageOffset: (offset: number) => void;
  onRefresh: () => void;
  onSelect: (candidateId: number) => void;
  selectedCandidateId: number | null;
  state: LoadState;
}) {
  if (state === "loading" || state === "idle") return <PanelLoading title="Loading reward candidates" />;
  if (state === "error") return <PanelError error={error} onRetry={onRefresh} title="Reward candidate queue failed" />;
  if (!candidates) return null;

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <Landmark size={20} aria-hidden />
        <div>
          <h2>Review queue</h2>
          <p>{candidates.total} candidate{candidates.total === 1 ? "" : "s"} match the current filters.</p>
        </div>
      </div>
      {candidates.candidates.length ? (
        <div className={styles.rowList}>
          {candidates.candidates.map((candidate) => (
            <RewardCandidateListRow
              candidate={candidate}
              isSelected={selectedCandidateId === candidate.id}
              key={candidate.id}
              onSelect={onSelect}
            />
          ))}
        </div>
      ) : (
        <EmptyState text="No reward candidates match these filters." />
      )}
      <RewardCandidatePagination candidates={candidates} filters={filters} onPageOffset={onPageOffset} />
    </section>
  );
}

function RewardCandidateListRow({
  candidate,
  isSelected,
  onSelect,
}: {
  candidate: PlatformRewardCandidateItem;
  isSelected: boolean;
  onSelect: (candidateId: number) => void;
}) {
  return (
    <article className={`${styles.compactRow} ${isSelected ? styles.selectedRow : ""}`}>
      <div>
        <strong>{candidate.student.name}</strong>
        <span>{candidate.student.email}</span>
        <small>
          {candidate.course.title} · {formatUnderscoreLabel(candidate.event_type)}
        </small>
      </div>
      <div className={styles.rowMeta}>
        <StatusPill
          label={formatUnderscoreLabel(candidate.status)}
          tone={rewardCandidateStatusTone(candidate.status)}
        />
        <span>{formatDate(candidate.updated_at)}</span>
        <button
          aria-label={`Review ${candidate.student.name}`}
          className={styles.secondaryButton}
          onClick={() => onSelect(candidate.id)}
          type="button"
        >
          Review
        </button>
      </div>
    </article>
  );
}

function RewardCandidatePagination({
  candidates,
  filters,
  onPageOffset,
}: {
  candidates: PlatformRewardCandidatesResponse;
  filters: RewardCandidateFilters;
  onPageOffset: (offset: number) => void;
}) {
  const start = candidates.candidates.length ? filters.offset + 1 : 0;
  const end = Math.min(filters.offset + candidates.candidates.length, candidates.total);

  return (
    <div className={styles.paginationRow}>
      <button
        className={styles.secondaryButton}
        disabled={filters.offset <= 0}
        onClick={() => onPageOffset(filters.offset - candidates.limit)}
        type="button"
      >
        Previous
      </button>
      <span>
        Showing {start}-{end} of {candidates.total}
      </span>
      <button
        className={styles.secondaryButton}
        disabled={filters.offset + candidates.limit >= candidates.total}
        onClick={() => onPageOffset(filters.offset + candidates.limit)}
        type="button"
      >
        Next
      </button>
    </div>
  );
}
