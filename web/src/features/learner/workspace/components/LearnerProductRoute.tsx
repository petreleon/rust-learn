"use client";

import { ProductShell } from "@/components/product-shell";
import { CoursesContent } from "./CoursesContent";
import { ErrorState } from "./ErrorState";
import { LoadingState } from "./LoadingState";
import { RewardsContent } from "./RewardsContent";
import { SignedOutState } from "./SignedOutState";
import { StatusPill } from "./StatusPill";
import { WalletContent } from "./WalletContent";
import { learnerNotice } from "./learnerNotice";
import { routeConfig } from "./routeConfig";
import { type LearnerProductRouteKind } from "./LearnerProductRouteKind";
import { useLearnerProductRoute } from "./useLearnerProductRoute";

export function LearnerProductRoute({ kind }: { kind: LearnerProductRouteKind }) {
  const config = routeConfig[kind];
  const route = useLearnerProductRoute(kind);
  const notice = route.actionNotice || learnerNotice(route.error, kind);
  const statusLabel = route.loadState === "loading" ? "Loading" : route.session ? "Learner access" : "Sign in required";

  return (
    <ProductShell
      activeNav="learn"
      breadcrumbs={[{ href: "/session", label: "Workspace" }, { href: "/learn", label: "Learner" }, { label: config.title }]}
      description={config.description}
      eyebrow="Learner"
      isSignedIn={route.hasToken || Boolean(route.session)}
      notice={notice}
      onSignOut={route.signOut}
      session={route.session}
      statusItems={<StatusPill label={statusLabel} tone={route.session ? "good" : "neutral"} />}
      title={config.title}
    >
      {route.loadState === "idle" && !route.session ? <SignedOutState redirect={config.href} /> : null}
      {route.loadState === "loading" ? <LoadingState /> : null}
      {route.error ? <ErrorState error={route.error} onRetry={route.loadRoute} redirect={config.href} /> : null}
      {route.loadState === "success" && route.session && kind === "courses" ? (
        <CoursesContent
          catalog={route.catalog}
          enrollmentFilter={route.courseEnrollmentFilter}
          joiningCourseId={route.joiningCourseId}
          onApplySearch={route.applyCourseSearch}
          onChangeEnrollmentFilter={route.setCourseEnrollmentFilter}
          onChangeRewardOnly={route.setCourseRewardOnly}
          onChangeSearchInput={route.setCourseSearchInput}
          onClearFilters={route.clearCourseFilters}
          onRefresh={route.loadRoute}
          onRequestJoin={route.requestJoin}
          rewardOnly={route.courseRewardOnly}
          search={route.courseSearch}
          searchInput={route.courseSearchInput}
          session={route.session}
        />
      ) : null}
      {route.loadState === "success" && route.session && kind === "rewards" ? (
        <RewardsContent filter={route.rewardStatus} onChangeFilter={route.setRewardStatus} rewards={route.rewards} />
      ) : null}
      {route.loadState === "success" && route.session && kind === "wallet" ? (
        <WalletContent
          kycVerified={route.session.user.kyc_verified}
          linking={route.walletLinking}
          onLinkWallet={route.linkWallet}
          onRefresh={route.loadRoute}
          rewards={route.rewards}
          wallet={route.wallet}
          walletHistory={route.walletHistory}
        />
      ) : null}
    </ProductShell>
  );
}
