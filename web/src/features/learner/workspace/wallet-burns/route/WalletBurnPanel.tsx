"use client";

import { WalletBurnView } from "../view/WalletBurnView";
import { useWalletBurns } from "./useWalletBurns";

export function WalletBurnPanel({
  enabled,
  onRefresh,
  walletLinked,
}: {
  enabled: boolean;
  onRefresh: () => void;
  walletLinked: boolean;
}) {
  const burns = useWalletBurns({ canSubmit: enabled && walletLinked, onRefresh, walletLinked });

  return (
    <WalletBurnView
      burns={burns.burns}
      draft={burns.draft}
      enabled={enabled}
      error={burns.error}
      loading={burns.loading}
      submitting={burns.submitting}
      walletLinked={walletLinked}
      onSubmit={burns.submitBurn}
      onUpdateDraft={burns.updateDraft}
    />
  );
}
