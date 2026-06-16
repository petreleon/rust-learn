"use client";

import { useCallback, useState } from "react";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { createAdminFraudBlock, revokeAdminFraudBlock } from "../api/fraudBlocksApi";
import { type FraudBlockActionState } from "../model/FraudBlockActionState";
import {
  defaultFraudBlockCreateDraft,
  type FraudBlockCreateDraft,
} from "../model/FraudBlockCreateDraft";
import { normalizeAdminFraudBlockRouteError } from "./normalizeAdminFraudBlockRouteError";

export function useAdminFraudBlockActions({
  canCreate,
  canRevoke,
  onBlocksChanged,
  onSelectedRevoked,
  selectedBlockId,
}: {
  canCreate: boolean;
  canRevoke: boolean;
  onBlocksChanged: () => Promise<void>;
  onSelectedRevoked: (blockId: number) => Promise<void>;
  selectedBlockId: number | null;
}) {
  const [createDraft, setCreateDraft] = useState<FraudBlockCreateDraft>(defaultFraudBlockCreateDraft);
  const [createError, setCreateError] = useState<RouteError | null>(null);
  const [createState, setCreateState] = useState<FraudBlockActionState>("idle");
  const [revokeError, setRevokeError] = useState<RouteError | null>(null);
  const [revokeState, setRevokeState] = useState<FraudBlockActionState>("idle");

  const updateCreateDraft = useCallback((patch: Partial<FraudBlockCreateDraft>) => {
    setCreateDraft((current) => ({ ...current, ...patch }));
  }, []);

  const handleCreate = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token || !canCreate) return;

    const validationError = validateCreateDraft(createDraft);
    if (validationError) {
      setCreateError(validationError);
      setCreateState("error");
      return;
    }

    setCreateError(null);
    setCreateState("submitting");

    try {
      await createAdminFraudBlock({ ...createDraft, token });
      setCreateState("success");
      setCreateDraft(defaultFraudBlockCreateDraft);
      await onBlocksChanged();
    } catch (nextError) {
      setCreateError(
        normalizeAdminFraudBlockRouteError(nextError, "Fraud block could not be created."),
      );
      setCreateState("error");
    }
  }, [canCreate, createDraft, onBlocksChanged]);

  const handleRevoke = useCallback(
    async (blockId: number) => {
      const token = readBrowserSessionToken();
      if (!token || !canRevoke) return;

      setRevokeError(null);
      setRevokeState("submitting");

      try {
        await revokeAdminFraudBlock({ blockId, token });
        setRevokeState("success");
        await onBlocksChanged();
        if (selectedBlockId === blockId) {
          await onSelectedRevoked(blockId);
        }
      } catch (nextError) {
        setRevokeError(
          normalizeAdminFraudBlockRouteError(nextError, "Fraud block could not be revoked."),
        );
        setRevokeState("error");
      }
    },
    [canRevoke, onBlocksChanged, onSelectedRevoked, selectedBlockId],
  );

  return {
    createDraft,
    createError,
    createState,
    handleCreate,
    handleRevoke,
    revokeError,
    revokeState,
    updateCreateDraft,
  };
}

function validateCreateDraft(draft: FraudBlockCreateDraft): RouteError | null {
  if (!draft.reason.trim()) {
    return { code: "validation_error", message: "A reason is required.", status: 400 };
  }
  if (!draft.scopeType) {
    return { code: "validation_error", message: "Select a scope type.", status: 400 };
  }
  return null;
}
