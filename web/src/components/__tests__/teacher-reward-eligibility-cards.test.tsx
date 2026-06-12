import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { RosterLearnerCard } from "@/components/teacher-routes/RosterLearnerCard";
import { StudentProgressCard } from "@/components/teacher-routes/StudentProgressCard";
import { type TeacherCourseRosterLearner, type TeacherCourseStudentProgressItem } from "@/lib/teacher";

const user = {
  email: "learner@example.com",
  email_verified: true,
  id: 7,
  kyc_verified: true,
  name: "Learner One",
};

const rewardEligibility = {
  active_policy_count: 1,
  event_types: ["course_completion"],
  reward_candidate_count: 3,
  supported: true,
};

function rosterLearner(): TeacherCourseRosterLearner {
  return {
    access_state: "enrolled",
    can_remove: false,
    latest_join_request_status: "approved",
    progress_supported: true,
    reward_eligibility: rewardEligibility,
    reward_eligibility_supported: true,
    roles: ["STUDENT"],
    user,
  };
}

function studentProgress(): TeacherCourseStudentProgressItem {
  return {
    access_state: "enrolled",
    latest_join_request_status: "approved",
    progress: {
      completed_content_count: 1,
      completion_percentage: 100,
      current_content_id: 11,
      current_content_label: "Module 1, lesson 1: article",
      last_activity_at: "2026-06-12T12:00:00Z",
      note: "Progress is tracked.",
      supported: true,
      total_content_count: 1,
    },
    reward_eligibility: rewardEligibility,
    rewards: {
      completed_count: 0,
      failed_count: 0,
      latest_candidate: null,
      pending_teacher_count: 1,
      reward_candidate_count: 3,
      teacher_approved_count: 2,
      teacher_rejected_count: 0,
    },
    roles: ["STUDENT"],
    user,
  };
}

describe("teacher reward eligibility cards", () => {
  it("shows active reward policy and candidate count on enrollment roster cards", () => {
    render(<RosterLearnerCard actionState="idle" confirmRemoval={false} learner={rosterLearner()} onRemove={vi.fn()} />);

    expect(screen.getByText("1 active: Course Completion")).toBeInTheDocument();
    expect(screen.getByText("Reward candidates")).toBeInTheDocument();
    expect(screen.getByText("3")).toBeInTheDocument();
  });

  it("shows reward eligibility next to persisted progress on student cards", () => {
    render(<StudentProgressCard student={studentProgress()} />);

    expect(screen.getByText("Reward eligibility")).toBeInTheDocument();
    expect(screen.getByText("1 active: Course Completion")).toBeInTheDocument();
    expect(screen.getByText("Reward candidates")).toBeInTheDocument();
  });
});
