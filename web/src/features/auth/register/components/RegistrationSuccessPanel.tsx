"use client";

import Link from "next/link";
import styles from "../../shared/auth.module.css";
import { registrationSuccessMessages, registrationSuccessTitle } from "../model/registrationSuccessMessages";

export function RegistrationSuccessPanel() {
  return (
    <div className={styles.successBox} role="status">
      <strong>{registrationSuccessTitle}</strong>
      {registrationSuccessMessages().map((message) => (
        <span key={message}>{message}</span>
      ))}
      <div className={styles.successActions}>
        <Link className={styles.secondaryLink} href="/login">
          Go to login
        </Link>
      </div>
    </div>
  );
}
