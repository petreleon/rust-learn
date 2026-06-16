"use client";

import { type PlatformCsvDownload } from "@/lib/admin";

export function startCsvDownload(csv: PlatformCsvDownload) {
  const blob = new Blob([csv.body], { type: "text/csv;charset=utf-8" });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = csv.filename;
  document.body.append(anchor);
  anchor.click();
  anchor.remove();
  URL.revokeObjectURL(url);
}
