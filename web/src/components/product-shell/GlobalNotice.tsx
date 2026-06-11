"use client";

import Link from "next/link";
import styles from "../product-shell.module.css";
import { type ShellNotice } from "./ShellNotice";

export function GlobalNotice({ notice }: { notice: ShellNotice }) {
  return (
    <section
      aria-live={notice.tone === "error" ? "assertive" : "polite"}
      className={`${styles.globalNotice} ${styles[notice.tone]}`}
      role="status"
    >
      <div>
        <strong>{notice.title}</strong>
        <span>{notice.message}</span>
      </div>
      {notice.actionHref && notice.actionLabel ? (
        <Link className={styles.noticeAction} href={notice.actionHref}>
          {notice.actionLabel}
        </Link>
      ) : null}
    </section>
  );
}
