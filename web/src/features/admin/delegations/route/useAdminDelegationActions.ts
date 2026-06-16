"use client";

import { useCallback, useState } from "react";
import { type DelegationItem } from "@/lib/admin/DelegationItem";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { grantAdminDelegation, revokeAdminDelegation } from "../api/delegationsApi";
import { type DelegationActionState } from "../model/DelegationActionState";
import {
  defaultDelegationCreateDraft,
  type DelegationCreateDraft,
} from "../model/DelegationCreateDraft";
import { normalizeAdminDelegationRouteError } from "./normalizeAdminDelegationRouteError";

export function useAdminDelegationActions({
  canGrant,
  canRevoke,
  onCreated,
  onRevoked,
}: {
  canGrant: boolean;
  canRevoke: boolean;
  onCreated: (delegation: DelegationItem) => void;
  onRevoked: (delegation: DelegationItem) => void;
}) {
  const [createDraft, setCreateDraft] = useState<DelegationCreateDraft>(defaultDelegationCreateDraft);
  const [createError, setCreateError] = useState<RouteError | null>(null);
  const [createState, setCreateState] = useState<DelegationActionState>("idle");
  const [revokeError, setRevokeError] = useState<RouteError | null>(null);
  const [revokeReason, setRevokeReason] = useState("");
  const [revokeState, setRevokeState] = useState<DelegationActionState>("idle");

  const updateCreateDraft = useCallback((patch: Partial<DelegationCreateDraft>) => {
    setCreateDraft((current) => ({ ...current, ...patch }));
  }, []);

  const handleCreate = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token || !canGrant) return;

    setCreateState("submitting");
    setCreateError(null);

    try {
      const created = await grantAdminDelegation({ ...createDraft, token });
      onCreated(created);
      setCreateState("success");
      setCreateDraft(defaultDelegationCreateDraft);
      window.setTimeout(() => setCreateState("idle"), 2000);
    } catch (nextError) {
      setCreateError(normalizeAdminDelegationRouteError(nextError, "Delegation could not be created."));
      setCreateState("error");
    }
  }, [canGrant, createDraft, onCreated]);

  const handleRevoke = useCallback(
    async (delegationId: number) => {
      const token = readBrowserSessionToken();
      if (!token || !canRevoke) return;

      setRevokeState("submitting");
      setRevokeError(null);

      try {
        const revoked = await revokeAdminDelegation({ delegationId, revokeReason, token });
        onRevoked(revoked);
        setRevokeState("success");
        setRevokeReason("");
        window.setTimeout(() => setRevokeState("idle"), 2000);
      } catch (nextError) {
        setRevokeError(
          normalizeAdminDelegationRouteError(nextError, "Delegation could not be revoked."),
        );
        setRevokeState("error");
      }
    },
    [canRevoke, onRevoked, revokeReason],
  );

  return {
    createDraft,
    createError,
    createState,
    handleCreate,
    handleRevoke,
    revokeError,
    revokeReason,
    revokeState,
    setRevokeReason,
    updateCreateDraft,
  };
}
