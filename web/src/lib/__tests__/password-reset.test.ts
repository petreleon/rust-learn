import { afterEach, describe, expect, it, vi } from "vitest";
import { AuthRequestError, requestPasswordReset, resetPassword } from "@/lib/auth";

function mockText(body: string, status = 200) {
  const fetchMock = vi.fn(async () => new Response(body, { status }));
  vi.stubGlobal("fetch", fetchMock);
  return fetchMock;
}

describe("password reset helpers", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("requests a reset email without expecting account existence details", async () => {
    const fetchMock = mockText("If an account matches that email, a password reset link has been sent.");

    const message = await requestPasswordReset({
      apiRoot: "http://api.test",
      email: "learner@example.com",
    });

    expect(message).toContain("If an account matches");
    expect(fetchMock).toHaveBeenCalledWith(
      "http://api.test/auth/forgot-password",
      expect.objectContaining({
        body: JSON.stringify({ email: "learner@example.com" }),
        method: "POST",
      }),
    );
  });

  it("submits a reset token and new password", async () => {
    const fetchMock = mockText("Password updated successfully");

    const message = await resetPassword({
      apiRoot: "http://api.test",
      password: "BetterPass123!",
      token: "reset-token",
    });

    expect(message).toBe("Password updated successfully");
    expect(fetchMock).toHaveBeenCalledWith(
      "http://api.test/auth/reset-password",
      expect.objectContaining({
        body: JSON.stringify({ password: "BetterPass123!", token: "reset-token" }),
        method: "POST",
      }),
    );
  });

  it("maps invalid reset tokens distinctly from password-policy errors", async () => {
    mockText("Invalid password reset token", 400);

    await expect(
      resetPassword({ apiRoot: "http://api.test", password: "BetterPass123!", token: "bad" }),
    ).rejects.toMatchObject<Partial<AuthRequestError>>({
      code: "invalid_reset_token",
      message: "Invalid password reset token",
      status: 400,
    });
  });
});
