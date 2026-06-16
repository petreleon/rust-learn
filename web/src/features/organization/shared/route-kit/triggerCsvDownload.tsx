"use client";
export function triggerCsvDownload(body: string, filename: string) {
  if (typeof window === "undefined") {
    return;
  }

  const blob = new Blob([body], { type: "text/csv;charset=utf-8" });
  const url = window.URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  link.remove();
  window.URL.revokeObjectURL(url);
}
