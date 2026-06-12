import { afterEach, describe, expect, it, vi } from "vitest";
import { fetchLearnerDashboard } from "@/lib/learner";
import { type CourseCatalogItem } from "@/lib/learner";

function course(id: number): CourseCatalogItem {
  return {
    access: { can_request_join: false, can_view_content: true, can_view_course: true, can_view_rewards: false },
    content: { chapter_count: 1, content_count: 2, content_types: ["text"], has_content: true },
    description: null,
    enrollment: { can_request_join: false, reason: null, request_id: null, roles: ["STUDENT"], state: "enrolled" },
    id,
    lifecycle_status: "published",
    organizations: [],
    prerequisites: [],
    rewards: { active_policy_count: 0, available: false, event_types: [], payment_strategies: [], token_amounts: [] },
    teachers: [],
    title: `Course ${id}`,
    topics: [],
  };
}

function catalog(courses: CourseCatalogItem[]) {
  return { courses, enrollment_status: null, lifecycle_status: null, limit: 5, offset: 0, organization_id: null, reward_available: null, search: null, total: courses.length };
}

describe("fetchLearnerDashboard", () => {
  afterEach(() => vi.unstubAllGlobals());

  it("loads saved progress for enrolled courses with content", async () => {
    const fetchMock = vi.fn(async (url: string) => {
      if (url.includes("enrollment_status=enrolled")) return Response.json(catalog([course(7)]));
      if (url.includes("enrollment_status=available")) return Response.json(catalog([]));
      if (url.includes("/reward-candidates/me/history")) return Response.json([]);
      if (url.endsWith("/wallets/me")) return new Response("Not found", { status: 404 });
      if (url.endsWith("/courses/7/progress")) {
        return Response.json({ content_id: 44, course_id: 7, id: 3, user_id: 9, viewed_at: "2026-06-12T10:00:00Z" });
      }
      throw new Error(`Unexpected URL ${url}`);
    });
    vi.stubGlobal("fetch", fetchMock);

    const dashboard = await fetchLearnerDashboard({ apiRoot: "http://api.test", token: "token" });

    expect(dashboard.progress_by_course[7]?.content_id).toBe(44);
    expect(fetchMock).toHaveBeenCalledWith(
      "http://api.test/courses/7/progress",
      expect.objectContaining({ headers: { Authorization: "Bearer token" }, method: "GET" }),
    );
  });
});
