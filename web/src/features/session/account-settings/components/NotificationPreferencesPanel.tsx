import { Bell, Send } from "lucide-react";
import styles from "@/app/settings/account/page.module.css";
import { PreferenceRow } from "./PreferenceRow";
import { notificationDescription } from "../model/notificationDescription";
import { type PrefsSaveState } from "../model/PrefsSaveState";

export function NotificationPreferencesPanel({
  emailEnabled,
  message,
  onEmailChange,
  onPushChange,
  onSave,
  prefsLoaded,
  pushEnabled,
  saveState,
}: {
  emailEnabled: boolean;
  message: string | null;
  onEmailChange: (value: boolean) => void;
  onPushChange: (value: boolean) => void;
  onSave: () => void;
  prefsLoaded: boolean;
  pushEnabled: boolean;
  saveState: PrefsSaveState;
}) {
  return (
    <section className={styles.preferencePanel}>
      <div className={styles.panelHeader}>
        <Bell size={22} aria-hidden />
        <h2>Notification preferences</h2>
      </div>
      <p className={styles.muted}>
        Choose which notifications RustLearn can send. Preferences are saved to your account.
      </p>
      {!prefsLoaded ? <p className={styles.muted}>Defaults are shown until saved preferences load.</p> : null}
      <div className={styles.preferenceList}>
        <PreferenceRow checked={emailEnabled} detail={notificationDescription.email_enabled} disabled={saveState === "saving"} label="Account and access updates" onChange={onEmailChange} />
        <PreferenceRow checked={pushEnabled} detail={notificationDescription.push_enabled} disabled={saveState === "saving"} label="Reward status updates" onChange={onPushChange} />
      </div>
      {message ? <p className={styles.muted}>{message}</p> : null}
      <button className={styles.primaryLink} disabled={saveState === "saving"} onClick={onSave} type="button">
        <Send size={17} aria-hidden />
        {saveState === "saving" ? "Saving..." : "Save preferences"}
      </button>
    </section>
  );
}
