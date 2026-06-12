import { RefreshCw, UserRound } from "lucide-react";
import { type CurrentSession } from "@/lib/session";
import styles from "../page.module.css";
import { StatusLine } from "./StatusLine";

export function ProfilePanel({
  onRefresh,
  session,
}: {
  onRefresh: () => void;
  session: CurrentSession;
}) {
  return (
    <article className={`${styles.panel} ${styles.profilePanel}`}>
      <div className={styles.panelHeader}>
        <UserRound size={22} aria-hidden />
        <h2>Profile</h2>
      </div>
      <div className={styles.profileBlock}>
        <div>
          <p className={styles.profileName}>{session.user.name}</p>
          <p className={styles.muted}>{session.user.email}</p>
        </div>
        <div className={styles.badgeRow}>
          <StatusLine label={session.user.email_verified ? "Email verified" : "Email pending"} tone={session.user.email_verified ? "good" : "warn"} />
          <StatusLine label={session.user.kyc_verified ? "KYC verified" : "KYC pending"} tone={session.user.kyc_verified ? "good" : "neutral"} />
        </div>
        <div className={styles.actionRow}>
          <button className={styles.secondaryButton} type="button" onClick={onRefresh}>
            <RefreshCw size={18} aria-hidden />
            Refresh
          </button>
        </div>
      </div>
    </article>
  );
}
