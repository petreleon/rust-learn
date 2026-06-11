import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminRawRequest } from "./adminRawRequest";
import { filenameFromContentDisposition } from "./filenameFromContentDisposition";
import { platformCsvEndpoints } from "./platformCsvEndpoints";
import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type PlatformCsvDownload } from "./PlatformCsvDownload";
import { type PlatformCsvReport } from "./PlatformCsvReport";

export async function downloadPlatformCsv({
  apiRoot = "/api",
  report,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions & { report: PlatformCsvReport }): Promise<PlatformCsvDownload> {
  const endpoint = platformCsvEndpoints[report];
  const response = await adminRawRequest({
    accept: "text/csv, text/plain",
    apiRoot,
    path: endpoint.path,
    timeoutMs,
    token,
  });
  const body = await response.text();

  return {
    body,
    filename: filenameFromContentDisposition(response.headers.get("content-disposition")) || endpoint.filename,
  };
}
