"use client";

import { WalletTransferView } from "../view/WalletTransferView";
import { useWalletTransfers } from "./useWalletTransfers";

export function WalletTransferPanel({
  enabled,
  onRefresh,
  walletLinked,
}: {
  enabled: boolean;
  onRefresh: () => void;
  walletLinked: boolean;
}) {
  const transfers = useWalletTransfers({ onRefresh });

  return (
    <WalletTransferView
      drafts={transfers.drafts}
      enabled={enabled}
      error={transfers.error}
      pendingOperation={transfers.pendingOperation}
      results={transfers.results}
      walletLinked={walletLinked}
      onSubmit={transfers.submitTransfer}
      onUpdateDraft={transfers.updateDraft}
    />
  );
}
