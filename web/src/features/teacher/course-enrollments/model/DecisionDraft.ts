import { type DecisionStatus } from "./DecisionStatus";

export type DecisionDraft = {
  reason: string;
  status: DecisionStatus;
};
