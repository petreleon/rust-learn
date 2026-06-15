import { afterEach, describe, expect, it, vi } from "vitest";
import { requestEmailVerification } from "@/lib/auth";

function mockText(body: string, status = 200) {
  const fetchMock = vi.fn(async () => new Response(body, { status }));
  vi.stubGlobal("fetch", fetchMock);
  return fetchMock;
}

describe("email verification helpers", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("requests a new verification email with generic account copy", async () => {
    const fetchMock = mockText("If an unverified account matches that email, a verification link has been sent.");

    const message = await requestEmailVerification({
      apiRoot: "http://api.test",
      email: "learner@example.com",
    });

    expect(message).toContain("If an unverified account matches");
    expect(fetchMock).toHaveBeenCalledWith(
      "http://api.test/auth/resend-verification",
      expect.objectContaining({
        body: JSON.stringify({ email: "learner@example.com" }),
        method: "POST",
      }),
    );
  });

  it("maps missing email to a validation error", async () => {
    mockText("Email is required", 400);

    await expect(
      requestEmailVerification({ apiRoot: "http://api.test", email: "" }),
    ).rejects.toMatchObject({
      code: "missing_email",
      message: "Email is required",
      status: 400,
    });
  });
});
