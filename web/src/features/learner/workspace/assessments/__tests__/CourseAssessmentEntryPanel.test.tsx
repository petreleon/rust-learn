import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { type AssessmentItem, type CourseCatalogItem } from "@/lib/learner";
import { CourseAssessmentEntryPanel } from "../components/CourseAssessmentEntryPanel";

const assessment: AssessmentItem = {
  course_id: 9,
  created_at: "2026-06-16T00:00:00Z",
  description: "Checks ownership basics.",
  id: 42,
  max_attempts: 2,
  passing_score: 70,
  published: true,
  questions: [{
    assessment_id: 42,
    id: 101,
    options: ["Ownership", "Borrow checker"],
    order: 0,
    points: 2,
    question_type: "multiple_choice",
    text: "Which concept owns values?",
  }],
  title: "Ownership check",
  updated_at: "2026-06-16T00:00:00Z",
};

function course(overrides: Partial<CourseCatalogItem> = {}): CourseCatalogItem {
  return {
    access: {
      can_request_join: false,
      can_view_content: true,
      can_view_course: true,
      can_view_rewards: false,
    },
    content: {
      chapter_count: 1,
      content_count: 1,
      content_types: ["text"],
      has_content: true,
    },
    description: "Course",
    enrollment: {
      can_request_join: false,
      reason: null,
      request_id: null,
      roles: ["learner"],
      state: "enrolled",
    },
    id: 9,
    lifecycle_status: "published",
    organizations: [],
    prerequisites: [],
    rewards: {
      active_policy_count: 0,
      available: false,
      event_types: [],
      payment_strategies: [],
      token_amounts: [],
    },
    teachers: [],
    title: "Rust Safety",
    topics: [],
    ...overrides,
  };
}

describe("CourseAssessmentEntryPanel", () => {
  it("does not render an assessment link when there are no assessments", () => {
    render(<CourseAssessmentEntryPanel assessments={[]} course={course()} />);

    expect(screen.queryByRole("heading", { name: "Assessments" })).not.toBeInTheDocument();
  });

  it("links enrolled learners to course assessments", () => {
    render(<CourseAssessmentEntryPanel assessments={[assessment]} course={course()} />);

    expect(screen.getByRole("heading", { name: "Assessments" })).toBeVisible();
    expect(screen.getByRole("link", { name: "Open assessments" })).toHaveAttribute(
      "href",
      "/courses/9/learn",
    );
  });

  it("shows preview wording for non-enrolled content access", () => {
    render(
      <CourseAssessmentEntryPanel
        assessments={[assessment]}
        course={course({ enrollment: { ...course().enrollment, state: "available" } })}
      />,
    );

    expect(screen.getByRole("link", { name: "Preview assessments" })).toHaveAttribute(
      "href",
      "/courses/9/learn",
    );
  });

  it("does not link assessments without content access", () => {
    render(
      <CourseAssessmentEntryPanel
        assessments={[assessment]}
        course={course({ access: { ...course().access, can_view_content: false } })}
      />,
    );

    expect(screen.queryByRole("link", { name: /assessments/i })).not.toBeInTheDocument();
    expect(screen.getByText("Request course access before opening assessments.")).toBeVisible();
  });
});
