export type BurnLeaderboardScope = "all" | "users" | "organizations";
export type BurnLeaderboardWindow = "7d" | "30d" | "365d";

export type BurnLeaderboardRow = {
  burn_count: number;
  burner_type: string;
  latest_burn_at: string;
  organization_id: number | null;
  rank: number;
  total_burned: string;
  user_id: number | null;
};

export type BurnLeaderboard = {
  rows: BurnLeaderboardRow[];
  scope: BurnLeaderboardScope;
  window_days: number;
};
