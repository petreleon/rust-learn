export type DecideTeacherJoinRequestPayload = {
  decision_reason?: string | null;
  status: "approved" | "rejected" | "waitlisted";
};
