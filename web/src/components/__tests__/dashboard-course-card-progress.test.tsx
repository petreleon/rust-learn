import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { DashboardCourseCard } from "@/components/learner-routes/DashboardCourseCard";
import { type CourseCatalogItem } from "@/lib/learner";

const course: CourseCatalogItem = {
  access: { can_request_join: false, can_view_content: true, can_view_course: true, can_view_rewards: false },
  content: { chapter_count: 1, content_count: 2, content_types: ["text"], has_content: true },
  description: null,
  enrollment: { can_request_join: false, reason: "You already have course access.", request_id: null, roles: ["STUDENT"], state: "enrolled" },
  id: 7,
  lifecycle_status: "published",
  organizations: [],
  prerequisites: [],
  rewards: { active_policy_count: 0, available: false, event_types: [], payment_strategies: [], token_amounts: [] },
  teachers: [],
  title: "Ownership Basics",
  topics: [],
};

describe("DashboardCourseCard progress", () => {
  it("shows saved progress for enrolled courses", () => {
    render(
      <DashboardCourseCard
        course={course}
        progress={{ content_id: 44, course_id: 7, id: 3, user_id: 9, viewed_at: "2026-06-12T10:00:00Z" }}
      />,
    );

    expect(screen.getByText(/Progress saved/i)).toBeVisible();
    expect(screen.getByRole("link", { name: /Learn/i })).toHaveAttribute("href", "/courses/7/learn");
  });

  it("shows an explicit empty progress state when checked", () => {
    render(<DashboardCourseCard course={course} progress={null} />);
    expect(screen.getByText("No saved progress yet")).toBeVisible();
  });
});
