import { type FormEvent, useCallback, useEffect, useState } from "react";
import { type ShellNotice } from "@/components/product-shell";
import { fetchCourseCatalog, fetchLearnerWallet, fetchRewardHistory, linkMyWallet, requestCourseJoin, type CourseCatalogItem, type CourseCatalogResponse, type RewardHistoryEntry, type WalletSummary } from "@/lib/learner";
import { clearStoredSessionToken, fetchCurrentSession, readStoredSessionToken, type CurrentSession } from "@/lib/session";
import { type EnrollmentStatusFilter } from "./EnrollmentStatusFilter";
import { humanize } from "./humanize";
import { type LearnerProductRouteKind } from "./LearnerProductRouteKind";
import { type LoadState } from "./LoadState";
import { normalizeRouteError } from "./normalizeRouteError";
import { type RewardStatusFilter } from "./RewardStatusFilter";
import { type RouteError } from "./RouteError";

export function useLearnerProductRoute(kind: LearnerProductRouteKind) {
  const [hasToken, setHasToken] = useState(false);
  const [session, setSession] = useState<CurrentSession | null>(null);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [error, setError] = useState<RouteError | null>(null);
  const [actionNotice, setActionNotice] = useState<ShellNotice | null>(null);
  const [catalog, setCatalog] = useState<CourseCatalogResponse | null>(null);
  const [courseEnrollmentFilter, setCourseEnrollmentFilter] = useState<EnrollmentStatusFilter>("all");
  const [courseRewardOnly, setCourseRewardOnly] = useState(false);
  const [courseSearch, setCourseSearch] = useState("");
  const [courseSearchInput, setCourseSearchInput] = useState("");
  const [joiningCourseId, setJoiningCourseId] = useState<number | null>(null);
  const [rewards, setRewards] = useState<RewardHistoryEntry[]>([]);
  const [rewardStatus, setRewardStatus] = useState<RewardStatusFilter>("all");
  const [wallet, setWallet] = useState<WalletSummary | null>(null);
  const [walletLinking, setWalletLinking] = useState(false);

  const loadRoute = useCallback(async () => {
    const token = readStoredSessionToken();
    if (!token) {
      setHasToken(false);
      setSession(null);
      setError(null);
      setCatalog(null);
      setRewards([]);
      setWallet(null);
      setLoadState("idle");
      return;
    }
    setHasToken(true);
    setLoadState("loading");
    setError(null);
    try {
      const rewardsPromise =
        kind === "rewards"
          ? fetchRewardHistory({ status: rewardStatus === "all" ? undefined : rewardStatus, token })
          : Promise.resolve([]);
      const catalogPromise =
        kind === "courses"
          ? fetchCourseCatalog({
              enrollmentStatus: courseEnrollmentFilter === "all" ? undefined : courseEnrollmentFilter,
              rewardAvailable: courseRewardOnly ? true : undefined,
              search: courseSearch,
              token,
            })
          : Promise.resolve(null);
      const walletPromise = kind === "wallet" ? fetchLearnerWallet({ token }) : Promise.resolve(null);
      const [nextSession, nextRewards, nextCatalog, nextWalletSnapshot] = await Promise.all([
        fetchCurrentSession({ token }),
        rewardsPromise,
        catalogPromise,
        walletPromise,
      ]);
      setSession(nextSession);
      setRewards(kind === "wallet" ? nextWalletSnapshot?.reward_history || [] : nextRewards);
      setCatalog(nextCatalog);
      setWallet(nextWalletSnapshot?.wallet || null);
      setLoadState("success");
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (requestError.status === 401 || requestError.status === 404) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setSession(null);
      setError({ code: requestError.code, message: requestError.message, status: requestError.status });
      setLoadState("error");
    }
  }, [courseEnrollmentFilter, courseRewardOnly, courseSearch, kind, rewardStatus]);

  useEffect(() => {
    const timeout = window.setTimeout(() => void loadRoute(), 0);
    return () => window.clearTimeout(timeout);
  }, [loadRoute]);

  function signOut() {
    clearStoredSessionToken();
    setHasToken(false);
    setSession(null);
    setError(null);
    setActionNotice(null);
    setCatalog(null);
    setRewards([]);
    setWallet(null);
    setLoadState("idle");
  }

  async function linkWallet() {
    const token = readStoredSessionToken();
    if (!token) return setLoadState("idle");
    setWalletLinking(true);
    setActionNotice(null);
    try {
      const result = await linkMyWallet({ token });
      setWallet(result.wallet);
      setActionNotice({
        message: result.created ? "Approved rewards can now be credited to this wallet." : "This account already had a RustLearn wallet.",
        title: result.created ? "Wallet linked" : "Wallet ready",
        tone: "success",
      });
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (requestError.status === 401) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setActionNotice({
        message: requestError.message,
        title: requestError.code,
        tone: requestError.code === "timeout" || requestError.code === "network_error" ? "warn" : "error",
      });
    } finally {
      setWalletLinking(false);
    }
  }

  async function requestJoin(course: CourseCatalogItem) {
    const token = readStoredSessionToken();
    if (!token) return setLoadState("idle");
    setJoiningCourseId(course.id);
    setActionNotice(null);
    try {
      const joinRequest = await requestCourseJoin({ courseId: course.id, token });
      setActionNotice({
        message: `${course.title} is now ${humanize(joinRequest.status)} and waiting for course staff when review is required.`,
        title: "Enrollment request sent",
        tone: "success",
      });
      await loadRoute();
    } catch (nextError) {
      const requestError = normalizeRouteError(nextError);
      if (requestError.status === 401 || requestError.status === 404) {
        clearStoredSessionToken();
        setHasToken(false);
      }
      setActionNotice({
        message: requestError.message,
        title: requestError.code,
        tone: requestError.code === "timeout" || requestError.code === "network_error" ? "warn" : "error",
      });
    } finally {
      setJoiningCourseId(null);
    }
  }

  function applyCourseSearch(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setCourseSearch(courseSearchInput.trim());
  }

  function clearCourseFilters() {
    setCourseEnrollmentFilter("all");
    setCourseRewardOnly(false);
    setCourseSearch("");
    setCourseSearchInput("");
  }

  return { actionNotice, applyCourseSearch, catalog, clearCourseFilters, courseEnrollmentFilter,
    courseRewardOnly, courseSearch, courseSearchInput, error, hasToken, joiningCourseId, linkWallet,
    loadRoute, loadState, requestJoin, rewardStatus, rewards, session, setCourseEnrollmentFilter,
    setCourseRewardOnly, setCourseSearchInput, setRewardStatus, signOut, wallet, walletLinking };
}
