"use client";

import { Send, UserPlus } from "lucide-react";
import styles from "../organization-routes.module.css";
import { type SettingsSaveState } from "./SettingsSaveState";

export function InviteMemberForm({
  email,
  inviteMessage,
  inviteState,
  onEmailChange,
  onRoleChange,
  onSubmit,
  roleName,
}: {
  email: string;
  inviteMessage: string | null;
  inviteState: SettingsSaveState;
  onEmailChange: (value: string) => void;
  onRoleChange: (value: string) => void;
  onSubmit: (event: React.FormEvent) => void;
  roleName: string;
}) {
  return (
    <section className={`${styles.panel} ${styles.singlePanel}`}>
      <div className={styles.panelHeader}>
        <UserPlus size={20} aria-hidden />
        <h2>Add member by email</h2>
      </div>
      <p className={styles.muted}>Enter an email for an existing user to add them to this organization with a role.</p>
      {inviteMessage ? (
        <p className={styles.muted} style={{ color: inviteState === "error" ? "var(--color-warn, #dc2626)" : undefined }}>
          {inviteMessage}
        </p>
      ) : null}
      <form className={styles.authoringForm} onSubmit={onSubmit}>
        <label>
          <span>Email</span>
          <input
            disabled={inviteState === "saving"}
            onChange={(event) => onEmailChange(event.target.value)}
            placeholder="user@example.com"
            type="email"
            value={email}
          />
        </label>
        <label>
          <span>Role (defaults to Student)</span>
          <select
            aria-label="Invite role"
            onChange={(event) => onRoleChange(event.target.value)}
            value={roleName}
          >
            <option value="">Student (default)</option>
            <option value="TEACHER">Teacher</option>
            <option value="MODERATOR">Moderator</option>
            <option value="ADMIN">Admin</option>
          </select>
        </label>
        <button className={styles.primaryButton} disabled={inviteState === "saving"} type="submit">
          <Send size={17} aria-hidden />
          {inviteState === "saving" ? "Adding…" : "Add member"}
        </button>
      </form>
    </section>
  );
}
