"use client";

import { useCallback, useMemo, useState } from "react";
import { type FraudBlockItem } from "@/lib/admin/FraudBlockItem";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type LoadState } from "@/shared/route-state/LoadState";
import { type RouteError } from "@/shared/route-state/RouteError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadAdminFraudBlocks, type FraudBlockListResult } from "../api/fraudBlocksApi";
import { defaultFraudBlockFilters, type FraudBlockFilters } from "../model/FraudBlockFilters";
import { normalizeAdminFraudBlockRouteError } from "./normalizeAdminFraudBlockRouteError";

export function useAdminFraudBlockList({
  allowed,
  canView,
  session,
}: {
  allowed: boolean;
  canView: boolean;
  session: CurrentSession | null;
}) {
  const [blocks, setBlocks] = useState<FraudBlockListResult | null>(null);
  const [blocksError, setBlocksError] = useState<RouteError | null>(null);
  const [blocksState, setBlocksState] = useState<LoadState>("idle");
  const [filters, setFilters] = useState<FraudBlockFilters>(defaultFraudBlockFilters);
  const [selectedBlockId, setSelectedBlockId] = useState<number | null>(null);

  const selectedBlock = useMemo(
    () => blocks?.blocks.find((block) => block.id === selectedBlockId) || blocks?.blocks[0] || null,
    [blocks, selectedBlockId],
  );

  const loadBlocks = useCallback(async () => {
    const token = readBrowserSessionToken();
    if (!session || !token || !allowed || !canView) return;

    setBlocksState("loading");
    setBlocksError(null);

    try {
      const response = await loadAdminFraudBlocks({
        active: filters.active,
        offset: filters.offset,
        scopeType: filters.scopeType,
        token,
      });
      setBlocks(response);
      setBlocksState("success");
      setSelectedBlockId((current) => nextSelectedBlockId(current, response.blocks));
    } catch (nextError) {
      setBlocks(null);
      setBlocksError(
        normalizeAdminFraudBlockRouteError(nextError, "Fraud blocks could not be loaded."),
      );
      setBlocksState("error");
    }
  }, [allowed, canView, filters.active, filters.offset, filters.scopeType, session]);

  const applyFilters = useCallback(() => {
    setFilters((current) => ({ ...current, offset: 0 }));
  }, []);

  const resetFilters = useCallback(() => {
    setFilters(defaultFraudBlockFilters);
  }, []);

  const updateFilters = useCallback((patch: Partial<FraudBlockFilters>) => {
    setFilters((current) => ({ ...current, ...patch }));
  }, []);

  const setPageOffset = useCallback((offset: number) => {
    setFilters((current) => ({ ...current, offset: Math.max(0, offset) }));
  }, []);

  return {
    applyFilters,
    blocks,
    blocksError,
    blocksState,
    filters,
    loadBlocks,
    resetFilters,
    selectedBlock,
    selectedBlockId,
    setPageOffset,
    setSelectedBlockId,
    updateFilters,
  };
}

function nextSelectedBlockId(current: number | null, blocks: FraudBlockItem[]) {
  if (current && blocks.some((block) => block.id === current)) return current;
  return blocks[0]?.id || null;
}
