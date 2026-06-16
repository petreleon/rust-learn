export type FraudBlockFilters = {
  active: boolean | null;
  offset: number;
  scopeType: string;
  searchInput: string;
};

export const defaultFraudBlockFilters: FraudBlockFilters = {
  active: true,
  offset: 0,
  scopeType: "",
  searchInput: "",
};
