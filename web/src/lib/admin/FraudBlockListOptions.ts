import { type AdminRequestOptions } from "./AdminRequestOptions";

export type FraudBlockListOptions = AdminRequestOptions & {
  scope_type?: string | null;
  active?: boolean | null;
  limit?: number;
  offset?: number;
};
