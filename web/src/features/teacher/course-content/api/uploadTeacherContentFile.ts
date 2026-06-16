import { TeacherRequestError } from "@/lib/teacher";

export function uploadTeacherContentFile({
  file,
  onProgress,
  uploadUrl,
}: {
  file: File;
  onProgress: (progress: number) => void;
  uploadUrl: string;
}): Promise<void> {
  return new Promise((resolve, reject) => {
    const request = new XMLHttpRequest();
    request.open("PUT", uploadUrl);
    request.upload.onprogress = (event) => {
      if (event.lengthComputable && event.total > 0) {
        onProgress(Math.round((event.loaded / event.total) * 100));
      }
    };
    request.onload = () => {
      if (request.status >= 200 && request.status < 300) {
        onProgress(100);
        resolve();
        return;
      }
      reject(uploadFailedError(request.status, request.statusText));
    };
    request.onerror = () => {
      reject(new TeacherRequestError("Upload failed because the network request did not complete.", 0, "network_error"));
    };
    onProgress(0);
    request.send(file);
  });
}

function uploadFailedError(status: number, statusText: string) {
  if (status === 401 || status === 403) {
    return new TeacherRequestError(
      "Upload URL expired or was rejected. Select Upload content again to request a fresh URL.",
      status,
      "upload_url_expired",
    );
  }

  return new TeacherRequestError(
    `Upload failed: ${status} ${statusText}`.trim(),
    status,
    "upload_failed",
  );
}
