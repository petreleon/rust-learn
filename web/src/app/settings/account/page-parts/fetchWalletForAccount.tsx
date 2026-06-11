"use client";

import { fetchMyWallet } from "@/lib/learner";
import { normalizeAccountError } from "./normalizeAccountError";
import { type WalletResult } from "./WalletResult";

export async function fetchWalletForAccount(token: string): Promise<WalletResult> {
  try {
    return {
      error: null,
      wallet: await fetchMyWallet({ token }),
    };
  } catch (error) {
    return {
      error: normalizeAccountError(error),
      wallet: null,
    };
  }
}
