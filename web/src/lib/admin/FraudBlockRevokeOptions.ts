import { type AdminRequestOptions } from "./AdminRequestOptions";

export type FraudBlockRevokeOptions = AdminRequestOptions & {
  blockId: number;
};
