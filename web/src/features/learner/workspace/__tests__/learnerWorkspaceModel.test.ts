import { describe, expect, it } from "vitest";
import { type RewardHistoryEntry } from "@/lib/learner/RewardHistoryEntry";
import { contentStateTone } from "../components/contentStateTone";
import { enrollmentTone } from "../components/enrollmentTone";
import { humanize } from "../components/humanize";
import { isWalletCreditPending } from "../components/isWalletCreditPending";
import { lifecycleTone } from "../components/lifecycleTone";
import { plural } from "../components/plural";
import { rewardTone } from "../components/rewardTone";
import { summarizeRewards } from "../components/summarizeRewards";

describe("learner workspace model helpers", () => {
  it("maps reward tones", () => {
    expect(rewardTone("wallet_credited")).toBe("good");
    expect(rewardTone("failed")).toBe("bad");
    expect(rewardTone("teacher_rejected")).toBe("warn");
    expect(rewardTone("pending_teacher_approval")).toBe("neutral");
  });

  it("maps enrollment and lifecycle tones", () => {
    expect(enrollmentTone("enrolled")).toBe("good");
    expect(enrollmentTone("pending")).toBe("warn");
    expect(enrollmentTone("rejected")).toBe("bad");
    expect(lifecycleTone("published")).toBe("good");
    expect(lifecycleTone("archived")).toBe("warn");
    expect(lifecycleTone("suspended")).toBe("bad");
  });

  it("maps content states", () => {
    expect(contentStateTone("ready")).toBe("good");
    expect(contentStateTone("processing")).toBe("warn");
    expect(contentStateTone("failed_processing")).toBe("bad");
  });

  it("humanizes copied backend values and policy counts", () => {
    expect(humanize("pending_teacher_approval")).toBe("pending teacher approval");
    expect(plural(1)).toBe("policy");
    expect(plural(2)).toBe("policies");
  });

  it("summarizes reward history", () => {
    expect(
      summarizeRewards([
        reward({ status: "pending_teacher_approval" }),
        reward({ status: "token_pending" }),
        reward({ status: "wallet_credited", wallet_credit: walletCredit() }),
        reward({ status: "failed" }),
      ]),
    ).toEqual({
      credited: 1,
      needsHelp: 1,
      pendingTeacher: 1,
      processing: 1,
    });
  });

  it("detects pending wallet credits", () => {
    expect(isWalletCreditPending(reward({ status: "amount_approved" }))).toBe(true);
    expect(isWalletCreditPending(reward({ status: "amount_approved", wallet_credit: walletCredit() }))).toBe(false);
    expect(isWalletCreditPending(reward({ status: "teacher_rejected" }))).toBe(false);
  });
});

function reward(overrides: Partial<RewardHistoryEntry>): RewardHistoryEntry {
  return {
    approved_amount: "10",
    course_id: 1,
    course_title: "Rust",
    created_at: "2026-06-12T10:00:00Z",
    event_type: "completed",
    reward_candidate_id: 7,
    status: "pending_teacher_approval",
    token_transaction: null,
    updated_at: "2026-06-12T10:00:00Z",
    wallet_credit: null,
    ...overrides,
  };
}

function walletCredit(): NonNullable<RewardHistoryEntry["wallet_credit"]> {
  return {
    amount: "10",
    credited_at: "2026-06-12T10:00:00Z",
    internal_transaction_id: 44,
    reward_wallet_credit_record_id: 55,
    transaction_id: 66,
    wallet_id: 77,
  };
}
