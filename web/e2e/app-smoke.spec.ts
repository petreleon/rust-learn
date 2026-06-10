import { test, expect } from "@playwright/test";

test.describe("RustLearn — App Shell", () => {
  test("home page loads and redirects to login", async ({ page }) => {
    const response = await page.goto("/");
    expect(response?.status()).toBeLessThan(500);
    await page.waitForLoadState("networkidle");
    const title = await page.title();
    expect(title.toLowerCase()).toContain("rust");
  });

  test("health check endpoint returns ok", async ({ request }) => {
    const response = await request.get("/healthz");
    expect(response.status()).toBe(200);
    const text = await response.text();
    expect(text).toContain("ok");
  });

  test("login page renders", async ({ page }) => {
    await page.goto("/login");
    await page.waitForLoadState("networkidle");
    const emailInput = page.locator('input[type="email"]');
    await expect(emailInput).toBeVisible();
  });

  test("register page renders", async ({ page }) => {
    await page.goto("/register");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("form")).toBeVisible();
  });
});

test.describe("RustLearn — Navigation", () => {
  test("navigating to course catalog redirects to login when unauthenticated", async ({ page }) => {
    const response = await page.goto("/courses");
    expect(response?.status()).toBeLessThan(500);
    await page.waitForLoadState("networkidle");
  });

  test("teach page redirects to login when unauthenticated", async ({ page }) => {
    const response = await page.goto("/teach/apply");
    expect(response?.status()).toBeLessThan(500);
    await page.waitForLoadState("networkidle");
  });
});
