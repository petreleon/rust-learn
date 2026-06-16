"use client";

import { Send, UserPlus } from "lucide-react";
import { useState } from "react";
import { nominateTeacherApplication, type OrganizationWorkspaceItem } from "@/lib/organization";
import { readStoredSessionToken } from "@/lib/session";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { type SettingsSaveState } from "./SettingsSaveState";
import { normalizeRouteError } from "./normalizeRouteError";

export function NominationPanel({
  organization,
  onSuccess,
}: {
  organization: OrganizationWorkspaceItem;
  onSuccess: () => void;
}) {
  const canNominate = organization.capabilities.some((capability) => capability.key === "teacher_applications" && capability.enabled);
  const [applicantId, setApplicantId] = useState("");
  const [experienceSummary, setExperienceSummary] = useState("");
  const [nominateState, setNominateState] = useState<SettingsSaveState>("idle");
  const [nominateMessage, setNominateMessage] = useState<string | null>(null);

  if (!canNominate) return null;

  async function handleSubmit(event: React.FormEvent) {
    event.preventDefault();
    const token = readStoredSessionToken();
    const numericApplicantId = Number(applicantId.trim());
    if (!token || !applicantId.trim() || !Number.isFinite(numericApplicantId)) {
      setNominateMessage("Enter a valid applicant user ID.");
      setNominateState("error");
      return;
    }

    setNominateState("saving");
    setNominateMessage(null);
    try {
      await nominateTeacherApplication({
        organizationId: organization.id,
        payload: {
          applicantUserId: numericApplicantId,
          experienceSummary: experienceSummary.trim() || "Nominated by organization.",
        },
        token,
      });
      setApplicantId("");
      setExperienceSummary("");
      setNominateState("success");
      setNominateMessage("Nomination submitted.");
      onSuccess();
    } catch (nextError) {
      const routeError = normalizeRouteError(nextError);
      setNominateMessage(routeError.message);
      setNominateState("error");
    }
  }

  return (
    <section className={`${styles.panel} ${styles.singlePanel}`}>
      <div className={styles.panelHeader}>
        <UserPlus size={20} aria-hidden />
        <h2>Nominate teacher</h2>
      </div>
      <p className={styles.muted}>
        Submit a sponsored teacher application for an existing platform user. A user ID is required; a searchable picker will replace this raw-id form when the user-lookup contract is available.
      </p>
      {nominateMessage ? (
        <p className={styles.muted} style={{ color: nominateState === "error" ? "var(--color-warn, #dc2626)" : undefined }}>
          {nominateMessage}
        </p>
      ) : null}
      <form className={styles.authoringForm} onSubmit={handleSubmit}>
        <label>
          <span>Applicant user ID</span>
          <input
            disabled={nominateState === "saving"}
            onChange={(event) => setApplicantId(event.target.value)}
            placeholder="User ID"
            value={applicantId}
          />
        </label>
        <label>
          <span>Experience summary (optional)</span>
          <textarea
            disabled={nominateState === "saving"}
            onChange={(event) => setExperienceSummary(event.target.value)}
            placeholder="Teaching and subject experience"
            rows={3}
            value={experienceSummary}
          />
        </label>
        <button className={styles.primaryButton} disabled={nominateState === "saving"} type="submit">
          <Send size={17} aria-hidden />
          {nominateState === "saving" ? "Submitting…" : "Submit nomination"}
        </button>
      </form>
    </section>
  );
}
