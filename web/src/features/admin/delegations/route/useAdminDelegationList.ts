"use client";

import { useCallback, useState } from "react";
import { type DelegationItem } from "@/lib/admin/DelegationItem";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadAdminDelegations } from "../api/delegationsApi";
import { normalizeAdminDelegationRouteError } from "./normalizeAdminDelegationRouteError";

export function useAdminDelegationList({
  allowed,
  canView,
  session,
}: {
  allowed: boolean;
  canView: boolean;
  session: CurrentSession | null;
}) {
  const [delegationError, setDelegationError] = useState<RouteError | null>(null);
  const [delegations, setDelegations] = useState<DelegationItem[]>([]);
  const [delegationState, setDelegationState] = useState<LoadState>("idle");
  const [selectedDelegation, setSelectedDelegation] = useState<DelegationItem | null>(null);

  const loadDelegations = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!session || !token || !allowed || !canView) return;

    setDelegationState("loading");
    setDelegationError(null);

    try {
      const response = await loadAdminDelegations({ token });
      setDelegations(response.delegations);
      setDelegationState("success");
    } catch (nextError) {
      setDelegations([]);
      setDelegationError(
        normalizeAdminDelegationRouteError(nextError, "Delegations could not be loaded."),
      );
      setDelegationState("error");
    }
  }, [allowed, canView, session]);

  const prependDelegation = useCallback((delegation: DelegationItem) => {
    setDelegations((current) => [delegation, ...current]);
    setSelectedDelegation(delegation);
  }, []);

  const replaceDelegation = useCallback((delegation: DelegationItem) => {
    setDelegations((current) => current.map((item) => (item.id === delegation.id ? delegation : item)));
    setSelectedDelegation(delegation);
  }, []);

  return {
    delegationError,
    delegations,
    delegationState,
    loadDelegations,
    prependDelegation,
    replaceDelegation,
    selectedDelegation,
    setSelectedDelegation,
  };
}
