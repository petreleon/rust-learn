import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { AdminRequestError } from "@/lib/admin/AdminRequestError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { AdminRewardAmountReviewRoute } from "../route/AdminRewardAmountReviewRoute";
import {
  decideAdminRewardAmount,
  loadAdminRewardAmountSession,
  loadAdminRewardCandidateAudit,
  loadAdminRewardCandidates,
} from "../api/rewardAmountReviewApi";
import {
  adminRewardAmountSession,
  rewardAuditEvent,
  rewardCandidate,
  rewardCandidateResponse,
} from "./adminRewardAmountTestFixtures";

vi.mock("next/navigation", () => ({
  usePathname: () => "/admin/rewards/amount-review",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/rewardAmountReviewApi", () => ({
  decideAdminRewardAmount: vi.fn(),
  loadAdminRewardAmountSession: vi.fn(),
  loadAdminRewardCandidateAudit: vi.fn(),
  loadAdminRewardCandidates: vi.fn(),
}));

function mockToken(token: string | null) {
  vi.mocked(readBrowserSessionToken).mockReturnValue(token);
}

describe("AdminRewardAmountReviewRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockToken("admin-token");
    vi.mocked(loadAdminRewardAmountSession).mockResolvedValue(
      adminRewardAmountSession(["VIEW_REWARD_AUDIT", "APPROVE_REWARD_AMOUNT"]),
    );
    vi.mocked(loadAdminRewardCandidates).mockResolvedValue(rewardCandidateResponse([]));
    vi.mocked(loadAdminRewardCandidateAudit).mockResolvedValue([]);
  });

  it("shows the signed-out state without loading reward review APIs", async () => {
    mockToken(null);

    render(<AdminRewardAmountReviewRoute />);

    expect(await screen.findByText("Sign in required")).toBeVisible();
    expect(loadAdminRewardAmountSession).not.toHaveBeenCalled();
    expect(loadAdminRewardCandidates).not.toHaveBeenCalled();
  });

  it("loads reward candidates and the selected candidate audit trail", async () => {
    vi.mocked(loadAdminRewardCandidates).mockResolvedValue(rewardCandidateResponse([rewardCandidate()]));
    vi.mocked(loadAdminRewardCandidateAudit).mockResolvedValue([rewardAuditEvent()]);

    render(<AdminRewardAmountReviewRoute />);

    expect((await screen.findAllByText("Ada Student"))[0]).toBeVisible();
    await waitFor(() =>
      expect(loadAdminRewardCandidates).toHaveBeenCalledWith({
        offset: 0,
        search: "",
        status: "teacher_approved",
        token: "admin-token",
      }),
    );
    await waitFor(() =>
      expect(loadAdminRewardCandidateAudit).toHaveBeenCalledWith({
        candidateId: 31,
        token: "admin-token",
      }),
    );
    expect(await screen.findByText("Reviewed by teacher")).toBeVisible();
  });

  it("submits an approved amount decision for the selected candidate", async () => {
    const updated = rewardCandidate({ approved_amount: "12.50", status: "amount_approved" });
    vi.mocked(loadAdminRewardCandidates)
      .mockResolvedValueOnce(rewardCandidateResponse([rewardCandidate()]))
      .mockResolvedValueOnce(rewardCandidateResponse([updated]));
    vi.mocked(decideAdminRewardAmount).mockResolvedValue(updated);
    const user = userEvent.setup();

    render(<AdminRewardAmountReviewRoute />);

    expect(await screen.findByText("Candidate 31")).toBeVisible();
    await user.type(screen.getByLabelText("Approved amount"), "12.50");
    await user.type(screen.getByLabelText("Decision reason"), "Evidence amount verified");
    await user.click(screen.getByRole("button", { name: "Save decision" }));

    await waitFor(() =>
      expect(decideAdminRewardAmount).toHaveBeenCalledWith({
        approvedAmount: "12.50",
        candidateId: 31,
        decisionReason: "Evidence amount verified",
        status: "approved",
        token: "admin-token",
      }),
    );
    expect(await screen.findByText("Decision saved")).toBeVisible();
    expect(await screen.findByText("This candidate is in a final state and cannot be changed from this screen.")).toBeVisible();
  });

  it("refreshes candidate data when the decision conflicts", async () => {
    const changed = rewardCandidate({ status: "amount_approved" });
    vi.mocked(loadAdminRewardCandidates)
      .mockResolvedValueOnce(rewardCandidateResponse([rewardCandidate()]))
      .mockResolvedValueOnce(rewardCandidateResponse([changed]));
    vi.mocked(decideAdminRewardAmount).mockRejectedValue(
      new AdminRequestError("Candidate changed", 409, "conflict"),
    );
    const user = userEvent.setup();

    render(<AdminRewardAmountReviewRoute />);

    expect(await screen.findByText("Candidate 31")).toBeVisible();
    await user.type(screen.getByLabelText("Approved amount"), "12.50");
    await user.type(screen.getByLabelText("Decision reason"), "Evidence amount verified");
    await user.click(screen.getByRole("button", { name: "Save decision" }));

    expect((await screen.findAllByText("Candidate changed"))[0]).toBeVisible();
    await waitFor(() => expect(loadAdminRewardCandidates).toHaveBeenCalledTimes(2));
    expect(await screen.findByText("This candidate is in a final state and cannot be changed from this screen.")).toBeVisible();
  });

  it("shows a permission gate before calling the reward candidate API", async () => {
    vi.mocked(loadAdminRewardAmountSession).mockResolvedValue(adminRewardAmountSession(["VIEW_REPORT"]));

    render(<AdminRewardAmountReviewRoute />);

    expect(await screen.findByText("Reward amount review unavailable")).toBeVisible();
    expect(loadAdminRewardCandidates).not.toHaveBeenCalled();
  });
});
