"use client";

import { FileCheck2, Loader2, Mail, Send } from "lucide-react";
import Link from "next/link";
import { FormEvent, useEffect, useState } from "react";
import { fetchKycStatus, submitKyc, type KycStatusResponse } from "@/lib/kyc";
import { readStoredSessionToken, type CurrentSession } from "@/lib/session";
import styles from "../page.module.css";
import { ReadinessItem } from "./ReadinessItem";
import { kycReadinessCopy } from "./kycReadiness";

type SaveState = "idle" | "loading" | "saving" | "success" | "error";

export function VerificationPanel({ session }: { session: CurrentSession }) {
  const [countryCode, setCountryCode] = useState("US");
  const [documentLast4, setDocumentLast4] = useState("");
  const [documentType, setDocumentType] = useState("passport");
  const [evidenceReference, setEvidenceReference] = useState("");
  const [legalName, setLegalName] = useState(session.user.name);
  const [message, setMessage] = useState<string | null>(null);
  const [saveState, setSaveState] = useState<SaveState>("loading");
  const [status, setStatus] = useState<KycStatusResponse | null>(null);

  useEffect(() => {
    const token = readStoredSessionToken();
    if (!token) return;
    void fetchKycStatus({ token })
      .then((nextStatus) => {
        setStatus(nextStatus);
        setSaveState("idle");
      })
      .catch((error) => {
        setMessage(error instanceof Error ? error.message : "KYC status could not be loaded.");
        setSaveState("error");
      });
  }, []);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const token = readStoredSessionToken();
    if (!token) return;
    setMessage(null);
    setSaveState("saving");
    try {
      const nextStatus = await submitKyc({
        payload: { country_code: countryCode, document_last4: documentLast4, document_type: documentType, evidence_reference: evidenceReference, legal_name: legalName },
        token,
      });
      setStatus(nextStatus);
      setMessage("KYC submitted for platform review.");
      setSaveState("success");
    } catch (error) {
      setMessage(error instanceof Error ? error.message : "KYC submission failed.");
      setSaveState("error");
    }
  }

  const canSubmit = !session.user.kyc_verified && status?.next_action !== "wait_for_review";
  const readiness = kycReadinessCopy(status, session.user.kyc_verified);

  return (
    <article className={styles.panel}>
      <div className={styles.panelHeader}>
        <Mail size={22} aria-hidden />
        <h2>Verification</h2>
      </div>
      <div className={styles.checkList}>
        <ReadinessItem detail={session.user.email_verified ? "Login, learner routes, and workspace APIs can load normally." : "Verify this address before using protected product routes."} label={session.user.email_verified ? "Email is verified" : "Email verification needed"} tone={session.user.email_verified ? "good" : "warn"} />
        <ReadinessItem detail={readiness.detail} label={readiness.label} tone={readiness.tone} />
      </div>
      {!session.user.email_verified ? <Link className={styles.secondaryLink} href="/verify-email">Open verification</Link> : null}
      {saveState === "loading" ? <p className={styles.muted}><Loader2 className={styles.spin} size={15} aria-hidden /> Loading KYC status</p> : null}
      {canSubmit ? (
        <form className={styles.formGrid} onSubmit={handleSubmit}>
          <label className={styles.fieldLabel}>Legal name<input value={legalName} onChange={(event) => setLegalName(event.target.value)} required /></label>
          <label className={styles.fieldLabel}>Country code<input maxLength={2} value={countryCode} onChange={(event) => setCountryCode(event.target.value.toUpperCase())} required /></label>
          <label className={styles.fieldLabel}>Document type<select value={documentType} onChange={(event) => setDocumentType(event.target.value)}><option value="passport">Passport</option><option value="national_id">National ID</option><option value="driver_license">Driver license</option></select></label>
          <label className={styles.fieldLabel}>Document last digits<input maxLength={16} value={documentLast4} onChange={(event) => setDocumentLast4(event.target.value)} /></label>
          <label className={styles.fieldLabel}>Evidence reference<input value={evidenceReference} onChange={(event) => setEvidenceReference(event.target.value)} placeholder="Optional provider or storage reference" /></label>
          {message ? <p className={styles.muted}>{message}</p> : null}
          <button className={styles.primaryLink} disabled={saveState === "saving"} type="submit"><Send size={17} aria-hidden />{saveState === "saving" ? "Submitting" : "Submit KYC"}</button>
        </form>
      ) : (
        <p className={styles.muted}><FileCheck2 size={16} aria-hidden /> {message || "No KYC action is required from this screen right now."}</p>
      )}
    </article>
  );
}
