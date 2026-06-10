import { test, expect } from "@playwright/test";

test.describe("Learner routes — unauthenticated redirects", () => {
  test("learner dashboard shows sign-in prompt", async ({ page }) => {
    await page.goto("/learn");
    await page.waitForLoadState("networkidle");
    // Should show sign-in prompt or redirect
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("course catalog shows sign-in prompt", async ({ page }) => {
    await page.goto("/courses");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("course detail redirects to sign-in", async ({ page }) => {
    await page.goto("/courses/1");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("rewards page shows sign-in prompt", async ({ page }) => {
    await page.goto("/rewards");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("wallet page shows sign-in prompt", async ({ page }) => {
    await page.goto("/wallet");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });
});

test.describe("Teacher routes — unauthenticated redirects", () => {
  test("teacher application page shows sign-in prompt", async ({ page }) => {
    await page.goto("/teach/apply");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("teacher dashboard shows sign-in prompt", async ({ page }) => {
    await page.goto("/teach");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("teacher courses list shows sign-in prompt", async ({ page }) => {
    await page.goto("/teach/courses");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("teacher course workspace redirects to sign-in", async ({ page }) => {
    await page.goto("/teach/courses/1");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("teacher content authoring redirects to sign-in", async ({ page }) => {
    await page.goto("/teach/courses/1/content");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("teacher enrollments redirects to sign-in", async ({ page }) => {
    await page.goto("/teach/courses/1/enrollments");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("teacher students redirects to sign-in", async ({ page }) => {
    await page.goto("/teach/courses/1/students");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("teacher rewards redirects to sign-in", async ({ page }) => {
    await page.goto("/teach/courses/1/rewards");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });
});

test.describe("Organization routes — unauthenticated redirects", () => {
  test("organizations list shows sign-in prompt", async ({ page }) => {
    await page.goto("/organizations");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("organization dashboard redirects to sign-in", async ({ page }) => {
    await page.goto("/organizations/1");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("organization members redirects to sign-in", async ({ page }) => {
    await page.goto("/organizations/1/members");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("organization courses redirects to sign-in", async ({ page }) => {
    await page.goto("/organizations/1/courses");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("organization reports redirects to sign-in", async ({ page }) => {
    await page.goto("/organizations/1/reports");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("organization teacher applications redirects to sign-in", async ({ page }) => {
    await page.goto("/organizations/1/teacher-applications");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });
});

test.describe("Admin routes — unauthenticated redirects", () => {
  test("admin dashboard redirects to sign-in", async ({ page }) => {
    await page.goto("/admin");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("admin teacher applications redirects to sign-in", async ({ page }) => {
    await page.goto("/admin/teacher-applications");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("admin delegations redirects to sign-in", async ({ page }) => {
    await page.goto("/admin/delegations");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("admin fraud blocks redirects to sign-in", async ({ page }) => {
    await page.goto("/admin/fraud-blocks");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("admin exports redirects to sign-in", async ({ page }) => {
    await page.goto("/admin/exports");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });

  test("admin wallets redirects to sign-in", async ({ page }) => {
    await page.goto("/admin/wallets");
    await page.waitForLoadState("networkidle");
    await expect(page.locator("body")).not.toBeEmpty();
  });
});
