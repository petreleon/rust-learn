import { expect, test, type Page } from "@playwright/test";

const authRoutes = [
  { path: "/login", heading: "Open your RustLearn workspace" },
  { path: "/register", heading: "Create your RustLearn account" },
  { path: "/forgot-password", heading: "Reset access to your workspace" },
  { path: "/reset-password", heading: "Choose a new password" },
  { path: "/verify-email", heading: "Confirm your RustLearn account" },
];

const protectedRoutes = [
  { path: "/session", heading: "Current session" },
  { path: "/admin", heading: "Platform admin dashboard" },
  { path: "/admin/kyc", heading: "KYC review" },
  { path: "/admin/delegations", heading: "Delegated permissions" },
];

function watchConsole(page: Page) {
  const problems: string[] = [];
  page.on("pageerror", (error) => problems.push(error.message));
  page.on("console", (message) => {
    if (["error", "warning"].includes(message.type())) {
      problems.push(`${message.type()}: ${message.text()}`);
    }
  });
  return problems;
}

async function openRoute(page: Page, path: string) {
  const response = await page.goto(path);
  expect(response?.status() ?? 200).toBeLessThan(500);
  await page.waitForLoadState("domcontentloaded");
}

async function expectNoFrameworkOverlay(page: Page) {
  await expect(
    page.getByText(/Application error|Runtime Error|Unhandled Runtime Error|not_found/i),
  ).toHaveCount(0);
}

async function expectNoHorizontalOverflow(page: Page) {
  const overflow = await page.evaluate(() => {
    const bodyWidth = document.body?.scrollWidth ?? 0;
    return Math.max(document.documentElement.scrollWidth, bodyWidth) - window.innerWidth;
  });
  expect(overflow).toBeLessThanOrEqual(0);
}

test.describe("Route smoke coverage", () => {
  for (const route of authRoutes) {
    test(`auth route ${route.path} renders without framework errors`, async ({ page }) => {
      const problems = watchConsole(page);

      await openRoute(page, route.path);

      await expect(page.getByRole("heading", { name: route.heading })).toBeVisible();
      await expectNoFrameworkOverlay(page);
      await expectNoHorizontalOverflow(page);
      expect(problems).toEqual([]);
    });
  }

  for (const route of protectedRoutes) {
    test(`signed-out shell route ${route.path} renders a login gate`, async ({ page }) => {
      const problems = watchConsole(page);

      await openRoute(page, route.path);

      await expect(page.getByRole("heading", { name: route.heading })).toBeVisible();
      await expect(page.getByText("Sign in required")).toBeVisible();
      await expectNoFrameworkOverlay(page);
      await expectNoHorizontalOverflow(page);
      expect(problems).toEqual([]);
    });
  }

  for (const route of protectedRoutes) {
    test(`mobile shell route ${route.path} has no horizontal overflow`, async ({ page }) => {
      const problems = watchConsole(page);
      await page.setViewportSize({ width: 390, height: 844 });

      await openRoute(page, route.path);

      await expect(page.getByText("Menu")).toBeVisible();
      await expect(page.getByRole("heading", { name: route.heading })).toBeVisible();
      await expectNoFrameworkOverlay(page);
      await expectNoHorizontalOverflow(page);
      expect(problems).toEqual([]);
    });
  }
});
