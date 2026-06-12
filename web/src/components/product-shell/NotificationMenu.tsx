"use client";

import { Bell, CheckCircle2, Loader2, RefreshCw, Trash2 } from "lucide-react";
import { type SyntheticEvent, useMemo, useState } from "react";
import {
  clearNotifications,
  fetchNotifications,
  markNotificationRead,
  readStoredSessionToken,
  type NotificationItem,
} from "@/lib/session";
import styles from "../product-shell.module.css";
import { closeOtherProductShellMenus } from "./closeOtherProductShellMenus";

type LoadState = "idle" | "loading" | "success" | "error";

function formatDate(value: string) {
  return value.replace("T", " ").slice(0, 16);
}

export function NotificationMenu({ isSignedIn, variant = "icon" }: { isSignedIn: boolean; variant?: "icon" | "row" }) {
  const [error, setError] = useState<string | null>(null);
  const [items, setItems] = useState<NotificationItem[]>([]);
  const [loadState, setLoadState] = useState<LoadState>("idle");
  const [savingId, setSavingId] = useState<number | null>(null);
  const unreadCount = useMemo(() => items.filter((item) => !item.read).length, [items]);
  const summaryClass = variant === "row" ? styles.notificationRowSummary : styles.notificationSummary;

  function handleToggle(event: SyntheticEvent<HTMLDetailsElement>) {
    if (!event.currentTarget.open) return;
    closeOtherProductShellMenus(event.currentTarget);
    void loadNotifications();
  }

  async function loadNotifications() {
    const token = readStoredSessionToken();
    if (!token) {
      setLoadState("error");
      setError("Sign in again to load notifications.");
      return;
    }
    setLoadState("loading");
    setError(null);
    try {
      setItems(await fetchNotifications({ token }));
      setLoadState("success");
    } catch (nextError) {
      setError(nextError instanceof Error ? nextError.message : "Notifications could not be loaded.");
      setLoadState("error");
    }
  }

  async function markRead(item: NotificationItem) {
    const token = readStoredSessionToken();
    if (!token) return;
    setSavingId(item.id);
    setError(null);
    try {
      await markNotificationRead({ notificationId: item.id, token });
      setItems((current) => current.map((entry) => (entry.id === item.id ? { ...entry, read: true } : entry)));
    } catch (nextError) {
      setError(nextError instanceof Error ? nextError.message : "Notification could not be marked read.");
    } finally {
      setSavingId(null);
    }
  }

  async function clearAll() {
    const token = readStoredSessionToken();
    if (!token) return;
    setSavingId(0);
    setError(null);
    try {
      await clearNotifications({ token });
      setItems([]);
      setLoadState("success");
    } catch (nextError) {
      setError(nextError instanceof Error ? nextError.message : "Notifications could not be cleared.");
    } finally {
      setSavingId(null);
    }
  }

  if (!isSignedIn) {
    return (
      <button
        aria-label="Notifications"
        className={variant === "row" ? styles.accountMenuItem : styles.iconButton}
        disabled
        type="button"
      >
        <Bell size={16} aria-hidden />
        {variant === "row" ? "Notifications" : null}
      </button>
    );
  }

  return (
    <details className={styles.notificationMenu} data-product-shell-menu onToggle={handleToggle}>
      <summary aria-label={unreadCount ? `${unreadCount} unread notifications` : "Notifications"} className={summaryClass}>
        <Bell size={variant === "row" ? 16 : 18} aria-hidden />
        {variant === "row" ? <span>Notifications</span> : null}
        {unreadCount ? <strong className={styles.notificationBadge}>{unreadCount}</strong> : null}
      </summary>
      <div className={styles.notificationPanel}>
        <div className={styles.notificationHeader}>
          <div>
            <strong>Notifications</strong>
            <span>{unreadCount ? `${unreadCount} unread` : "All caught up"}</span>
          </div>
          <button className={styles.notificationAction} onClick={() => void loadNotifications()} type="button">
            <RefreshCw size={14} aria-hidden />
            Refresh
          </button>
        </div>
        {loadState === "loading" ? (
          <p className={styles.notificationState}>
            <Loader2 className={styles.spin} size={15} aria-hidden />
            Loading notifications
          </p>
        ) : null}
        {error ? <p className={styles.notificationError}>{error}</p> : null}
        {loadState === "success" && !items.length ? <p className={styles.notificationState}>No notifications yet.</p> : null}
        {items.length ? (
          <ol className={styles.notificationList}>
            {items.map((item) => (
              <li className={item.read ? styles.notificationItem : styles.notificationItemUnread} key={item.id}>
                <div>
                  <strong>{item.title}</strong>
                  <span>{formatDate(item.created_at)}</span>
                  <p>{item.body}</p>
                </div>
                {!item.read ? (
                  <button className={styles.notificationIconAction} disabled={savingId === item.id} onClick={() => void markRead(item)} type="button">
                    <CheckCircle2 size={14} aria-hidden />
                    Mark read
                  </button>
                ) : null}
              </li>
            ))}
          </ol>
        ) : null}
        {items.length ? (
          <button className={styles.notificationDanger} disabled={savingId === 0} onClick={() => void clearAll()} type="button">
            <Trash2 size={14} aria-hidden />
            Clear all
          </button>
        ) : null}
      </div>
    </details>
  );
}
