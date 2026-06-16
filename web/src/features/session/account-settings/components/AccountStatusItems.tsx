"use client";

import { CreditCard, ShieldCheck } from "lucide-react";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type LoadState } from "../model/LoadState";
import { type WalletLoadState } from "../model/WalletLoadState";
import { type RouteError } from "../model/RouteError";
import { type WalletSummary } from "@/lib/learner/WalletSummary";
import { StatusLine } from "./StatusLine";
import { walletStatusLabel } from "../model/walletStatusLabel";

export function AccountStatusItems({
  loadState,
  session,
  wallet,
  walletError,
  walletState,
}: {
  loadState: LoadState;
  session: CurrentSession | null;
  wallet: WalletSummary | null;
  walletError: RouteError | null;
  walletState: WalletLoadState;
}) {
  const walletStatus = walletStatusLabel(wallet, walletState, walletError);
  const sessionLabel = loadState === "loading"
    ? "Resolving account"
    : session
      ? session.user.email_verified
        ? "Email verified"
        : "Email pending"
      : "No active session";

  return (
    <>
      <StatusLine
        icon={<ShieldCheck size={16} aria-hidden />}
        label={sessionLabel}
        tone={session?.user.email_verified ? "good" : loadState === "error" ? "warn" : "neutral"}
      />
      <StatusLine icon={<CreditCard size={16} aria-hidden />} label={walletStatus.label} tone={walletStatus.tone} />
    </>
  );
}
