"use client";

import styles from "../ops-console.module.css";
import { type OpsConsoleController } from "../route/useOpsConsoleController";

export function ResultPanel({ auth }: { auth: OpsConsoleController["auth"] }) {
  const resultTone = auth.result.status === "Pending" ? "pending" : auth.result.ok ? "success" : "error";
  const resultToneClass =
    resultTone === "pending" ? styles.checking : resultTone === "success" ? styles.online : styles.offline;
  const resultToneLabel = resultTone === "pending" ? "Pending" : resultTone === "success" ? "OK" : "Error";
  const resultOutcome =
    resultTone === "pending" ? "Request pending" : resultTone === "success" ? "Request succeeded" : "Request failed";
  const resultAnnouncement = [auth.result.label, auth.result.status, auth.result.status === resultOutcome ? undefined : resultOutcome]
    .filter(Boolean)
    .join(". ")
    .concat(".");

  return (
    <section id="result-panel" className={styles.resultPanel} aria-labelledby="result-title">
      <p className={styles.visuallyHidden} aria-live="polite" aria-atomic="true">
        {resultAnnouncement}
      </p>
      <div className={styles.panelHeader}>
        <div>
          <p className={styles.eyebrow}>{auth.result.status}</p>
          <h2 id="result-title">{auth.result.label}</h2>
        </div>
        <span className={`${styles.statusPill} ${resultToneClass}`}>{resultToneLabel}</span>
      </div>
      <pre aria-label="API response body" tabIndex={0}>
        {auth.result.body}
      </pre>
    </section>
  );
}
