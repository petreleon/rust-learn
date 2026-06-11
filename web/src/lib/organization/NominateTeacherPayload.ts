export type NominateTeacherPayload = {
  applicantUserId: number;
  experienceSummary: string;
  idempotencyKey?: string;
  portfolioLinks?: string[];
  requestedCourseId?: number;
  requestedScope?: string;
};
