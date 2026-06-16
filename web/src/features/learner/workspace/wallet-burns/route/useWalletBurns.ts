"use client";

import { useCallback, useEffect, useState } from "react";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { normalizeRouteError } from "../../components/normalizeRouteError";
import { createMyWalletBurn, fetchMyWalletBurns } from "../api/walletBurnApi";
import { walletBurnDraftIsReady } from "../model/walletBurnPayload";
import {
  defaultWalletBurnDraft,
  type WalletBurnDraft,
  type WalletBurnResult,
} from "../model/walletBurnTypes";

export function useWalletBurns({
  canSubmit,
  onRefresh,
  walletLinked,
}: {
  canSubmit: boolean;
  onRefresh: () => void;
  walletLinked: boolean;
}) {
  const [burns, setBurns] = useState<WalletBurnResult[]>([]);
  const [draft, setDraft] = useState(defaultWalletBurnDraft);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [submitting, setSubmitting] = useState(false);

  const loadBurns = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token || !walletLinked) return;
    setError(null);
    setLoading(true);
    try {
      setBurns(await fetchMyWalletBurns({ token }));
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (requestError.status === 401) clearBrowserSession();
      setError(requestError.message);
    } finally {
      setLoading(false);
    }
  }, [walletLinked]);

  useEffect(() => {
    void loadBurns();
  }, [loadBurns]);

  function updateDraft(patch: Partial<WalletBurnDraft>) {
    setDraft((current) => ({ ...current, ...patch }));
  }

  async function submitBurn() {
    const token = readBrowserSessionToken();
    if (!token || !canSubmit || !walletBurnDraftIsReady(draft)) return;
    setError(null);
    setSubmitting(true);
    try {
      const result = await createMyWalletBurn({
        draft,
        idempotencyKey: burnIdempotencyKey(),
        token,
      });
      setBurns((current) => [result, ...current].slice(0, 8));
      setDraft(defaultWalletBurnDraft());
      onRefresh();
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (requestError.status === 401) clearBrowserSession();
      setError(requestError.message);
    } finally {
      setSubmitting(false);
    }
  }

  return { burns, draft, error, loading, submitBurn, submitting, updateDraft };
}

function burnIdempotencyKey() {
  return `user-burn-${globalThis.crypto?.randomUUID?.() ?? `${Date.now()}-${Math.random()}`}`;
}
