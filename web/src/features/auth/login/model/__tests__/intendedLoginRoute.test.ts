import { describe, expect, it } from "vitest";
import { intendedLoginRoute } from "../intendedLoginRoute";

describe("intendedLoginRoute", () => {
  it("uses a safe same-origin redirect path", () => {
    window.history.replaceState(null, "", "/login?redirect=/teach/courses/9/content");

    expect(intendedLoginRoute()).toBe("/teach/courses/9/content");
  });

  it("falls back when the redirect is external or missing", () => {
    window.history.replaceState(null, "", "/login?redirect=//evil.example/path");
    expect(intendedLoginRoute()).toBe("/session");

    window.history.replaceState(null, "", "/login");
    expect(intendedLoginRoute()).toBe("/session");
  });
});
