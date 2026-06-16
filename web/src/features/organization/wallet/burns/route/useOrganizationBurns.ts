"use client";

import { useCallback, useEffect, useState } from "react";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { normalizeRouteError } from "@/features/organization/shared/route-kit/normalizeRouteError";
import {
  createOrganizationTokenBurn,
  fetchOrganizationBurnPermissions,
  fetchOrganizationTokenBurns,
} from "../api/organizationBurnApi";
import { organizationBurnDraftIsReady } from "../model/organizationBurnPayload";
import {
  defaultOrganizationBurnDraft,
  type OrganizationBurnDraft,
  type OrganizationBurnPermissions,
  type OrganizationTokenBurn,
} from "../model/organizationBurnTypes";

export function useOrganizationBurns({
  enabled,
  onRefresh,
  organizationId,
}: {
  enabled: boolean;
  onRefresh: () => void;
  organizationId: number;
}) {
  const [burns, setBurns] = useState<OrganizationTokenBurn[]>([]);
  const [draft, setDraft] = useState(defaultOrganizationBurnDraft);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [permissions, setPermissions] = useState<OrganizationBurnPermissions | null>(null);
  const [submitting, setSubmitting] = useState(false);

  const loadBurns = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!token || !enabled) return;
    setError(null);
    setLoading(true);
    try {
      const [nextPermissions, nextBurns] = await Promise.all([
        fetchOrganizationBurnPermissions({ organizationId, token }),
        fetchOrganizationTokenBurns({ organizationId, token }),
      ]);
      setPermissions(nextPermissions);
      setBurns(nextBurns);
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (requestError.status === 401) clearBrowserSession();
      setError(requestError.message);
    } finally {
      setLoading(false);
    }
  }, [enabled, organizationId]);

  useEffect(() => {
    void loadBurns();
  }, [loadBurns]);

  function updateDraft(patch: Partial<OrganizationBurnDraft>) {
    setDraft((current) => ({ ...current, ...patch }));
  }

  async function submitBurn() {
    const token = readBrowserSessionToken();
    if (!token || !permissions?.can_request_burn || !organizationBurnDraftIsReady(draft)) return;
    setError(null);
    setSubmitting(true);
    try {
      const result = await createOrganizationTokenBurn({
        draft,
        idempotencyKey: burnIdempotencyKey(organizationId),
        organizationId,
        token,
      });
      setBurns((current) => [result, ...current].slice(0, 8));
      setDraft(defaultOrganizationBurnDraft());
      onRefresh();
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (requestError.status === 401) clearBrowserSession();
      setError(requestError.message);
    } finally {
      setSubmitting(false);
    }
  }

  return { burns, draft, error, loading, permissions, submitBurn, submitting, updateDraft };
}

function burnIdempotencyKey(organizationId: number) {
  const random = globalThis.crypto?.randomUUID?.() ?? `${Date.now()}-${Math.random()}`;
  return `org-${organizationId}-burn-${random}`;
}
