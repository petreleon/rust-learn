"use client";

import { AlertCircle, CreditCard, Mail } from "lucide-react";
import Link from "next/link";
import { type ReactNode } from "react";
import { type WalletSummary } from "@/lib/learner/WalletSummary";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import styles from "@/features/session/account-settings/page.module.css";
import { type RouteError } from "../model/RouteError";

export function accountNextStep(
  session: CurrentSession,
  wallet: WalletSummary | null,
  walletError: RouteError | null,
): {
  actionIcon: ReactNode;
  actionLabel: string;
  detail: string;
  href: string;
  icon: ReactNode;
  title: string;
} | null {
  if (!session.user.email_verified) {
    return {
      actionIcon: <Mail size={18} aria-hidden />,
      actionLabel: "Open verification",
      detail: "Verify your email address before relying on protected account and learner workflows.",
      href: "/verify-email",
      icon: <Mail size={22} aria-hidden />,
      title: "Verify email",
    };
  }

  if (walletError) {
    return {
      actionIcon: <CreditCard size={18} aria-hidden />,
      actionLabel: "Open wallet",
      detail: "Your profile loaded, but RustLearn could not read wallet status. Open the wallet route to retry or continue from there.",
      href: "/wallet",
      icon: <AlertCircle size={22} aria-hidden />,
      title: "Check wallet status",
    };
  }

  if (!wallet) {
    return {
      actionIcon: <CreditCard size={18} aria-hidden />,
      actionLabel: "Link wallet",
      detail: "Link a RustLearn wallet so approved learner rewards have a destination.",
      href: "/wallet",
      icon: <CreditCard size={22} aria-hidden />,
      title: "Link wallet",
    };
  }

  return null;
}

export function AccountNextStep({
  session,
  wallet,
  walletError,
}: {
  session: CurrentSession;
  wallet: WalletSummary | null;
  walletError: RouteError | null;
}) {
  const nextStep = accountNextStep(session, wallet, walletError);
  if (!nextStep) return null;

  return (
    <section className={styles.nextStepPanel}>
      <div className={styles.panelHeader}>
        {nextStep.icon}
        <h2>{nextStep.title}</h2>
      </div>
      <p className={styles.muted}>{nextStep.detail}</p>
      <Link className={styles.primaryLink} href={nextStep.href}>
        {nextStep.actionIcon}
        {nextStep.actionLabel}
      </Link>
    </section>
  );
}
