export type RewardCandidateFilters = {
  appliedSearch: string;
  offset: number;
  searchInput: string;
  status: string;
};

export const defaultRewardCandidateFilters: RewardCandidateFilters = {
  appliedSearch: "",
  offset: 0,
  searchInput: "",
  status: "teacher_approved",
};
