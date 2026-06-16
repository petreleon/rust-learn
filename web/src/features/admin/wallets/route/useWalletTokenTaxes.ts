"use client";

import { useCallback, useEffect, useState } from "react";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadWalletTokenTaxes, saveWalletTokenTax } from "../api/walletsApi";
import {
  type WalletTokenTaxDraft,
  type WalletTokenTaxOperation,
  type WalletTokenTaxSettings,
} from "../model/WalletTokenTax";
import { normalizeAdminWalletRouteError } from "./normalizeAdminWalletRouteError";

const emptyDraft: WalletTokenTaxDraft = { deposit: "", retire: "" };

export function useWalletTokenTaxes({
  allowed,
  canSetDeposit,
  canSetRetire,
}: {
  allowed: boolean;
  canSetDeposit: boolean;
  canSetRetire: boolean;
}) {
  const canManage = canSetDeposit || canSetRetire;
  const [draft, setDraft] = useState<WalletTokenTaxDraft>(emptyDraft);
  const [error, setError] = useState<RouteError | null>(null);
  const [savingOperation, setSavingOperation] = useState<WalletTokenTaxOperation | null>(null);
  const [settings, setSettings] = useState<WalletTokenTaxSettings | null>(null);
  const [state, setState] = useState<LoadState>("idle");

  const loadTokenTaxes = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token || !allowed || !canManage) return;

    setState("loading");
    setError(null);
    try {
      const nextSettings = await loadWalletTokenTaxes({ token });
      setSettings(nextSettings);
      setDraft({
        deposit: nextSettings.deposit.tax_amount,
        retire: nextSettings.retire.tax_amount,
      });
      setState("success");
    } catch (nextError) {
      setSettings(null);
      setError(normalizeAdminWalletRouteError(nextError, "Wallet token taxes could not be loaded."));
      setState("error");
    }
  }, [allowed, canManage]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadTokenTaxes(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadTokenTaxes]);

  function changeTokenTax(operation: WalletTokenTaxOperation, value: string) {
    setDraft((current) => ({ ...current, [operation]: value }));
  }

  async function saveTokenTax(operation: WalletTokenTaxOperation) {
    const token = readBrowserSessionToken();
    if (!token || !allowed || savingOperation) return;
    if (operation === "deposit" && !canSetDeposit) return;
    if (operation === "retire" && !canSetRetire) return;

    setSavingOperation(operation);
    setError(null);
    try {
      const nextTax = await saveWalletTokenTax({
        operation,
        taxAmount: draft[operation],
        token,
      });
      const fallbackSettings = {
        deposit: { operation: "deposit" as const, tax_amount: draft.deposit },
        retire: { operation: "retire" as const, tax_amount: draft.retire },
      };
      setSettings((current) => ({
        deposit: operation === "deposit" ? nextTax : current?.deposit ?? fallbackSettings.deposit,
        retire: operation === "retire" ? nextTax : current?.retire ?? fallbackSettings.retire,
      }));
      setDraft((current) => ({ ...current, [operation]: nextTax.tax_amount }));
      setState("success");
    } catch (nextError) {
      setError(normalizeAdminWalletRouteError(nextError, "Wallet token tax could not be saved."));
    } finally {
      setSavingOperation(null);
    }
  }

  return {
    changeTokenTax,
    loadTokenTaxes,
    saveTokenTax,
    tokenTaxDraft: draft,
    tokenTaxError: error,
    tokenTaxSavingOperation: savingOperation,
    tokenTaxSettings: settings,
    tokenTaxState: state,
  };
}
