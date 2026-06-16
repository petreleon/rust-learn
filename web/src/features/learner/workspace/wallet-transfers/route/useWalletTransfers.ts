"use client";

import { useState } from "react";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { normalizeRouteError } from "../../components/normalizeRouteError";
import { createWalletTransfer } from "../api/walletTransferApi";
import { walletTransferDraftIsReady } from "../model/walletTransferPayload";
import {
  defaultWalletTransferDraft,
  type WalletTransferDraft,
  type WalletTransferOperation,
  type WalletTransferResult,
} from "../model/walletTransferTypes";

export function useWalletTransfers({ onRefresh }: { onRefresh: () => void }) {
  const [drafts, setDrafts] = useState<Record<WalletTransferOperation, WalletTransferDraft>>({
    deposit: defaultWalletTransferDraft(),
    retire: defaultWalletTransferDraft(),
  });
  const [error, setError] = useState<string | null>(null);
  const [pendingOperation, setPendingOperation] = useState<WalletTransferOperation | null>(null);
  const [results, setResults] = useState<WalletTransferResult[]>([]);

  function updateDraft(operation: WalletTransferOperation, patch: Partial<WalletTransferDraft>) {
    setDrafts((current) => ({
      ...current,
      [operation]: {
        ...current[operation],
        ...patch,
      },
    }));
  }

  async function submitTransfer(operation: WalletTransferOperation) {
    const token = readBrowserSessionToken();
    const draft = drafts[operation];
    if (!token || !walletTransferDraftIsReady(draft)) return;

    setPendingOperation(operation);
    setError(null);
    try {
      const result = await createWalletTransfer({ draft, operation, token });
      setResults((current) => [result, ...current].slice(0, 5));
      setDrafts((current) => ({ ...current, [operation]: defaultWalletTransferDraft() }));
      onRefresh();
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (requestError.status === 401) clearBrowserSession();
      setError(requestError.message);
    } finally {
      setPendingOperation(null);
    }
  }

  return {
    drafts,
    error,
    pendingOperation,
    results,
    submitTransfer,
    updateDraft,
  };
}
