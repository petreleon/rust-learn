"use client";

import { ProductShell } from "@/components/product-shell";
import { ErrorPanel, LoadingPanel, SignedOutPanel } from "../components/SessionStatePanels";
import { SessionStatusItems } from "../components/SessionStatusItems";
import { SessionWorkspaceContent } from "../components/SessionWorkspaceContent";
import { type SessionWorkspaceRouteController } from "../route/useSessionWorkspaceRoute";
import { sessionNotice } from "./sessionNotice";

export function SessionWorkspaceView({ route }: { route: SessionWorkspaceRouteController }) {
  return (
    <ProductShell
      activeNav="session"
      breadcrumbs={[{ label: "Workspace" }]}
      description="Profile, workspace scopes, and delegated permissions resolved from the API."
      eyebrow="Account"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={sessionNotice(route.error)}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={
        <SessionStatusItems
          loadState={route.loadState}
          session={route.session}
          workspaceCount={route.workspaceCount}
        />
      }
      title="Current session"
    >
      <SessionWorkspaceBody route={route} />
    </ProductShell>
  );
}

function SessionWorkspaceBody({ route }: { route: SessionWorkspaceRouteController }) {
  if (route.loadState === "idle" && !route.session) return <SignedOutPanel />;
  if (route.loadState === "loading") return <LoadingPanel />;
  if (route.error) return <ErrorPanel error={route.error} />;
  if (route.session) {
    return (
      <SessionWorkspaceContent
        onRefresh={() => void route.loadSession()}
        onSignOut={route.signOut}
        session={route.session}
      />
    );
  }
  return null;
}
