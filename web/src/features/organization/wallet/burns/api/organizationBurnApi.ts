import { DEFAULT_TIMEOUT_MS } from "@/lib/organization/DEFAULT_TIMEOUT_MS";
import { organizationJsonRequest } from "@/lib/organization/organizationJsonRequest";
import { type OrganizationWalletOptions } from "@/lib/organization/OrganizationWalletOptions";
import { organizationBurnPayload } from "../model/organizationBurnPayload";
import {
  type OrganizationBurnDraft,
  type OrganizationBurnPermissions,
  type OrganizationTokenBurn,
} from "../model/organizationBurnTypes";

export function fetchOrganizationBurnPermissions({
  apiRoot = "/api",
  organizationId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationWalletOptions): Promise<OrganizationBurnPermissions> {
  return organizationJsonRequest({
    apiRoot,
    path: `/wallets/organizations/${organizationId}/burns/permissions`,
    timeoutMs,
    token,
  });
}

export function fetchOrganizationTokenBurns({
  apiRoot = "/api",
  organizationId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationWalletOptions): Promise<OrganizationTokenBurn[]> {
  return organizationJsonRequest({
    apiRoot,
    path: `/wallets/organizations/${organizationId}/burns`,
    timeoutMs,
    token,
  });
}

export function createOrganizationTokenBurn({
  apiRoot = "/api",
  draft,
  idempotencyKey,
  organizationId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationWalletOptions & {
  draft: OrganizationBurnDraft;
  idempotencyKey: string;
}): Promise<OrganizationTokenBurn> {
  return organizationJsonRequest({
    apiRoot,
    body: JSON.stringify(organizationBurnPayload(draft, idempotencyKey)),
    method: "POST",
    path: `/wallets/organizations/${organizationId}/burns`,
    timeoutMs,
    token,
  });
}
