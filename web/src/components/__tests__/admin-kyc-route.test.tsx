import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { AdminKycReviewRoute } from "@/features/admin/kyc-review/route/AdminKycReviewRoute";
import { useAdminSession } from "@/components/admin-routes/useAdminSession";
import { decideKycSubmission, fetchKycReviewQueue, fetchKycSubmissionAudit, type KycSubmission } from "@/lib/admin";
import { type CurrentSession } from "@/lib/session";

vi.mock("next/navigation", () => ({
  usePathname: () => "/admin/kyc",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/components/admin-routes/useAdminSession", () => ({
  useAdminSession: vi.fn(),
}));

vi.mock("@/lib/admin", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@/lib/admin")>();
  return {
    ...actual,
    decideKycSubmission: vi.fn(),
    fetchKycReviewQueue: vi.fn(),
    fetchKycSubmissionAudit: vi.fn(),
  };
});

function session(permissions: string[]): CurrentSession {
  const kycReviewPermission = "REVIEW_KYC_SUBMISSIONS";

  return {
    access: {
      learner: true,
      teacher: false,
      teacher_application: false,
      organization: false,
      platform_admin: permissions.length > 0,
    },
    courses: [],
    delegated_permissions: [],
    organizations: [],
    platform: {
      delegated_permissions: [],
      direct_permissions: permissions,
      effective_permissions: permissions,
      capabilities: [
        {
          enabled: permissions.includes(kycReviewPermission),
          key: "kyc_reviews",
          label: "KYC review",
          permissions: [kycReviewPermission],
        },
      ],
      roles: permissions.length ? ["platform_admin"] : [],
    },
    user: {
      email: "admin@example.com",
      email_verified: true,
      id: 1,
      kyc_verified: true,
      name: "Admin User",
    },
  };
}

function submission(overrides: Partial<KycSubmission> = {}): KycSubmission {
  return {
    country_code: "US",
    created_at: "2026-06-12T09:00:00Z",
    document_last4: "1234",
    document_type: "passport",
    evidence_reference: "s3://kyc/evidence-1",
    id: 7,
    legal_name: "Learner User",
    provider_reference: null,
    rejection_reason: null,
    reviewed_at: null,
    reviewer_user_id: null,
    status: "submitted",
    submitted_at: "2026-06-12T09:00:00Z",
    updated_at: "2026-06-12T09:00:00Z",
    user_id: 42,
    ...overrides,
  };
}

function mockRoute(permissions = ["REVIEW_KYC_SUBMISSIONS"]) {
  vi.mocked(useAdminSession).mockReturnValue({
    error: null,
    hasToken: true,
    loadSession: vi.fn(),
    loadState: "success",
    session: session(permissions),
    signOut: vi.fn(),
    token: "admin-token",
  });
}

describe("AdminKycReviewRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(decideKycSubmission).mockReset();
    vi.mocked(fetchKycReviewQueue).mockReset();
    vi.mocked(fetchKycSubmissionAudit).mockReset();
    vi.mocked(fetchKycSubmissionAudit).mockResolvedValue([]);
    mockRoute();
  });

  it("shows the exact permission gate before loading KYC submissions", () => {
    mockRoute(["VIEW_REPORT"]);

    render(<AdminKycReviewRoute />);

    expect(screen.getByText("KYC review unavailable")).toBeVisible();
    expect(screen.getByText("Missing platform permission: REVIEW_KYC_SUBMISSIONS")).toBeVisible();
    expect(fetchKycReviewQueue).not.toHaveBeenCalled();
  });

  it("loads the queue and verifies a submission", async () => {
    const row = submission();
    vi.mocked(fetchKycReviewQueue)
      .mockResolvedValueOnce({ submissions: [row] })
      .mockResolvedValueOnce({ submissions: [] });
    vi.mocked(fetchKycSubmissionAudit).mockResolvedValue([
      {
        actor_user_id: 42,
        created_at: "2026-06-12T09:00:00Z",
        event_type: "submitted",
        from_status: null,
        id: 11,
        metadata: {},
        reason: null,
        submission_id: 7,
        to_status: "submitted",
      },
    ]);
    vi.mocked(decideKycSubmission).mockResolvedValue({ ...row, status: "verified" });
    const user = userEvent.setup();

    render(<AdminKycReviewRoute />);

    expect((await screen.findAllByText("Learner User")).length).toBeGreaterThan(0);
    expect((await screen.findAllByText("Submitted")).length).toBeGreaterThan(0);
    await user.click(screen.getByRole("button", { name: "Save KYC decision" }));

    await waitFor(() =>
      expect(decideKycSubmission).toHaveBeenCalledWith({
        rejectionReason: "",
        status: "verified",
        submissionId: 7,
        token: "admin-token",
      }),
    );
    expect(await screen.findByText("KYC decision saved")).toBeVisible();
  });

  it("requires a reason before rejecting a submission", async () => {
    vi.mocked(fetchKycReviewQueue).mockResolvedValue({ submissions: [submission()] });
    vi.mocked(decideKycSubmission).mockResolvedValue(submission({ status: "rejected" }));
    const user = userEvent.setup();

    render(<AdminKycReviewRoute />);

    expect((await screen.findAllByText("Learner User")).length).toBeGreaterThan(0);
    await user.selectOptions(screen.getByLabelText("Decision status"), "rejected");
    expect(screen.getByRole("button", { name: "Save KYC decision" })).toBeDisabled();
    await user.type(screen.getByLabelText("Rejection reason"), "Document is expired");
    await user.click(screen.getByRole("button", { name: "Save KYC decision" }));

    await waitFor(() =>
      expect(decideKycSubmission).toHaveBeenCalledWith({
        rejectionReason: "Document is expired",
        status: "rejected",
        submissionId: 7,
        token: "admin-token",
      }),
    );
  });
});
