"use client";

import { IdCard } from "lucide-react";
import { type FormEvent, useCallback, useEffect, useMemo, useState } from "react";
import { hasPlatformAdminAccess } from "@/lib/access";
import { buildPlatformAdminWorkspace, decideKycSubmission, fetchKycReviewQueue, type KycDecisionStatus, type KycReviewQueueResponse } from "@/lib/admin";
import { ProductShell, type ShellNotice } from "@/components/product-shell";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { AdminDeniedState } from "@/components/admin-routes/AdminDeniedState";
import { GatedPanel } from "@/components/admin-routes/GatedPanel";
import { KycReviewDetail } from "@/components/admin-routes/KycReviewDetail";
import { KycReviewQueue } from "@/components/admin-routes/KycReviewQueue";
import { KycReviewSummaryPanel } from "@/components/admin-routes/KycReviewSummaryPanel";
import { LoadingState } from "@/components/admin-routes/LoadingState";
import { PanelError } from "@/components/admin-routes/PanelError";
import { PanelLoading } from "@/components/admin-routes/PanelLoading";
import { SessionErrorState } from "@/components/admin-routes/SessionErrorState";
import { SignedOutState } from "@/components/admin-routes/SignedOutState";
import { StatusPill } from "@/components/admin-routes/StatusPill";
import { emptyWorkspace } from "@/components/admin-routes/emptyWorkspace";
import { getCapability } from "@/components/admin-routes/getCapability";
import { hasPlatformCapability } from "@/components/admin-routes/hasPlatformCapability";
import { normalizeRouteError } from "@/components/admin-routes/normalizeRouteError";
import { useAdminSession } from "@/components/admin-routes/useAdminSession";
import { useKycReviewAudit } from "@/components/admin-routes/useKycReviewAudit";
import { type RouteError } from "@/components/admin-routes/RouteError";
import { type SectionState } from "@/components/admin-routes/SectionState";

type DecisionState = "idle" | "submitting" | "success" | "error";

