"use client";

import { Users } from "lucide-react";
import { type AdminUserProfile } from "@/lib/admin";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { EmptyState } from "@/features/admin/shared/route-kit/EmptyState";
import { StatusPill } from "@/features/admin/shared/route-kit/StatusPill";
import { userVerificationTone } from "../model/userVerificationTone";

export function UserListPanel({
  onSelect,
  selectedUserId,
  state,
  users,
}: {
  onSelect: (userId: number) => void;
  selectedUserId: number | null;
  state: string;
  users: AdminUserProfile[];
}) {
  return (
    <section className={styles.panel}>
      <div className={styles.panelHeader}>
        <Users size={20} aria-hidden />
        <div>
          <h2>User search results</h2>
          <p>Select a user to inspect profile status and assign platform roles.</p>
        </div>
        <StatusPill label={`${users.length} users`} />
      </div>
      {state === "idle" ? <EmptyState text="Search by name or email to load users." /> : null}
      {state === "success" && !users.length ? <EmptyState text="No users matched that search." /> : null}
      {users.length ? (
        <div className={styles.queueList}>
          {users.map((user) => (
            <button
              className={`${styles.queueItem} ${selectedUserId === user.id ? styles.queueItemSelected : ""}`}
              key={user.id}
              onClick={() => onSelect(user.id)}
              type="button"
            >
              <strong>{user.name}</strong>
              <span>{user.email}</span>
              <div className={styles.rowMeta}>
                <StatusPill label={user.email_verified ? "Email verified" : "Email pending"} tone={userVerificationTone(user.email_verified)} />
                <StatusPill label={user.kyc_verified ? "KYC verified" : "KYC pending"} tone={userVerificationTone(user.kyc_verified)} />
              </div>
            </button>
          ))}
        </div>
      ) : null}
    </section>
  );
}
