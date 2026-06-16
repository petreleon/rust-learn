import { type TeacherContentProcessingHistory } from "@/lib/teacher";

export type ContentProcessingHistoryState = {
  history: TeacherContentProcessingHistory | null;
  message: string | null;
  status: "idle" | "loading" | "success" | "error";
};

export const idleProcessingHistoryState: ContentProcessingHistoryState = {
  history: null,
  message: null,
  status: "idle",
};
