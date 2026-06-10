import { test, expect } from "@playwright/test";

test.describe("API — Health and session", () => {
  test("GET /healthz returns ok", async ({ request }) => {
    const response = await request.get("/healthz");
    expect(response.status()).toBe(200);
    const text = await response.text();
    expect(text).toContain("ok");
  });

  test("GET /api/health returns ready", async ({ request }) => {
    const response = await request.get("/api/health");
    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body).toHaveProperty("status", "ok");
  });

  test("GET /api/ready returns ready when dependencies available", async ({ request }) => {
    const response = await request.get("/api/ready");
    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body).toHaveProperty("status");
  });

  test("GET /api/me returns 401 without token", async ({ request }) => {
    const response = await request.get("/api/me");
    expect(response.status()).toBe(401);
  });
});

test.describe("API — Auth endpoints", () => {
  test("POST /api/auth/login returns 401 with invalid credentials", async ({ request }) => {
    const response = await request.post("/api/auth/login", {
      data: { email: "nobody@example.invalid", password: "wrong" },
    });
    expect(response.status()).toBe(401);
  });

  test("POST /api/auth/register returns 400 with empty body", async ({ request }) => {
    const response = await request.post("/api/auth/register", {
      data: {},
    });
    expect(response.status()).toBeGreaterThanOrEqual(400);
  });

  test("POST /api/auth/register returns 400 with invalid password", async ({ request }) => {
    const response = await request.post("/api/auth/register", {
      data: { name: "Test", email: "test@example.com", password: "short" },
    });
    expect(response.status()).toBeGreaterThanOrEqual(400);
  });
});

test.describe("API — Public endpoints", () => {
  test("GET /api/courses/discovery returns JSON", async ({ request }) => {
    const response = await request.get("/api/courses/discovery?limit=5");
    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body).toHaveProperty("courses");
    expect(body).toHaveProperty("total");
    expect(Array.isArray(body.courses)).toBe(true);
  });

  test("GET /api/me/notifications returns 401 without token", async ({ request }) => {
    const response = await request.get("/api/me/notifications");
    expect(response.status()).toBe(401);
  });
});
