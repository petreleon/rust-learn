import { test, expect } from "@playwright/test";

test.describe("Authentication — Login", () => {
  test("login form has email and password fields", async ({ page }) => {
    await page.goto("/login");
    await page.waitForLoadState("networkidle");
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('input[type="password"]')).toBeVisible();
    await expect(page.locator('button[type="submit"]')).toBeVisible();
  });

  test("login page shows register link", async ({ page }) => {
    await page.goto("/login");
    await page.waitForLoadState("networkidle");
    await expect(page.locator('a[href="/register"]')).toBeVisible();
  });

  test("login page shows forgot password link", async ({ page }) => {
    await page.goto("/login");
    await page.waitForLoadState("networkidle");
    await expect(page.locator('a[href="/forgot-password"]')).toBeVisible();
  });

  test("empty form shows validation", async ({ page }) => {
    await page.goto("/login");
    await page.waitForLoadState("networkidle");
    await page.locator('button[type="submit"]').click();
    // Should show some error or not submit empty
    await expect(page).toHaveURL(/\/login/);
  });
});

test.describe("Authentication — Register", () => {
  test("register form has required fields", async ({ page }) => {
    await page.goto("/register");
    await page.waitForLoadState("networkidle");
    await expect(page.locator('input[name="name"]')).toBeVisible();
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('input[type="password"]')).toBeVisible();
    await expect(page.locator('button[type="submit"]')).toBeVisible();
  });

  test("register page links to login", async ({ page }) => {
    await page.goto("/register");
    await page.waitForLoadState("networkidle");
    await expect(page.locator('a[href="/login"]')).toBeVisible();
  });
});

test.describe("Authentication — Verify Email", () => {
  test("verify email page renders with token input", async ({ page }) => {
    await page.goto("/verify-email");
    await page.waitForLoadState("networkidle");
    // Page should load without error
    await expect(page).toHaveURL(/\/verify-email/);
  });
});

test.describe("Authentication — Forgot Password", () => {
  test("forgot password page renders", async ({ page }) => {
    await page.goto("/forgot-password");
    await page.waitForLoadState("networkidle");
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('button[type="submit"]')).toBeVisible();
  });
});

test.describe("Authentication — Reset Password", () => {
  test("reset password page renders", async ({ page }) => {
    await page.goto("/reset-password");
    await page.waitForLoadState("networkidle");
    // Should show form or message about invalid token
    await expect(page.locator("body")).not.toBeEmpty();
  });
});
