"use client";

import { OrganizationBurnView } from "../view/OrganizationBurnView";
import { useOrganizationBurns } from "./useOrganizationBurns";

export function OrganizationBurnPanel({
  onRefresh,
  organizationId,
  walletLinked,
}: {
  onRefresh: () => void;
  organizationId: number;
  walletLinked: boolean;
}) {
  const burns = useOrganizationBurns({ enabled: walletLinked, onRefresh, organizationId });

  return (
    <OrganizationBurnView
      burns={burns.burns}
      draft={burns.draft}
      enabled={walletLinked}
      error={burns.error}
      loading={burns.loading}
      permissions={burns.permissions}
      submitting={burns.submitting}
      onSubmit={burns.submitBurn}
      onUpdateDraft={burns.updateDraft}
    />
  );
}
