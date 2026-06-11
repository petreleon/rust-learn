"use client";

import { type WalletSummary } from "@/lib/learner";
import { type RouteError } from "./RouteError";
import { type WalletLoadState } from "./WalletLoadState";

export function walletStatusLabel(
  wallet: WalletSummary | null,
  state: WalletLoadState,
  error: RouteError | null,
): { label: string; tone: "good" | "neutral" | "warn" } {
  if (state === "loading") {
    return { label: "Wallet loading", tone: "neutral" };
  }
  if (state === "idle") {
    return { label: "Wallet not loaded", tone: "neutral" };
  }
  if (error) {
    return { label: "Wallet unavailable", tone: "warn" };
  }
  if (wallet) {
    return { label: "Wallet linked", tone: "good" };
  }
  return { label: "Wallet unlinked", tone: "warn" };
}
