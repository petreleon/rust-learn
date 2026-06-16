"use client";

import { FileText, UserCheck } from "lucide-react";
import { type PlatformTeacherApplicationItem } from "@/lib/admin/PlatformTeacherApplicationItem";
import { type TeacherApplicationAuditEvent } from "@/lib/admin/TeacherApplicationAuditEvent";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import styles from "@/components/admin-routes.module.css";
import { ContextRow } from "@/components/admin-routes/ContextRow";
import { formatDate } from "@/components/admin-routes/formatDate";
import { formatUnderscoreLabel } from "@/components/admin-routes/formatUnderscoreLabel";
import { StatusPill } from "@/components/admin-routes/StatusPill";
import { type TeacherApplicationDecisionDraft } from "../model/TeacherApplicationDecisionDraft";
import { type TeacherApplicationDecisionState } from "../model/TeacherApplicationDecisionState";
import {
  teacherApplicationScopeTargetLabel,
  teacherApplicationStatusTone,
} from "../model/teacherApplicationDisplay";
import { TeacherApplicationAuditPanel } from "./TeacherApplicationAuditPanel";
import { TeacherApplicationDecisionForm } from "./TeacherApplicationDecisionForm";

export function TeacherApplicationDetailPanel({
  application,
  auditError,
  auditEvents,
  auditState,
  canApprove,
  canReject,
  decisionDraft,
  decisionError,
  decisionState,
  onDecisionDraftChange,
  onRefreshAudit,
  onSubmitDecision,
}: {
  application: PlatformTeacherApplicationItem | null;
  auditError: RouteError | null;
  auditEvents: TeacherApplicationAuditEvent[];
  auditState: LoadState;
  canApprove: boolean;
  canReject: boolean;
  decisionDraft: TeacherApplicationDecisionDraft;
  decisionError: RouteError | null;
  decisionState: TeacherApplicationDecisionState;
  onDecisionDraftChange: (patch: Partial<TeacherApplicationDecisionDraft>) => void;
  onRefreshAudit: () => void;
  onSubmitDecision: () => void;
}) {
  if (!application) return <EmptyTeacherApplicationDetail />;

  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <FileText size={20} aria-hidden />
        <div>
          <h2>{application.applicant.name}</h2>
          <p>{application.applicant.email}</p>
        </div>
        <StatusPill
          label={formatUnderscoreLabel(application.status)}
          tone={teacherApplicationStatusTone(application.status)}
        />
      </div>
      <TeacherApplicationContext application={application} />
      <TeacherApplicationNarrative application={application} />
      <TeacherApplicationAuditPanel
        error={auditError}
        events={auditEvents}
        onRefresh={onRefreshAudit}
        state={auditState}
      />
      <TeacherApplicationDecisionForm
        application={application}
        canApprove={canApprove}
        canReject={canReject}
        draft={decisionDraft}
        error={decisionError}
        onDraftChange={onDecisionDraftChange}
        onSubmit={onSubmitDecision}
        state={decisionState}
      />
    </section>
  );
}

function EmptyTeacherApplicationDetail() {
  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <UserCheck size={20} aria-hidden />
        <div>
          <h2>Application detail</h2>
          <p>Select an application from the queue to inspect context and audit history.</p>
        </div>
      </div>
    </section>
  );
}

function TeacherApplicationContext({ application }: { application: PlatformTeacherApplicationItem }) {
  return (
    <div className={styles.detailGrid}>
      <ContextRow label="Requested scope" value={formatUnderscoreLabel(application.requested_scope)} />
      <ContextRow label="Target" value={teacherApplicationScopeTargetLabel(application)} />
      <ContextRow label="Sponsor" value={application.sponsor_organization?.name || "No sponsor recorded"} />
      <ContextRow label="Submitted" value={formatDate(application.created_at)} />
      <ContextRow label="Latest update" value={formatDate(application.updated_at)} />
      <ContextRow label="Reviewer" value={application.reviewer?.name || "No reviewer yet"} />
    </div>
  );
}

function TeacherApplicationNarrative({ application }: { application: PlatformTeacherApplicationItem }) {
  return (
    <>
      <div className={styles.textBlock}>
        <h3>Experience summary</h3>
        <p>{application.experience_summary}</p>
      </div>
      <div className={styles.textBlock}>
        <h3>Portfolio</h3>
        {application.portfolio_links.length ? (
          <div className={styles.linkList}>
            {application.portfolio_links.map((link) => (
              <a href={link} key={link} rel="noreferrer" target="_blank">
                {link}
              </a>
            ))}
          </div>
        ) : (
          <p className={styles.muted}>No portfolio links were submitted.</p>
        )}
      </div>
      {application.decision_reason ? (
        <div className={styles.inlineNotice}>
          <strong>Latest decision note</strong>
          <span>{application.decision_reason}</span>
        </div>
      ) : null}
    </>
  );
}
