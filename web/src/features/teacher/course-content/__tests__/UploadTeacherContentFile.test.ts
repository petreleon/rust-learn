import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { uploadTeacherContentFile } from "../api/uploadTeacherContentFile";

class MockXmlHttpRequest {
  static nextStatus = 204;
  static nextStatusText = "No Content";
  static requests: MockXmlHttpRequest[] = [];

  onerror: (() => void) | null = null;
  onload: (() => void) | null = null;
  status = MockXmlHttpRequest.nextStatus;
  statusText = MockXmlHttpRequest.nextStatusText;
  upload: { onprogress: ((event: ProgressEvent) => void) | null } = { onprogress: null };

  constructor() {
    MockXmlHttpRequest.requests.push(this);
  }

  open = vi.fn();

  send = vi.fn(() => {
    this.onload?.();
  });
}

describe("uploadTeacherContentFile", () => {
  beforeEach(() => {
    MockXmlHttpRequest.nextStatus = 204;
    MockXmlHttpRequest.nextStatusText = "No Content";
    MockXmlHttpRequest.requests = [];
    vi.stubGlobal("XMLHttpRequest", MockXmlHttpRequest);
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("reports initial and completed upload progress", async () => {
    const onProgress = vi.fn();

    await uploadTeacherContentFile({
      file: new File(["video"], "intro.mp4", { type: "video/mp4" }),
      onProgress,
      uploadUrl: "https://upload.example.test/intro.mp4",
    });

    expect(onProgress).toHaveBeenCalledWith(0);
    expect(onProgress).toHaveBeenCalledWith(100);
    expect(MockXmlHttpRequest.requests[0].open).toHaveBeenCalledWith(
      "PUT",
      "https://upload.example.test/intro.mp4",
    );
  });

  it("maps expired presigned URLs to actionable route errors", async () => {
    MockXmlHttpRequest.nextStatus = 403;
    MockXmlHttpRequest.nextStatusText = "Forbidden";

    await expect(
      uploadTeacherContentFile({
        file: new File(["video"], "intro.mp4", { type: "video/mp4" }),
        onProgress: vi.fn(),
        uploadUrl: "https://upload.example.test/intro.mp4",
      }),
    ).rejects.toMatchObject({
      code: "upload_url_expired",
      message: "Upload URL expired or was rejected. Select Upload content again to request a fresh URL.",
      status: 403,
    });
  });
});
