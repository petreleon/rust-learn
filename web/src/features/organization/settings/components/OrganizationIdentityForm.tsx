"use client";

import { type FormEvent } from "react";
import { RefreshCw, Send, Settings } from "lucide-react";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { type SettingsSaveState } from "../model/SettingsSaveState";

export function OrganizationIdentityForm({
  nameDraft,
  onNameDraftChange,
  onProfileUrlDraftChange,
  onRefresh,
  onSave,
  onWebsiteDraftChange,
  profileUrlDraft,
  saveState,
  websiteDraft,
}: {
  nameDraft: string;
  onNameDraftChange: (value: string) => void;
  onProfileUrlDraftChange: (value: string) => void;
  onRefresh: () => void;
  onSave: (event: FormEvent) => void;
  onWebsiteDraftChange: (value: string) => void;
  profileUrlDraft: string;
  saveState: SettingsSaveState;
  websiteDraft: string;
}) {
  const isSaving = saveState === "saving";

  return (
    <form className={`${styles.panel} ${styles.singlePanel}`} onSubmit={onSave}>
      <div className={styles.panelHeader}>
        <Settings size={20} aria-hidden />
        <h2>Organization identity</h2>
      </div>
      <div className={styles.authoringForm}>
        <label>
          <span>Name</span>
          <input disabled={isSaving} maxLength={120} onChange={(event) => onNameDraftChange(event.target.value)} placeholder="Organization name" required value={nameDraft} />
        </label>
        <label>
          <span>Website</span>
          <input disabled={isSaving} maxLength={240} onChange={(event) => onWebsiteDraftChange(event.target.value)} placeholder="https://example.com" value={websiteDraft} />
        </label>
        <label>
          <span>Profile image URL</span>
          <input disabled={isSaving} maxLength={240} onChange={(event) => onProfileUrlDraftChange(event.target.value)} placeholder="https://example.com/logo.png" value={profileUrlDraft} />
        </label>
        <div style={{ display: "flex", gap: "0.75rem", marginTop: "0.5rem" }}>
          <button className={styles.primaryButton} disabled={isSaving} type="submit">
            <Send size={17} aria-hidden />
            {isSaving ? "Saving..." : "Save settings"}
          </button>
          <button className={styles.secondaryButton} disabled={isSaving} onClick={onRefresh} type="button">
            <RefreshCw size={17} aria-hidden />
            Refresh
          </button>
        </div>
      </div>
    </form>
  );
}