export function AdminKycReviewRoute() {
  const route = useAdminSession();
  const workspace = useMemo(() => (route.session ? buildPlatformAdminWorkspace(route.session) : emptyWorkspace), [route.session]);
  const allowed = route.session ? hasPlatformAdminAccess(route.session) : false;
  const canReview = hasPlatformCapability(workspace, "kyc_reviews");
  const [queue, setQueue] = useState<KycReviewQueueResponse | null>(null);
  const [queueError, setQueueError] = useState<RouteError | null>(null);
  const [queueState, setQueueState] = useState<SectionState>("idle");
  const [selectedSubmissionId, setSelectedSubmissionId] = useState<number | null>(null);
  const [decisionStatus, setDecisionStatus] = useState<KycDecisionStatus>("verified");
  const [rejectionReason, setRejectionReason] = useState("");
  const [decisionError, setDecisionError] = useState<RouteError | null>(null);
  const [decisionState, setDecisionState] = useState<DecisionState>("idle");
  const selectedSubmission =
    queue?.submissions.find((submission) => submission.id === selectedSubmissionId) || queue?.submissions[0] || null;
  const audit = useKycReviewAudit({ canReview, submissionId: selectedSubmission?.id ?? null, token: route.token ?? undefined });

  const loadQueue = useCallback(async () => {
    const token = route.token;
    if (!route.session || !token || !allowed || !canReview) {
      return;
    }
    setQueueState("loading");
    setQueueError(null);
    try {
      const nextQueue = await fetchKycReviewQueue({ token });
      setQueue(nextQueue);
      setQueueState("success");
      setSelectedSubmissionId((current) =>
        current && nextQueue.submissions.some((submission) => submission.id === current)
          ? current
          : nextQueue.submissions[0]?.id || null,
      );
    } catch (error) {
      setQueue(null);
      setQueueError(normalizeRouteError(error, "KYC review queue could not be loaded."));
      setQueueState("error");
    }
  }, [allowed, canReview, route.session, route.token]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadQueue(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadQueue]);

  function selectSubmission(submissionId: number) {
    setSelectedSubmissionId(submissionId);
    setDecisionStatus("verified");
    setRejectionReason("");
    setDecisionError(null);
    setDecisionState("idle");
  }

  async function submitDecision(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = route.token;
    if (!token || !selectedSubmission) {
      return;
    }
    const trimmedReason = rejectionReason.trim();
    if (decisionStatus === "rejected" && !trimmedReason) {
      setDecisionError({ code: "validation_error", message: "Add a rejection reason before rejecting KYC.", status: 400 });
      setDecisionState("error");
      return;
    }
    setDecisionError(null);
    setDecisionState("submitting");
    try {
      await decideKycSubmission({
        rejectionReason: trimmedReason,
        status: decisionStatus,
        submissionId: selectedSubmission.id,
        token,
      });
      setDecisionState("success");
      setRejectionReason("");
      await loadQueue();
    } catch (error) {
      setDecisionError(normalizeRouteError(error, "KYC decision could not be saved."));
      setDecisionState("error");
    }
  }

  const notice: ShellNotice | null =
    decisionState === "success"
      ? { message: "The KYC queue was refreshed after the decision.", title: "KYC decision saved", tone: "success" }
      : decisionState === "error" && decisionError
        ? { message: decisionError.message, title: "KYC decision failed", tone: decisionError.status === 403 ? "warn" : "error" }
        : null;

  return (
    <ProductShell
      activeNav="admin"
      breadcrumbs={[{ label: "Admin", href: "/admin" }, { label: "KYC review" }]}
      description="Review learner identity submissions, inspect evidence references, and decide verified or rejected outcomes."
      eyebrow="Platform admin"
      isSignedIn={route.hasToken}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <>
          <StatusPill label={route.loadState === "loading" ? "Resolving session" : canReview ? "KYC review access" : "Review gated"} />
          <StatusPill label={`${queue?.submissions.length ?? 0} pending`} tone={(queue?.submissions.length ?? 0) ? "warn" : "neutral"} />
          <StatusPill label="Permission scoped" tone={canReview ? "good" : "neutral"} />
        </>
      }
      title="KYC review"
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <SessionErrorState error={route.error} /> : null}
      {route.session && !allowed ? <AdminDeniedState workspace={workspace} /> : null}
      {route.session && allowed ? (
        canReview ? (
          <div className={styles.stack}>
            <KycReviewSummaryPanel onRefresh={loadQueue} queue={queue} state={queueState} />
            {queueState === "loading" || queueState === "idle" ? <PanelLoading title="Loading KYC submissions" /> : null}
            {queueState === "error" ? <PanelError error={queueError} onRetry={loadQueue} title="KYC review queue failed" /> : null}
            {queueState === "success" && queue ? (
              <div className={styles.twoColumnWide}>
                <KycReviewQueue onSelect={selectSubmission} selectedSubmissionId={selectedSubmission?.id || null} submissions={queue.submissions} />
                <KycReviewDetail
                  auditError={audit.auditError}
                  auditEvents={audit.auditEvents}
                  auditState={audit.auditState}
                  decisionError={decisionError}
                  decisionState={decisionState}
                  decisionStatus={decisionStatus}
                  onDecisionStatusChange={setDecisionStatus}
                  onRefreshAudit={audit.loadAudit}
                  onRejectionReasonChange={setRejectionReason}
                  onSubmitDecision={submitDecision}
                  rejectionReason={rejectionReason}
                  submission={selectedSubmission}
                />
              </div>
            ) : null}
          </div>
        ) : (
          <GatedPanel capability={getCapability(workspace, "kyc_reviews")} icon={<IdCard size={20} aria-hidden />} title="KYC review unavailable" />
        )
      ) : null}
    </ProductShell>
  );
}
