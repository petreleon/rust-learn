import { type TeacherApplicationStatus } from "@/lib/admin/TeacherApplicationStatus";

export type TeacherApplicationFilters = {
  appliedSearch: string;
  offset: number;
  searchInput: string;
  status: TeacherApplicationStatus | "";
};

export const defaultTeacherApplicationFilters: TeacherApplicationFilters = {
  appliedSearch: "",
  offset: 0,
  searchInput: "",
  status: "submitted",
};
