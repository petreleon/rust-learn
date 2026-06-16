"use client";

import { AccountSettingsView } from "../view/AccountSettingsView";
import { useAccountSettingsRoute } from "./useAccountSettingsRoute";

export function AccountSettingsRoute() {
  const route = useAccountSettingsRoute();

  return <AccountSettingsView route={route} />;
}
