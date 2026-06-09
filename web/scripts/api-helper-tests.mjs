import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import test from "node:test";
import { pathToFileURL } from "node:url";
import ts from "typescript";

const repoRoot = path.resolve(import.meta.dirname, "..");
const compiledDir = mkdtempSync(path.join(tmpdir(), "rustlearn-api-helper-tests-"));

const session = await importTranspiled("src/lib/session.ts");
const auth = await importTranspiled("src/lib/auth.ts");
const learner = await importTranspiled("src/lib/learner.ts");
const organization = await importTranspiled("src/lib/organization.ts");
const teacher = await importTranspiled("src/lib/teacher.ts");
const admin = await importTranspiled("src/lib/admin.ts");

test("fetchCurrentSession parses JSON success and sends bearer token", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(url, "/api/me");
    assert.equal(init.headers.Authorization, "Bearer session-token");
    return jsonResponse(currentSessionFixture());
  });

  const result = await session.fetchCurrentSession({ token: "session-token" });

  assert.equal(calls.length, 1);
  assert.equal(result.user.email, "learner@example.test");
  assert.deepEqual(result.platform.effective_permissions, ["VIEW_REPORT"]);
});

test("fetchCurrentSession normalizes JSON 401 and 403 errors", async () => {
  mockFetch(() =>
    jsonResponse(
      {
        error: {
          code: "unauthorized",
          message: "A valid bearer token is required.",
        },
      },
      { status: 401 },
    ),
  );

  await assertRequestError(session.fetchCurrentSession({ token: "bad-token" }), {
    code: "unauthorized",
    errorClass: session.SessionRequestError,
    status: 401,
  });

  mockFetch(() =>
    jsonResponse(
      {
        error: {
          code: "unverified_email",
          message: "Email verification is required before using this session.",
        },
      },
      { status: 403 },
    ),
  );

  await assertRequestError(session.fetchCurrentSession({ token: "unverified-token" }), {
    code: "unverified_email",
    errorClass: session.SessionRequestError,
    status: 403,
  });
});

test("fetchCurrentSession normalizes timeout and network failure", async () => {
  mockFetch((_url, init) => {
    return new Promise((_resolve, reject) => {
      init.signal.addEventListener("abort", () => {
        reject(new DOMException("The operation was aborted.", "AbortError"));
      });
    });
  });

  await assertRequestError(session.fetchCurrentSession({ token: "slow-token", timeoutMs: 1 }), {
    code: "timeout",
    errorClass: session.SessionRequestError,
    status: 0,
  });

  mockFetch(() => {
    throw new TypeError("fetch failed");
  });

  await assertRequestError(session.fetchCurrentSession({ token: "network-token" }), {
    code: "network_error",
    errorClass: session.SessionRequestError,
    status: 0,
  });
});

test("buildPlatformAdminWorkspace maps platform capabilities from resolved permissions", () => {
  const workspace = admin.buildPlatformAdminWorkspace(platformAdminSessionFixture());

  assert.equal(workspace.roles[0], "PLATFORM_ADMIN");
  assert.equal(workspace.directPermissionCount, 6);
  assert.equal(workspace.delegatedPermissionCount, 1);
  assert.equal(workspace.effectivePermissionCount, 7);
  assert.equal(admin.platformCapabilityEnabled(workspace, "summary"), true);
  assert.equal(admin.platformCapabilityEnabled(workspace, "reward_amount_review"), true);
  assert.equal(admin.platformCapabilityEnabled(workspace, "fraud_blocks"), true);
  assert.equal(admin.platformCapabilityEnabled(workspace, "exports"), true);
  assert.equal(admin.platformCapabilityEnabled(workspace, "delegations"), true);
  assert.equal(admin.platformCapabilityEnabled(workspace, "system"), true);
});

test("platform admin helpers parse dashboard JSON and readiness states", async () => {
  const calls = mockFetch((url, init) => {
    if (url.startsWith("/api/")) {
      assert.equal(init.headers.Authorization, "Bearer admin-token");
      assert.equal(init.headers.Accept, "application/json, text/plain");
    }

    if (url === "/api/reports/platform/summary") {
      return jsonResponse(platformSummaryFixture());
    }
    if (url === "/api/reports/platform/reward-dashboard") {
      return jsonResponse(platformRewardDashboardFixture());
    }
    if (url === "/api/reports/platform/fraud-dashboard") {
      return jsonResponse(platformFraudDashboardFixture());
    }
    if (url === "/health") {
      assert.equal(init.headers.Authorization, undefined);
      return jsonResponse({ status: "ok" });
    }
    if (url === "/ready") {
      assert.equal(init.headers.Authorization, undefined);
      return jsonResponse(
        {
          checks: [
            { message: null, name: "postgres", status: "ok" },
            { message: "s3 timed out", name: "s3", status: "failed" },
          ],
          status: "not_ready",
        },
        { status: 503 },
      );
    }
    throw new Error(`Unexpected URL ${url}`);
  });

  const summary = await admin.fetchPlatformSummary({ token: "admin-token" });
  const rewardDashboard = await admin.fetchPlatformRewardDashboard({ token: "admin-token" });
  const fraudDashboard = await admin.fetchPlatformFraudDashboard({ token: "admin-token" });
  const systemStatus = await admin.fetchPlatformSystemStatus();

  assert.equal(calls.length, 5);
  assert.equal(summary.total_users, 12);
  assert.equal(rewardDashboard.teacher_applications.submitted, 3);
  assert.equal(rewardDashboard.pending_amount_approvals[0].reward_candidate_id, 201);
  assert.equal(fraudDashboard.active_by_scope.organization, 1);
  assert.equal(systemStatus.liveness.status, "ok");
  assert.equal(systemStatus.readiness.status, "not_ready");
  assert.equal(systemStatus.readiness.checks[1].message, "s3 timed out");
});

test("platform CSV helper parses content disposition filename", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(url, "/api/reports/platform/reward-dashboard.csv");
    assert.equal(init.headers.Authorization, "Bearer admin-token");
    assert.equal(init.headers.Accept, "text/csv, text/plain");
    return textResponse("section,metric,value\nreward_candidates,total,9\n", {
      headers: {
        "content-disposition": 'attachment; filename="platform-reward-dashboard.csv"',
      },
    });
  });

  const csv = await admin.downloadPlatformCsv({
    report: "reward_dashboard",
    token: "admin-token",
  });

  assert.equal(calls.length, 1);
  assert.equal(csv.filename, "platform-reward-dashboard.csv");
  assert.match(csv.body, /reward_candidates,total,9/);
});

test("platform admin helpers normalize missing token, denied, missing, server, timeout, and network errors", async () => {
  await assertRequestError(admin.fetchPlatformSummary({ token: " " }), {
    code: "missing_token",
    errorClass: admin.AdminRequestError,
    status: 401,
  });

  mockFetch(() => textResponse("User does not have platform report permission", { status: 403 }));

  await assertRequestError(admin.fetchPlatformSummary({ token: "admin-token" }), {
    code: "permission_denied",
    errorClass: admin.AdminRequestError,
    status: 403,
  });

  mockFetch(() =>
    jsonResponse(
      {
        error: {
          code: "not_found",
          message: "Platform export not found",
        },
      },
      { status: 404 },
    ),
  );

  await assertRequestError(
    admin.downloadPlatformCsv({
      report: "summary",
      token: "admin-token",
    }),
    {
      code: "not_found",
      errorClass: admin.AdminRequestError,
      status: 404,
    },
  );

  mockFetch(() => textResponse("Failed to load reward dashboard", { status: 500 }));

  await assertRequestError(admin.fetchPlatformRewardDashboard({ token: "admin-token" }), {
    code: "server_error",
    errorClass: admin.AdminRequestError,
    status: 500,
  });

  mockFetch((_url, init) => {
    return new Promise((_resolve, reject) => {
      init.signal.addEventListener("abort", () => {
        reject(new DOMException("The operation was aborted.", "AbortError"));
      });
    });
  });

  await assertRequestError(admin.fetchPlatformFraudDashboard({ timeoutMs: 1, token: "admin-token" }), {
    code: "timeout",
    errorClass: admin.AdminRequestError,
    status: 0,
  });

  mockFetch(() => {
    throw new TypeError("fetch failed");
  });

  await assertRequestError(admin.fetchPlatformSystemStatus(), {
    code: "network_error",
    errorClass: admin.AdminRequestError,
    status: 0,
  });
});

test("buildOrganizationWorkspace keeps multi-org capability signals honest", () => {
  const workspace = organization.buildOrganizationWorkspace(organizationSessionFixture());

  assert.equal(workspace.total, 3);
  assert.equal(workspace.reportScopeCount, 1);
  assert.equal(workspace.managementScopeCount, 2);
  assert.equal(workspace.walletScopeCount, 1);
  assert.equal(workspace.teacherNominationScopeCount, 1);
  assert.equal(workspace.courseRewardScopeCount, 1);
  assert.equal(workspace.delegatedOrganizationCount, 1);

  const academy = workspace.organizations.find((item) => item.name === "Ferris Academy");
  assert.equal(academy.capabilities.find((item) => item.key === "reports").enabled, true);
  assert.equal(academy.capabilities.find((item) => item.key === "members").enabled, true);
  assert.equal(academy.capabilities.find((item) => item.key === "wallet").enabled, true);

  const guild = workspace.organizations.find((item) => item.name === "Rust Guild");
  assert.equal(guild.delegatedPermissionCount, 1);
  assert.equal(guild.capabilities.find((item) => item.key === "teacher_applications").enabled, true);
  assert.equal(guild.capabilities.find((item) => item.key === "course_rewards").enabled, true);

  const roleOnly = workspace.organizations.find((item) => item.name === "Role Only Org");
  assert.equal(roleOnly.roles[0], "ORG_MEMBER");
  assert.equal(roleOnly.effectivePermissionCount, 0);
});

test("filterOrganizationWorkspace searches permissions and preserves stale-route misses", () => {
  const sessionFixture = organizationSessionFixture();
  const workspace = organization.buildOrganizationWorkspace(sessionFixture);

  assert.deepEqual(
    organization
      .filterOrganizationWorkspace(workspace.organizations, {
        capability: "reports",
        search: "",
      })
      .map((item) => item.name),
    ["Ferris Academy"],
  );
  assert.deepEqual(
    organization
      .filterOrganizationWorkspace(workspace.organizations, {
        capability: "delegated",
        search: "nominate",
      })
      .map((item) => item.name),
    ["Rust Guild"],
  );
  assert.equal(organization.findOrganizationWorkspaceItem(sessionFixture, 7).name, "Ferris Academy");
  assert.equal(organization.findOrganizationWorkspaceItem(sessionFixture, 404), null);
});

test("organization dashboard helper parses triage summaries", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(url, "/api/organizations/7/dashboard");
    assert.equal(init.headers.Authorization, "Bearer org-token");
    assert.equal(init.headers.Accept, "application/json, text/plain");
    return jsonResponse(organizationDashboardFixture());
  });

  const result = await organization.fetchOrganizationDashboard({
    organizationId: 7,
    token: "org-token",
  });

  assert.equal(calls.length, 1);
  assert.equal(result.organization.name, "Ferris Academy");
  assert.equal(result.health.status, "attention");
  assert.equal(result.members.total, 3);
  assert.equal(result.courses.published, 1);
  assert.equal(result.teacher_applications.submitted, 2);
  assert.equal(result.rewards.approved_amount_total, "40");
  assert.equal(result.wallet.balance_total, "125");
  assert.equal(result.operator_permissions.can_view_reports, true);
  assert.equal(result.alerts[0].kind, "teacher_applications_submitted");
});

test("organization dashboard helper normalizes denied, missing, server, timeout, and network errors", async () => {
  mockFetch(() => textResponse("User does not have permission to view organization dashboard", { status: 403 }));

  await assertRequestError(
    organization.fetchOrganizationDashboard({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "permission_denied",
      errorClass: organization.OrganizationRequestError,
      status: 403,
    },
  );

  mockFetch(() => textResponse("Organization not found", { status: 404 }));

  await assertRequestError(
    organization.fetchOrganizationDashboard({
      organizationId: 404,
      token: "org-token",
    }),
    {
      code: "not_found",
      errorClass: organization.OrganizationRequestError,
      status: 404,
    },
  );

  mockFetch(() => textResponse("Failed to fetch organization dashboard", { status: 500 }));

  await assertRequestError(
    organization.fetchOrganizationDashboard({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "server_error",
      errorClass: organization.OrganizationRequestError,
      status: 500,
    },
  );

  mockFetch((_url, init) => {
    return new Promise((_resolve, reject) => {
      init.signal.addEventListener("abort", () => {
        reject(new DOMException("The operation was aborted.", "AbortError"));
      });
    });
  });

  await assertRequestError(
    organization.fetchOrganizationDashboard({
      organizationId: 7,
      timeoutMs: 1,
      token: "org-token",
    }),
    {
      code: "timeout",
      errorClass: organization.OrganizationRequestError,
      status: 0,
    },
  );

  mockFetch(() => {
    throw new TypeError("fetch failed");
  });

  await assertRequestError(
    organization.fetchOrganizationDashboard({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "network_error",
      errorClass: organization.OrganizationRequestError,
      status: 0,
    },
  );
});

test("organization report helpers parse dashboard JSON and CSV exports", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(init.headers.Authorization, "Bearer org-token");
    if (url === "/api/reports/organizations/7/reward-dashboard") {
      assert.equal(init.headers.Accept, "application/json, text/plain");
      return jsonResponse(organizationRewardDashboardFixture());
    }
    assert.equal(url, "/api/reports/organizations/7/reward-dashboard.csv");
    assert.equal(init.headers.Accept, "text/csv, text/plain");
    return textResponse("section,metric,value\norganization,organization_name,Ferris Academy\n", {
      headers: {
        "content-disposition": 'attachment; filename="organization-7-reward-dashboard.csv"',
      },
    });
  });

  const dashboard = await organization.fetchOrganizationRewardDashboard({
    organizationId: 7,
    token: "org-token",
  });
  const csv = await organization.downloadOrganizationRewardDashboardCsv({
    organizationId: 7,
    token: "org-token",
  });

  assert.equal(calls.length, 2);
  assert.equal(dashboard.organization_name, "Ferris Academy");
  assert.equal(dashboard.courses[0].course_title, "Rust Ownership Lab");
  assert.equal(dashboard.wallets[0].balance, "125");
  assert.equal(csv.filename, "organization-7-reward-dashboard.csv");
  assert.match(csv.body, /organization_name,Ferris Academy/);
});

test("organization report helpers normalize permission, missing, timeout, and network errors", async () => {
  mockFetch(() => textResponse("User does not have organization reward report permission", { status: 403 }));

  await assertRequestError(
    organization.fetchOrganizationRewardDashboard({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "permission_denied",
      errorClass: organization.OrganizationRequestError,
      status: 403,
    },
  );

  mockFetch(() =>
    jsonResponse(
      {
        error: {
          code: "not_found",
          message: "Organization not found",
        },
      },
      { status: 404 },
    ),
  );

  await assertRequestError(
    organization.downloadOrganizationRewardDashboardCsv({
      organizationId: 404,
      token: "org-token",
    }),
    {
      code: "not_found",
      errorClass: organization.OrganizationRequestError,
      status: 404,
    },
  );

  mockFetch((_url, init) => {
    return new Promise((_resolve, reject) => {
      init.signal.addEventListener("abort", () => {
        reject(new DOMException("The operation was aborted.", "AbortError"));
      });
    });
  });

  await assertRequestError(
    organization.fetchOrganizationRewardDashboard({
      organizationId: 7,
      timeoutMs: 1,
      token: "org-token",
    }),
    {
      code: "timeout",
      errorClass: organization.OrganizationRequestError,
      status: 0,
    },
  );

  mockFetch(() => {
    throw new TypeError("fetch failed");
  });

  await assertRequestError(
    organization.fetchOrganizationRewardDashboard({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "network_error",
      errorClass: organization.OrganizationRequestError,
      status: 0,
    },
  );
});

test("organization wallet helpers parse audit JSON and link results", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(init.headers.Authorization, "Bearer org-token");
    assert.equal(init.headers.Accept, "application/json, text/plain");
    if (url === "/api/wallets/organizations/7/audit") {
      assert.equal(init.method, "GET");
      return jsonResponse(organizationWalletAuditFixture());
    }
    assert.equal(url, "/api/wallets/organizations/7/link");
    assert.equal(init.method, "POST");
    return jsonResponse(
      {
        created: false,
        wallet: organizationWalletAuditFixture().wallet,
      },
      { status: 200 },
    );
  });

  const audit = await organization.fetchOrganizationWalletAudit({
    organizationId: 7,
    token: "org-token",
  });
  const linkResult = await organization.linkOrganizationWallet({
    organizationId: 7,
    token: "org-token",
  });

  assert.equal(calls.length, 2);
  assert.equal(audit.wallet.owner_type, "organization");
  assert.equal(audit.wallet.value, "125");
  assert.equal(audit.internal_transactions[0].transaction_type, "organization_budget_adjustment");
  assert.equal(audit.external_transactions[0].transaction_hash, "0xabc123456789def0abc123456789def0abc12345");
  assert.equal(audit.reward_records[0].reconciliation_status, "needs_wallet_credit");
  assert.equal(audit.compensation_records[0].reason, "Manual reconciliation");
  assert.equal(linkResult.created, false);
  assert.equal(linkResult.wallet.id, audit.wallet.id);
});

test("organization wallet helpers normalize missing, denied, server, timeout, and network errors", async () => {
  mockFetch(() => textResponse("Wallet not linked", { status: 404 }));

  await assertRequestError(
    organization.fetchOrganizationWalletAudit({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "not_found",
      errorClass: organization.OrganizationRequestError,
      status: 404,
    },
  );

  mockFetch(() => textResponse("User does not have organization wallet access", { status: 403 }));

  await assertRequestError(
    organization.linkOrganizationWallet({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "permission_denied",
      errorClass: organization.OrganizationRequestError,
      status: 403,
    },
  );

  mockFetch(() => textResponse("Failed to load wallet audit", { status: 500 }));

  await assertRequestError(
    organization.fetchOrganizationWalletAudit({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "server_error",
      errorClass: organization.OrganizationRequestError,
      status: 500,
    },
  );

  mockFetch((_url, init) => {
    return new Promise((_resolve, reject) => {
      init.signal.addEventListener("abort", () => {
        reject(new DOMException("The operation was aborted.", "AbortError"));
      });
    });
  });

  await assertRequestError(
    organization.fetchOrganizationWalletAudit({
      organizationId: 7,
      timeoutMs: 1,
      token: "org-token",
    }),
    {
      code: "timeout",
      errorClass: organization.OrganizationRequestError,
      status: 0,
    },
  );

  mockFetch(() => {
    throw new TypeError("fetch failed");
  });

  await assertRequestError(
    organization.fetchOrganizationWalletAudit({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "network_error",
      errorClass: organization.OrganizationRequestError,
      status: 0,
    },
  );
});

test("organization course helpers send filters and parse operator summaries", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(
      url,
      "/api/organizations/7/courses?search=rust&lifecycle_status=published&reward_available=true&limit=6&offset=6",
    );
    assert.equal(init.headers.Authorization, "Bearer org-token");
    assert.equal(init.headers.Accept, "application/json, text/plain");
    return jsonResponse(organizationCoursesFixture());
  });

  const result = await organization.fetchOrganizationCourses({
    lifecycleStatus: "published",
    limit: 6,
    offset: 6,
    organizationId: 7,
    rewardAvailable: true,
    search: " rust ",
    token: "org-token",
  });

  assert.equal(calls.length, 1);
  assert.equal(result.organization.name, "Ferris Academy");
  assert.equal(result.total, 1);
  assert.equal(result.courses[0].title, "Rust Ownership Lab");
  assert.equal(result.courses[0].teachers[0].name, "Ada Teacher");
  assert.equal(result.courses[0].content.content_count, 4);
  assert.equal(result.courses[0].roster.pending_join_request_count, 2);
  assert.equal(result.courses[0].rewards.available, true);
  assert.equal(result.courses[0].permissions.can_submit_reward_events, true);
});

test("organization course helpers normalize denied, missing, server, timeout, and network errors", async () => {
  mockFetch(() => textResponse("User does not have permission to view organization courses", { status: 403 }));

  await assertRequestError(
    organization.fetchOrganizationCourses({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "permission_denied",
      errorClass: organization.OrganizationRequestError,
      status: 403,
    },
  );

  mockFetch(() => textResponse("Organization not found", { status: 404 }));

  await assertRequestError(
    organization.fetchOrganizationCourses({
      organizationId: 404,
      token: "org-token",
    }),
    {
      code: "not_found",
      errorClass: organization.OrganizationRequestError,
      status: 404,
    },
  );

  mockFetch(() => textResponse("Failed to fetch organization courses", { status: 500 }));

  await assertRequestError(
    organization.fetchOrganizationCourses({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "server_error",
      errorClass: organization.OrganizationRequestError,
      status: 500,
    },
  );

  mockFetch((_url, init) => {
    return new Promise((_resolve, reject) => {
      init.signal.addEventListener("abort", () => {
        reject(new DOMException("The operation was aborted.", "AbortError"));
      });
    });
  });

  await assertRequestError(
    organization.fetchOrganizationCourses({
      organizationId: 7,
      timeoutMs: 1,
      token: "org-token",
    }),
    {
      code: "timeout",
      errorClass: organization.OrganizationRequestError,
      status: 0,
    },
  );

  mockFetch(() => {
    throw new TypeError("fetch failed");
  });

  await assertRequestError(
    organization.fetchOrganizationCourses({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "network_error",
      errorClass: organization.OrganizationRequestError,
      status: 0,
    },
  );
});

test("organization member helpers send filters and parse permission summaries", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(
      url,
      "/api/organizations/7/members?search=ada&role=ADMIN&permission=VIEW_ORGANIZATION&limit=8&offset=8",
    );
    assert.equal(init.headers.Authorization, "Bearer org-token");
    assert.equal(init.headers.Accept, "application/json, text/plain");
    return jsonResponse(organizationMembersFixture());
  });

  const result = await organization.fetchOrganizationMembers({
    limit: 8,
    offset: 8,
    organizationId: 7,
    permission: "VIEW_ORGANIZATION",
    role: "ADMIN",
    search: " ada ",
    token: "org-token",
  });

  assert.equal(calls.length, 1);
  assert.equal(result.organization.name, "Ferris Academy");
  assert.equal(result.total, 1);
  assert.equal(result.operator_permissions.can_invite_members, true);
  assert.equal(result.operator_permissions.can_assign_roles, false);
  assert.equal(result.members[0].name, "Ada Admin");
  assert.deepEqual(result.members[0].roles, ["ADMIN"]);
  assert.equal(result.members[0].direct_permission_count, 3);
  assert.equal(result.members[0].delegated_permissions[0], "VIEW_ORG_REWARD_REPORTS");
  assert.equal(result.members[0].effective_permissions.includes("VIEW_ORGANIZATION"), true);
});

test("organization member helpers normalize denied, missing, server, timeout, and network errors", async () => {
  mockFetch(() => textResponse("User does not have permission to view organization members", { status: 403 }));

  await assertRequestError(
    organization.fetchOrganizationMembers({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "permission_denied",
      errorClass: organization.OrganizationRequestError,
      status: 403,
    },
  );

  mockFetch(() => textResponse("Organization not found", { status: 404 }));

  await assertRequestError(
    organization.fetchOrganizationMembers({
      organizationId: 404,
      token: "org-token",
    }),
    {
      code: "not_found",
      errorClass: organization.OrganizationRequestError,
      status: 404,
    },
  );

  mockFetch(() => textResponse("Failed to fetch organization members", { status: 500 }));

  await assertRequestError(
    organization.fetchOrganizationMembers({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "server_error",
      errorClass: organization.OrganizationRequestError,
      status: 500,
    },
  );

  mockFetch((_url, init) => {
    return new Promise((_resolve, reject) => {
      init.signal.addEventListener("abort", () => {
        reject(new DOMException("The operation was aborted.", "AbortError"));
      });
    });
  });

  await assertRequestError(
    organization.fetchOrganizationMembers({
      organizationId: 7,
      timeoutMs: 1,
      token: "org-token",
    }),
    {
      code: "timeout",
      errorClass: organization.OrganizationRequestError,
      status: 0,
    },
  );

  mockFetch(() => {
    throw new TypeError("fetch failed");
  });

  await assertRequestError(
    organization.fetchOrganizationMembers({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "network_error",
      errorClass: organization.OrganizationRequestError,
      status: 0,
    },
  );
});

test("organization teacher application helpers send filters and parse tracking rows", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(
      url,
      "/api/organizations/7/teacher-applications?search=ada&status=submitted&limit=6&offset=6",
    );
    assert.equal(init.headers.Authorization, "Bearer org-token");
    assert.equal(init.headers.Accept, "application/json, text/plain");
    return jsonResponse(organizationTeacherApplicationsFixture());
  });

  const result = await organization.fetchOrganizationTeacherApplications({
    limit: 6,
    offset: 6,
    organizationId: 7,
    search: " ada ",
    status: "submitted",
    token: "org-token",
  });

  assert.equal(calls.length, 1);
  assert.equal(result.organization.name, "Ferris Academy");
  assert.equal(result.total, 1);
  assert.equal(result.summary.approved, 1);
  assert.equal(result.operator_permissions.can_nominate_teachers, true);
  assert.equal(result.applications[0].applicant.name, "Ada Applicant");
  assert.equal(result.applications[0].requested_organization.name, "Ferris Academy");
  assert.equal(result.applications[0].audit.latest_event_type, "organization_nominated");
  assert.equal(result.applications[0].portfolio_links[0], "https://example.test/portfolio");
});

test("organization teacher application helpers normalize denied, missing, server, timeout, and network errors", async () => {
  mockFetch(() => textResponse("User does not have permission to view organization teacher applications", { status: 403 }));

  await assertRequestError(
    organization.fetchOrganizationTeacherApplications({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "permission_denied",
      errorClass: organization.OrganizationRequestError,
      status: 403,
    },
  );

  mockFetch(() => textResponse("Organization not found", { status: 404 }));

  await assertRequestError(
    organization.fetchOrganizationTeacherApplications({
      organizationId: 404,
      token: "org-token",
    }),
    {
      code: "not_found",
      errorClass: organization.OrganizationRequestError,
      status: 404,
    },
  );

  mockFetch(() => textResponse("Failed to fetch organization teacher applications", { status: 500 }));

  await assertRequestError(
    organization.fetchOrganizationTeacherApplications({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "server_error",
      errorClass: organization.OrganizationRequestError,
      status: 500,
    },
  );

  mockFetch((_url, init) => {
    return new Promise((_resolve, reject) => {
      init.signal.addEventListener("abort", () => {
        reject(new DOMException("The operation was aborted.", "AbortError"));
      });
    });
  });

  await assertRequestError(
    organization.fetchOrganizationTeacherApplications({
      organizationId: 7,
      timeoutMs: 1,
      token: "org-token",
    }),
    {
      code: "timeout",
      errorClass: organization.OrganizationRequestError,
      status: 0,
    },
  );

  mockFetch(() => {
    throw new TypeError("fetch failed");
  });

  await assertRequestError(
    organization.fetchOrganizationTeacherApplications({
      organizationId: 7,
      token: "org-token",
    }),
    {
      code: "network_error",
      errorClass: organization.OrganizationRequestError,
      status: 0,
    },
  );
});

test("loginWithPassword parses JSON token success", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(url, "/api/auth/login");
    assert.equal(init.method, "POST");
    assert.equal(init.headers["Content-Type"], "application/json");
    assert.deepEqual(JSON.parse(init.body), {
      email: "learner@example.test",
      password: "correct-password",
    });
    return jsonResponse("jwt-token");
  });

  const token = await auth.loginWithPassword({
    email: "learner@example.test",
    password: "correct-password",
  });

  assert.equal(calls.length, 1);
  assert.equal(token, "jwt-token");
});

test("loginWithPassword normalizes text 401, 403, and 500 errors", async () => {
  mockFetch(() => textResponse("Invalid credentials", { status: 401 }));

  await assertRequestError(
    auth.loginWithPassword({ email: "bad@example.test", password: "bad" }),
    {
      code: "invalid_credentials",
      errorClass: auth.AuthRequestError,
      status: 401,
    },
  );

  mockFetch(() => textResponse("Email verification required", { status: 403 }));

  await assertRequestError(
    auth.loginWithPassword({ email: "pending@example.test", password: "password" }),
    {
      code: "unverified_email",
      errorClass: auth.AuthRequestError,
      status: 403,
    },
  );

  mockFetch(() => textResponse("Database unavailable", { status: 500 }));

  await assertRequestError(
    auth.loginWithPassword({ email: "learner@example.test", password: "password" }),
    {
      code: "server_error",
      errorClass: auth.AuthRequestError,
      status: 500,
    },
  );
});

test("verifyEmailToken separates invalid and expired text errors", async () => {
  mockFetch(() => textResponse("Invalid verification token", { status: 400 }));

  await assertRequestError(auth.verifyEmailToken({ token: "invalid-token" }), {
    code: "invalid_token",
    errorClass: auth.AuthRequestError,
    status: 400,
  });

  mockFetch(() => textResponse("Verification token expired", { status: 400 }));

  await assertRequestError(auth.verifyEmailToken({ token: "expired-token" }), {
    code: "expired_token",
    errorClass: auth.AuthRequestError,
    status: 400,
  });
});

test("fetchRewardHistory parses learner reward filters and JSON success", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(url, "/api/reward-candidates/me/history?limit=10&status=wallet_credited");
    assert.equal(init.headers.Authorization, "Bearer learner-token");
    return jsonResponse([
      {
        approved_amount: "12",
        course_id: 7,
        course_title: "Rust Ownership",
        created_at: "2026-01-01T10:00:00Z",
        event_type: "course_completion",
        reward_candidate_id: 99,
        status: "wallet_credited",
        token_transaction: {
          amount: "12",
          blockchain_address: "0xlearner",
          chain_id: 31337,
          external_transaction_id: 5,
          payout_transaction_id: 4,
          recorded_at: "2026-01-02T10:00:00Z",
          transaction_hash: "0xtxhash",
        },
        updated_at: "2026-01-02T10:00:00Z",
        wallet_credit: {
          amount: "12",
          credited_at: "2026-01-02T10:10:00Z",
          internal_transaction_id: 8,
          reward_wallet_credit_record_id: 6,
          transaction_id: 7,
          wallet_id: 3,
        },
      },
    ]);
  });

  const history = await learner.fetchRewardHistory({
    limit: 10,
    status: "wallet_credited",
    token: "learner-token",
  });

  assert.equal(calls.length, 1);
  assert.equal(history.length, 1);
  assert.equal(history[0].course_title, "Rust Ownership");
  assert.equal(history[0].wallet_credit.amount, "12");
});

test("fetchCourseCatalog sends learner filters and parses summaries", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(
      url,
      "/api/courses/catalog?limit=5&reward_available=true&search=Rust&enrollment_status=available",
    );
    assert.equal(init.headers.Authorization, "Bearer learner-token");
    return jsonResponse({
      courses: [courseCatalogItemFixture()],
      enrollment_status: "available",
      lifecycle_status: null,
      limit: 5,
      offset: 0,
      organization_id: null,
      reward_available: true,
      search: "Rust",
      total: 1,
    });
  });

  const catalog = await learner.fetchCourseCatalog({
    enrollmentStatus: "available",
    limit: 5,
    rewardAvailable: true,
    search: " Rust ",
    token: "learner-token",
  });

  assert.equal(calls.length, 1);
  assert.equal(catalog.total, 1);
  assert.equal(catalog.courses[0].title, "Rust Ownership");
  assert.equal(catalog.courses[0].enrollment.state, "available");
});

test("fetchLearnerDashboard aggregates learner-safe route data", async () => {
  const enrolledCourse = {
    ...courseCatalogItemFixture(),
    enrollment: {
      ...courseCatalogItemFixture().enrollment,
      can_request_join: false,
      reason: "You are enrolled.",
      roles: ["STUDENT"],
      state: "enrolled",
    },
  };
  const calls = mockFetch((url, init) => {
    assert.equal(init.headers.Authorization, "Bearer learner-token");
    if (url === "/api/courses/catalog?limit=6&enrollment_status=enrolled") {
      return jsonResponse({
        courses: [enrolledCourse],
        enrollment_status: "enrolled",
        lifecycle_status: null,
        limit: 6,
        offset: 0,
        organization_id: null,
        reward_available: null,
        search: null,
        total: 1,
      });
    }
    if (url === "/api/courses/catalog?limit=3&enrollment_status=available") {
      return jsonResponse({
        courses: [courseCatalogItemFixture()],
        enrollment_status: "available",
        lifecycle_status: null,
        limit: 3,
        offset: 0,
        organization_id: null,
        reward_available: null,
        search: null,
        total: 1,
      });
    }
    if (url === "/api/reward-candidates/me/history?limit=6") {
      return jsonResponse([
        {
          approved_amount: null,
          course_id: 7,
          course_title: "Rust Ownership",
          created_at: "2026-01-01T10:00:00Z",
          event_type: "course_completion",
          reward_candidate_id: 101,
          status: "pending_teacher_approval",
          token_transaction: null,
          updated_at: "2026-01-01T10:00:00Z",
          wallet_credit: null,
        },
      ]);
    }
    if (url === "/api/wallets/me") {
      return textResponse("Wallet not linked", { status: 404 });
    }
    throw new Error(`Unexpected URL ${url}`);
  });

  const dashboard = await learner.fetchLearnerDashboard({ token: "learner-token" });

  assert.equal(calls.length, 4);
  assert.equal(dashboard.enrolled_catalog.total, 1);
  assert.equal(dashboard.enrolled_catalog.courses[0].enrollment.state, "enrolled");
  assert.equal(dashboard.recommended_catalog.courses[0].enrollment.state, "available");
  assert.equal(dashboard.reward_history[0].status, "pending_teacher_approval");
  assert.equal(dashboard.wallet, null);
});

test("fetchCourseDetail and requestCourseJoin use learner course routes", async () => {
  mockFetch((url, init) => {
    assert.equal(url, "/api/courses/catalog/7");
    assert.equal(init.method, "GET");
    assert.equal(init.headers.Authorization, "Bearer learner-token");
    return jsonResponse({
      chapters: [
        {
          contents: [{ content_type: "video", id: 3, order: 0 }],
          id: 2,
          order: 0,
          title: "Intro",
        },
      ],
      course: courseCatalogItemFixture(),
      prerequisites: [],
    });
  });

  const detail = await learner.fetchCourseDetail({ courseId: 7, token: "learner-token" });
  assert.equal(detail.course.id, 7);
  assert.equal(detail.chapters[0].contents[0].content_type, "video");

  const calls = mockFetch((url, init) => {
    assert.equal(url, "/api/courses/7/join-requests");
    assert.equal(init.method, "POST");
    assert.equal(init.headers.Authorization, "Bearer learner-token");
    return jsonResponse(
      {
        course_id: 7,
        created_at: "2026-01-01T10:00:00Z",
        decided_at: null,
        decision_reason: null,
        id: 44,
        requester_user_id: 1,
        reviewer_user_id: null,
        status: "pending",
        updated_at: "2026-01-01T10:00:00Z",
      },
      { status: 201 },
    );
  });

  const joinRequest = await learner.requestCourseJoin({ courseId: 7, token: "learner-token" });
  assert.equal(calls.length, 1);
  assert.equal(joinRequest.status, "pending");
});

test("fetchCourseLearning parses lesson content states", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(url, "/api/courses/catalog/7/learn");
    assert.equal(init.headers.Authorization, "Bearer learner-token");
    return jsonResponse({
      active_content_id: 3,
      chapters: [
        {
          contents: [
            {
              chapter_id: 2,
              content_type: "text",
              data: "Welcome to ownership.",
              display_state: "ready",
              id: 3,
              order: 0,
              processing_error: null,
              processing_status: null,
            },
            {
              chapter_id: 2,
              content_type: "video",
              data: "courses/7/chapters/2/video.mp4",
              display_state: "failed_processing",
              id: 4,
              order: 1,
              processing_error: "ffmpeg failed",
              processing_status: "failed",
            },
          ],
          id: 2,
          order: 0,
          title: "Intro",
        },
      ],
      course: courseCatalogItemFixture(),
      progress_supported: false,
    });
  });

  const learning = await learner.fetchCourseLearning({ courseId: 7, token: "learner-token" });

  assert.equal(calls.length, 1);
  assert.equal(learning.active_content_id, 3);
  assert.equal(learning.chapters[0].contents[0].display_state, "ready");
  assert.equal(learning.chapters[0].contents[1].processing_error, "ffmpeg failed");
  assert.equal(learning.progress_supported, false);
});

test("fetchCourseLearning normalizes content permission text errors", async () => {
  mockFetch(() => textResponse("User does not have permission to view course content", { status: 403 }));

  await assertRequestError(learner.fetchCourseLearning({ courseId: 7, token: "learner-token" }), {
    code: "permission_denied",
    errorClass: learner.LearnerRequestError,
    status: 403,
  });
});

test("fetchCourseDetail normalizes learner text not found errors", async () => {
  mockFetch(() => textResponse("Course not found", { status: 404 }));

  await assertRequestError(learner.fetchCourseDetail({ courseId: 404, token: "learner-token" }), {
    code: "not_found",
    errorClass: learner.LearnerRequestError,
    status: 404,
  });
});

test("fetchMyWallet returns null for unlinked wallet and parses linked wallet", async () => {
  mockFetch(() => textResponse("Wallet not linked", { status: 404 }));

  const missingWallet = await learner.fetchMyWallet({ token: "learner-token" });
  assert.equal(missingWallet, null);

  mockFetch((url, init) => {
    assert.equal(url, "/api/wallets/me");
    assert.equal(init.headers.Authorization, "Bearer learner-token");
    return jsonResponse({
      id: 3,
      organization_id: null,
      owner_type: "user",
      user_id: 1,
      value: "42",
    });
  });

  const wallet = await learner.fetchMyWallet({ token: "learner-token" });
  assert.equal(wallet.id, 3);
  assert.equal(wallet.value, "42");
});

test("fetchMyWallet normalizes plain text backend failures", async () => {
  mockFetch(() => textResponse("Wallet database unavailable", { status: 500 }));

  await assertRequestError(learner.fetchMyWallet({ token: "learner-token" }), {
    code: "server_error",
    errorClass: learner.LearnerRequestError,
    status: 500,
  });
});

test("fetchLearnerWallet aggregates wallet summary and reward credit history", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(init.headers.Authorization, "Bearer learner-token");
    if (url === "/api/wallets/me") {
      return jsonResponse({
        id: 3,
        organization_id: null,
        owner_type: "user",
        user_id: 1,
        value: "42",
      });
    }
    if (url === "/api/reward-candidates/me/history?limit=12") {
      return jsonResponse([
        {
          approved_amount: "12",
          course_id: 7,
          course_title: "Rust Ownership",
          created_at: "2026-01-01T10:00:00Z",
          event_type: "course_completion",
          reward_candidate_id: 201,
          status: "wallet_credited",
          token_transaction: null,
          updated_at: "2026-01-02T10:00:00Z",
          wallet_credit: {
            amount: "12",
            credited_at: "2026-01-02T10:10:00Z",
            internal_transaction_id: 8,
            reward_wallet_credit_record_id: 6,
            transaction_id: 7,
            wallet_id: 3,
          },
        },
      ]);
    }
    throw new Error(`Unexpected URL ${url}`);
  });

  const snapshot = await learner.fetchLearnerWallet({ token: "learner-token" });

  assert.equal(calls.length, 2);
  assert.equal(snapshot.wallet.value, "42");
  assert.equal(snapshot.reward_history[0].wallet_credit.amount, "12");
});

test("linkMyWallet parses created response and learner text errors", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(url, "/api/wallets/me/link");
    assert.equal(init.method, "POST");
    return jsonResponse(
      {
        created: true,
        wallet: {
          id: 4,
          organization_id: null,
          owner_type: "user",
          user_id: 1,
          value: "0",
        },
      },
      { status: 201 },
    );
  });

  const result = await learner.linkMyWallet({ token: "learner-token" });
  assert.equal(calls.length, 1);
  assert.equal(result.created, true);
  assert.equal(result.wallet.id, 4);

  mockFetch(() => textResponse("User does not have wallet access", { status: 403 }));

  await assertRequestError(learner.linkMyWallet({ token: "learner-token" }), {
    code: "permission_denied",
    errorClass: learner.LearnerRequestError,
    status: 403,
  });
});

test("fetchMyTeacherApplication parses current-user application snapshot", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(url, "/api/teacher-applications/me");
    assert.equal(init.headers.Authorization, "Bearer teacher-token");
    return jsonResponse(teacherApplicationSnapshotFixture());
  });

  const snapshot = await teacher.fetchMyTeacherApplication({ token: "teacher-token" });

  assert.equal(calls.length, 1);
  assert.equal(snapshot.application.status, "submitted");
  assert.equal(snapshot.application.portfolio_links[0], "https://example.test/portfolio");
  assert.equal(snapshot.audit_events.length, 1);

  mockFetch(() => jsonResponse({ application: null, audit_events: [] }));

  const emptySnapshot = await teacher.fetchMyTeacherApplication({ token: "teacher-token" });
  assert.equal(emptySnapshot.application, null);
  assert.deepEqual(emptySnapshot.audit_events, []);
});

test("fetchTeachingCourses sends filters and parses teaching dashboard summaries", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(url, "/api/courses/teaching?limit=5&lifecycle_status=published&search=Rust");
    assert.equal(init.headers.Authorization, "Bearer teacher-token");
    return jsonResponse(teacherCoursesFixture());
  });

  const dashboard = await teacher.fetchTeachingCourses({
    lifecycleStatus: "published",
    limit: 5,
    search: " Rust ",
    token: "teacher-token",
  });

  assert.equal(calls.length, 1);
  assert.equal(dashboard.total, 1);
  assert.equal(dashboard.courses[0].title, "Rust Ownership Lab");
  assert.equal(dashboard.courses[0].roster.pending_join_request_count, 2);
  assert.equal(dashboard.courses[0].reward_queue.pending_teacher_count, 1);
  assert.equal(dashboard.courses[0].permissions.can_approve_reward_candidates, true);
});

test("fetchTeachingCourses normalizes text backend failures", async () => {
  mockFetch(() => textResponse("Failed to load teaching courses", { status: 500 }));

  await assertRequestError(teacher.fetchTeachingCourses({ token: "teacher-token" }), {
    code: "server_error",
    errorClass: teacher.TeacherRequestError,
    status: 500,
  });
});

test("fetchTeachingCourseWorkspace parses structured authoring detail", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(url, "/api/courses/teaching/9");
    assert.equal(init.headers.Authorization, "Bearer teacher-token");
    return jsonResponse(teacherCourseWorkspaceFixture());
  });

  const workspace = await teacher.fetchTeachingCourseWorkspace({
    courseId: 9,
    token: "teacher-token",
  });

  assert.equal(calls.length, 1);
  assert.equal(workspace.course.title, "Rust Ownership Lab");
  assert.equal(workspace.teacher_roles[0], "TEACHER");
  assert.equal(workspace.publication.content_publication_status_supported, false);
  assert.equal(workspace.chapters[0].contents[0].display_state, "ready");
  assert.equal(workspace.chapters[0].contents[1].processing_status, "failed");
});

test("fetchTeachingCourseWorkspace normalizes permission and missing-course text errors", async () => {
  mockFetch(() => textResponse("User does not have permission to view this teaching course", { status: 403 }));

  await assertRequestError(teacher.fetchTeachingCourseWorkspace({ courseId: 10, token: "teacher-token" }), {
    code: "permission_denied",
    errorClass: teacher.TeacherRequestError,
    status: 403,
  });

  mockFetch(() => textResponse("Course not found", { status: 404 }));

  await assertRequestError(teacher.fetchTeachingCourseWorkspace({ courseId: 404, token: "teacher-token" }), {
    code: "not_found",
    errorClass: teacher.TeacherRequestError,
    status: 404,
  });
});

test("teacher enrollment helpers fetch queue, decide requests, and remove learners", async () => {
  const calls = mockFetch((url, init) => {
    if (url === "/api/courses/teaching/9/enrollments?limit=10&status=open") {
      assert.equal(init.method, "GET");
      assert.equal(init.headers.Authorization, "Bearer teacher-token");
      return jsonResponse(teacherCourseEnrollmentFixture());
    }

    if (url === "/api/courses/9/join-requests/55/decision") {
      assert.equal(init.method, "PUT");
      assert.equal(init.headers.Authorization, "Bearer teacher-token");
      assert.equal(init.headers["Content-Type"], "application/json");
      assert.deepEqual(JSON.parse(init.body), {
        decision_reason: "Welcome to the cohort.",
        status: "approved",
      });
      return jsonResponse({
        course_id: 9,
        created_at: "2026-01-02T10:00:00Z",
        decided_at: "2026-01-02T11:00:00Z",
        decision_reason: "Welcome to the cohort.",
        id: 55,
        requester_user_id: 88,
        reviewer_user_id: 42,
        status: "approved",
        updated_at: "2026-01-02T11:00:00Z",
      });
    }

    assert.equal(url, "/api/courses/9/enrollments/77");
    assert.equal(init.method, "DELETE");
    assert.equal(init.headers.Authorization, "Bearer teacher-token");
    return jsonResponse({
      course_id: 9,
      removed: true,
      user_id: 77,
    });
  });

  const enrollments = await teacher.fetchTeachingCourseEnrollments({
    courseId: 9,
    limit: 10,
    status: "open",
    token: "teacher-token",
  });
  const decision = await teacher.decideTeacherJoinRequest({
    courseId: 9,
    payload: {
      decision_reason: "Welcome to the cohort.",
      status: "approved",
    },
    requestId: 55,
    token: "teacher-token",
  });
  const removal = await teacher.removeTeacherEnrollment({
    courseId: 9,
    token: "teacher-token",
    userId: 77,
  });

  assert.equal(calls.length, 3);
  assert.equal(enrollments.join_requests.total, 2);
  assert.equal(enrollments.join_requests.requests[0].requester.name, "Ada Learner");
  assert.equal(enrollments.roster.learners[0].latest_join_request_status, "approved");
  assert.equal(enrollments.progress_supported, false);
  assert.equal(decision.status, "approved");
  assert.equal(removal.removed, true);
});

test("teacher enrollment helpers normalize permission and missing enrollment errors", async () => {
  mockFetch(() => textResponse("User does not have permission to view this teaching course", { status: 403 }));

  await assertRequestError(
    teacher.fetchTeachingCourseEnrollments({
      courseId: 9,
      token: "teacher-token",
    }),
    {
      code: "permission_denied",
      errorClass: teacher.TeacherRequestError,
      status: 403,
    },
  );

  mockFetch(() => textResponse("Course enrollment not found", { status: 404 }));

  await assertRequestError(
    teacher.removeTeacherEnrollment({
      courseId: 9,
      token: "teacher-token",
      userId: 99,
    }),
    {
      code: "not_found",
      errorClass: teacher.TeacherRequestError,
      status: 404,
    },
  );
});

test("fetchTeachingCourseStudents parses unsupported progress and reward evidence", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(url, "/api/courses/teaching/9/students");
    assert.equal(init.method, "GET");
    assert.equal(init.headers.Authorization, "Bearer teacher-token");
    return jsonResponse(teacherCourseStudentsFixture());
  });

  const students = await teacher.fetchTeachingCourseStudents({
    courseId: 9,
    token: "teacher-token",
  });

  assert.equal(calls.length, 1);
  assert.equal(students.total, 1);
  assert.equal(students.progress_supported, false);
  assert.equal(students.reward_evidence_supported, true);
  assert.equal(students.students[0].user.name, "Linus Learner");
  assert.equal(students.students[0].progress.supported, false);
  assert.equal(students.students[0].progress.total_content_count, 4);
  assert.equal(students.students[0].rewards.pending_teacher_count, 1);
  assert.equal(students.students[0].rewards.latest_candidate.status, "pending_teacher_approval");
});

test("fetchTeachingCourseStudents normalizes permission and missing-course errors", async () => {
  mockFetch(() => textResponse("User does not have permission to view this teaching course", { status: 403 }));

  await assertRequestError(
    teacher.fetchTeachingCourseStudents({
      courseId: 9,
      token: "teacher-token",
    }),
    {
      code: "permission_denied",
      errorClass: teacher.TeacherRequestError,
      status: 403,
    },
  );

  mockFetch(() => textResponse("Course not found", { status: 404 }));

  await assertRequestError(
    teacher.fetchTeachingCourseStudents({
      courseId: 404,
      token: "teacher-token",
    }),
    {
      code: "not_found",
      errorClass: teacher.TeacherRequestError,
      status: 404,
    },
  );
});

test("teacher reward helpers fetch filtered candidates and send teacher decisions only", async () => {
  const calls = mockFetch((url, init) => {
    if (url === "/api/courses/9/reward-candidates?limit=10&status=pending_teacher_approval") {
      assert.equal(init.method, "GET");
      assert.equal(init.headers.Authorization, "Bearer teacher-token");
      return jsonResponse(teacherRewardCandidatesFixture());
    }

    assert.equal(url, "/api/courses/9/reward-candidates/71/teacher-decision");
    assert.equal(init.method, "PUT");
    assert.equal(init.headers.Authorization, "Bearer teacher-token");
    assert.equal(init.headers["Content-Type"], "application/json");
    assert.deepEqual(JSON.parse(init.body), {
      decision_reason: "Evidence matches the course policy.",
      status: "teacher_approved",
    });
    return jsonResponse({
      ...teacherRewardCandidatesFixture()[0],
      status: "teacher_approved",
      teacher_decided_at: "2026-01-04T11:00:00Z",
      teacher_decision_reason: "Evidence matches the course policy.",
    });
  });

  const candidates = await teacher.fetchTeacherRewardCandidates({
    courseId: 9,
    limit: 10,
    status: "pending_teacher_approval",
    token: "teacher-token",
  });
  const decision = await teacher.decideTeacherRewardCandidate({
    candidateId: 71,
    courseId: 9,
    payload: {
      decision_reason: "Evidence matches the course policy.",
      status: "teacher_approved",
    },
    token: "teacher-token",
  });

  assert.equal(calls.length, 2);
  assert.equal(candidates.length, 2);
  assert.equal(candidates[0].status, "pending_teacher_approval");
  assert.deepEqual(candidates[0].evidence, { completion_percentage: 100, lesson_id: 14 });
  assert.equal(decision.status, "teacher_approved");
  assert.equal(decision.teacher_decision_reason, "Evidence matches the course policy.");
});

test("teacher reward helpers normalize permission, conflict, and missing-candidate errors", async () => {
  mockFetch(() => textResponse("User does not have reward candidate permission", { status: 403 }));

  await assertRequestError(
    teacher.fetchTeacherRewardCandidates({
      courseId: 9,
      token: "teacher-token",
    }),
    {
      code: "permission_denied",
      errorClass: teacher.TeacherRequestError,
      status: 403,
    },
  );

  mockFetch(() => textResponse("reward candidate has already left teacher approval", { status: 409 }));

  await assertRequestError(
    teacher.decideTeacherRewardCandidate({
      candidateId: 71,
      courseId: 9,
      payload: {
        decision_reason: null,
        status: "teacher_rejected",
      },
      token: "teacher-token",
    }),
    {
      code: "conflict",
      errorClass: teacher.TeacherRequestError,
      status: 409,
    },
  );

  mockFetch(() => textResponse("Reward candidate not found", { status: 404 }));

  await assertRequestError(
    teacher.decideTeacherRewardCandidate({
      candidateId: 404,
      courseId: 9,
      payload: {
        decision_reason: "No matching evidence.",
        status: "teacher_rejected",
      },
      token: "teacher-token",
    }),
    {
      code: "not_found",
      errorClass: teacher.TeacherRequestError,
      status: 404,
    },
  );
});

test("teacher chapter and content authoring helpers send structured JSON", async () => {
  const calls = mockFetch((url, init) => {
    if (url === "/api/courses/9/chapters") {
      assert.equal(init.method, "POST");
      assert.equal(init.headers.Authorization, "Bearer teacher-token");
      assert.equal(init.headers["Content-Type"], "application/json");
      assert.deepEqual(JSON.parse(init.body), {
        order: 3,
        title: "Ownership practice",
      });
      return jsonResponse({
        course_id: 9,
        id: 15,
        order: 3,
        title: "Ownership practice",
      }, { status: 201 });
    }

    assert.equal(url, "/api/courses/9/chapters/15/contents");
    assert.equal(init.method, "POST");
    assert.equal(init.headers.Authorization, "Bearer teacher-token");
    assert.equal(init.headers["Content-Type"], "application/json");
    assert.deepEqual(JSON.parse(init.body), {
      content_type: "article",
      data: "Lesson body",
      order: 1,
    });
    return jsonResponse({
      chapter_id: 15,
      content_type: "article",
      data: "Lesson body",
      id: 22,
      order: 1,
    }, { status: 201 });
  });

  const chapter = await teacher.createTeacherChapter({
    courseId: 9,
    payload: {
      order: 3,
      title: "Ownership practice",
    },
    token: "teacher-token",
  });
  const content = await teacher.createTeacherContent({
    chapterId: chapter.id,
    courseId: 9,
    payload: {
      content_type: "article",
      data: "Lesson body",
      order: 1,
    },
    token: "teacher-token",
  });

  assert.equal(calls.length, 2);
  assert.equal(chapter.title, "Ownership practice");
  assert.equal(content.data, "Lesson body");
});

test("teacher authoring helpers normalize permission and invalid chapter errors", async () => {
  mockFetch(() => textResponse("User does not have permission to access this course", { status: 403 }));

  await assertRequestError(
    teacher.createTeacherChapter({
      courseId: 9,
      payload: { order: 1, title: "Blocked" },
      token: "teacher-token",
    }),
    {
      code: "permission_denied",
      errorClass: teacher.TeacherRequestError,
      status: 403,
    },
  );

  mockFetch(() => textResponse("Chapter not found", { status: 404 }));

  await assertRequestError(
    teacher.createTeacherContent({
      chapterId: 99,
      courseId: 9,
      payload: { content_type: "article", data: "Nope", order: 1 },
      token: "teacher-token",
    }),
    {
      code: "not_found",
      errorClass: teacher.TeacherRequestError,
      status: 404,
    },
  );
});

test("submitTeacherApplication sends JSON payload and parses created application", async () => {
  const payload = {
    experience_summary: "I teach Rust fundamentals and review project work.",
    idempotency_key: "teacher-application-retry-key",
    portfolio_links: ["https://example.test/portfolio"],
    requested_scope: "platform",
  };
  const calls = mockFetch((url, init) => {
    assert.equal(url, "/api/teacher-applications");
    assert.equal(init.method, "POST");
    assert.equal(init.headers.Authorization, "Bearer teacher-token");
    assert.equal(init.headers["Content-Type"], "application/json");
    assert.deepEqual(JSON.parse(init.body), payload);
    return jsonResponse(teacherApplicationSnapshotFixture().application, { status: 201 });
  });

  const application = await teacher.submitTeacherApplication({
    payload,
    token: "teacher-token",
  });

  assert.equal(calls.length, 1);
  assert.equal(application.id, 44);
  assert.equal(application.requested_scope, "platform");
});

test("teacher helpers normalize conflict, server, timeout, and network failures", async () => {
  mockFetch(() => textResponse("teacher application already exists with status submitted", { status: 409 }));

  await assertRequestError(
    teacher.submitTeacherApplication({
      payload: {
        experience_summary: "Duplicate submission",
        requested_scope: "platform",
      },
      token: "teacher-token",
    }),
    {
      code: "conflict",
      errorClass: teacher.TeacherRequestError,
      status: 409,
    },
  );

  mockFetch(() => textResponse("Failed to process teacher application", { status: 500 }));

  await assertRequestError(teacher.fetchMyTeacherApplication({ token: "teacher-token" }), {
    code: "server_error",
    errorClass: teacher.TeacherRequestError,
    status: 500,
  });

  mockFetch((_url, init) => {
    return new Promise((_resolve, reject) => {
      init.signal.addEventListener("abort", () => {
        reject(new DOMException("The operation was aborted.", "AbortError"));
      });
    });
  });

  await assertRequestError(teacher.fetchMyTeacherApplication({ token: "slow-token", timeoutMs: 1 }), {
    code: "timeout",
    errorClass: teacher.TeacherRequestError,
    status: 0,
  });

  mockFetch(() => {
    throw new TypeError("fetch failed");
  });

  await assertRequestError(teacher.fetchMyTeacherApplication({ token: "network-token" }), {
    code: "network_error",
    errorClass: teacher.TeacherRequestError,
    status: 0,
  });
});

async function importTranspiled(relativePath) {
  const sourcePath = path.join(repoRoot, relativePath);
  const source = readFileSync(sourcePath, "utf8");
  const output = ts.transpileModule(source, {
    compilerOptions: {
      module: ts.ModuleKind.ES2022,
      target: ts.ScriptTarget.ES2022,
    },
    fileName: sourcePath,
  });
  const outputPath = path.join(compiledDir, relativePath.replaceAll("/", "__").replace(/\.ts$/, ".mjs"));
  writeFileSync(outputPath, output.outputText);
  return import(pathToFileURL(outputPath));
}

function mockFetch(handler) {
  const calls = [];
  globalThis.fetch = async (url, init = {}) => {
    calls.push({ init, url });
    return handler(url, init);
  };
  return calls;
}

function jsonResponse(body, { status = 200 } = {}) {
  return new Response(JSON.stringify(body), {
    headers: { "content-type": "application/json" },
    status,
  });
}

function textResponse(body, { headers = {}, status = 200 } = {}) {
  return new Response(body, {
    headers: { "content-type": "text/plain", ...headers },
    status,
  });
}

async function assertRequestError(promise, { code, errorClass, status }) {
  await assert.rejects(
    promise,
    (error) =>
      error instanceof errorClass &&
      error.code === code &&
      error.status === status &&
      typeof error.message === "string" &&
      error.message.length > 0,
  );
}

function currentSessionFixture() {
  return {
    user: {
      id: 1,
      name: "Learner One",
      email: "learner@example.test",
      email_verified: true,
      kyc_verified: false,
    },
    platform: {
      roles: ["LEARNER"],
      direct_permissions: ["VIEW_REPORT"],
      delegated_permissions: [],
      effective_permissions: ["VIEW_REPORT"],
    },
    organizations: [],
    courses: [],
    delegated_permissions: [],
  };
}

function platformAdminSessionFixture() {
  return {
    ...currentSessionFixture(),
    platform: {
      roles: ["PLATFORM_ADMIN"],
      direct_permissions: [
        "APPROVE_REWARD_AMOUNT",
        "EXPORT_DATA",
        "MANAGE_REWARD_FRAUD_BLOCKS",
        "VIEW_REPORT",
        "VIEW_REWARD_AUDIT",
        "VIEW_TRANSACTIONS",
      ],
      delegated_permissions: ["VIEW_ROLE_ASSIGNMENTS"],
      effective_permissions: [
        "APPROVE_REWARD_AMOUNT",
        "EXPORT_DATA",
        "MANAGE_REWARD_FRAUD_BLOCKS",
        "VIEW_REPORT",
        "VIEW_REWARD_AUDIT",
        "VIEW_ROLE_ASSIGNMENTS",
        "VIEW_TRANSACTIONS",
      ],
    },
  };
}

function platformSummaryFixture() {
  return {
    total_courses: 6,
    total_notifications: 14,
    total_organizations: 3,
    total_users: 12,
    total_wallets: 4,
  };
}

function platformRewardDashboardFixture() {
  return {
    payout_failure_count: 1,
    payout_failures: [
      {
        attempts: 3,
        last_error: "transaction reverted",
        reward_candidate_id: 203,
        reward_execution_job_id: 31,
        status: "failed",
        updated_at: "2026-01-06T10:00:00Z",
      },
    ],
    pending_amount_approval_count: 1,
    pending_amount_approvals: [
      {
        approved_amount: null,
        course_id: 9,
        event_type: "course_completion",
        reward_candidate_id: 201,
        source_organization_id: 7,
        status: "teacher_approved",
        student_user_id: 77,
        submitter_user_id: 42,
        updated_at: "2026-01-05T10:00:00Z",
      },
    ],
    reconciliation_mismatch_count: 1,
    reconciliation_mismatches: [
      {
        approved_amount: "12",
        course_id: 9,
        mismatch_type: "needs_wallet_credit",
        reward_candidate_id: 202,
        status: "token_confirmed",
        student_user_id: 88,
        updated_at: "2026-01-06T11:00:00Z",
      },
    ],
    reward_candidates: {
      amount_approved: 2,
      amount_rejected: 0,
      completed: 1,
      failed: 1,
      needs_reconciliation: 1,
      notified: 0,
      pending_teacher_approval: 4,
      teacher_approved: 1,
      teacher_rejected: 0,
      token_confirmed: 1,
      token_pending: 1,
      total: 9,
      wallet_credited: 1,
    },
    teacher_applications: {
      approved: 2,
      needs_changes: 1,
      rejected: 0,
      submitted: 3,
      total: 6,
    },
  };
}

function platformFraudDashboardFixture() {
  return {
    active_blocks: [
      {
        course_id: null,
        created_at: "2026-01-04T10:00:00Z",
        created_by_user_id: 1,
        evidence_reference: "case-17",
        expires_at: null,
        id: 17,
        organization_id: 7,
        reason: "Suspicious reward burst",
        reward_policy_id: null,
        scope_type: "organization",
        teacher_user_id: null,
        updated_at: "2026-01-04T10:00:00Z",
      },
    ],
    active_by_scope: {
      course: 0,
      organization: 1,
      reward_policy: 0,
      teacher: 0,
    },
    active_total: 1,
  };
}

function organizationSessionFixture() {
  return {
    ...currentSessionFixture(),
    organizations: [
      {
        id: 7,
        name: "Ferris Academy",
        roles: ["ORG_ADMIN"],
        direct_permissions: [
          "INVITE_USER_TO_ORGANIZATION",
          "MANAGE_ORG_WALLETS",
          "VIEW_ORGANIZATION",
          "VIEW_ORG_REWARD_REPORTS",
        ],
        delegated_permissions: [],
        effective_permissions: [
          "INVITE_USER_TO_ORGANIZATION",
          "MANAGE_ORG_WALLETS",
          "VIEW_ORGANIZATION",
          "VIEW_ORG_REWARD_REPORTS",
        ],
      },
      {
        id: 8,
        name: "Rust Guild",
        roles: [],
        direct_permissions: ["SUBMIT_ORG_COURSE_REWARD_EVENT"],
        delegated_permissions: ["NOMINATE_TEACHER_FOR_PLATFORM_REVIEW"],
        effective_permissions: [
          "NOMINATE_TEACHER_FOR_PLATFORM_REVIEW",
          "SUBMIT_ORG_COURSE_REWARD_EVENT",
        ],
      },
      {
        id: 9,
        name: "Role Only Org",
        roles: ["ORG_MEMBER"],
        direct_permissions: [],
        delegated_permissions: [],
        effective_permissions: [],
      },
    ],
    delegated_permissions: [
      {
        course_id: null,
        course_lifecycle_status: null,
        course_title: null,
        expires_at: null,
        grantor_user_id: 1,
        id: 51,
        organization_id: 8,
        organization_name: "Rust Guild",
        permission: "NOMINATE_TEACHER_FOR_PLATFORM_REVIEW",
        scope_type: "organization",
      },
    ],
  };
}

function organizationDashboardFixture() {
  return {
    alerts: [
      {
        action_href: "/organizations/7/teacher-applications",
        action_label: "Open teacher nominations",
        kind: "teacher_applications_submitted",
        message: "2 sponsored teacher applications are awaiting platform review.",
        severity: "warning",
      },
    ],
    courses: {
      approved: 0,
      archived: 0,
      available: true,
      draft: 1,
      missing_permissions: [],
      needs_changes: 1,
      published: 1,
      submitted: 0,
      suspended: 0,
      total: 3,
    },
    health: {
      alert_count: 1,
      status: "attention",
    },
    members: {
      available: true,
      delegated_permission_count: 1,
      kyc_ready_count: 1,
      missing_permissions: [],
      total: 3,
      verified_email_count: 2,
    },
    operator_permissions: {
      can_manage_reward_budget: true,
      can_manage_wallets: true,
      can_nominate_teachers: true,
      can_view_courses: true,
      can_view_dashboard: true,
      can_view_members: true,
      can_view_reports: true,
      can_view_teacher_applications: true,
    },
    organization: {
      id: 7,
      name: "Ferris Academy",
    },
    rewards: {
      approved_amount_total: "40",
      approved_reward_count: 2,
      available: true,
      failed_count: 1,
      missing_permissions: [],
      needs_reconciliation_count: 0,
      reward_candidate_count: 4,
    },
    teacher_applications: {
      approved: 1,
      available: true,
      missing_permissions: [],
      needs_changes: 0,
      rejected: 0,
      submitted: 2,
      total: 3,
    },
    wallet: {
      available: true,
      balance_total: "125",
      missing_permissions: [],
      wallet_count: 1,
    },
  };
}

function organizationRewardDashboardFixture() {
  return {
    approved_amount_total: "40",
    approved_reward_count: 2,
    course_reward_count: 3,
    courses: [
      {
        approved_amount_total: "40",
        approved_reward_count: 2,
        course_id: 9,
        course_title: "Rust Ownership Lab",
        reward_candidate_count: 3,
      },
    ],
    organization_id: 7,
    organization_name: "Ferris Academy",
    sponsored_teacher_applications: {
      approved: 1,
      needs_changes: 0,
      rejected: 0,
      submitted: 2,
      total: 3,
    },
    wallet_balance_total: "125",
    wallets: [
      {
        balance: "125",
        wallet_id: 4,
      },
    ],
  };
}

function organizationWalletAuditFixture() {
  return {
    compensation_records: [
      {
        amount: "5",
        created_at: "2026-06-09T12:00:00Z",
        created_by_user_id: 1,
        id: 12,
        idempotency_key: "manual-reconciliation-12",
        internal_transaction_id: 30,
        reason: "Manual reconciliation",
        reward_candidate_id: 91,
        transaction_id: 29,
        wallet_id: 4,
      },
    ],
    external_transactions: [
      {
        amount: "40",
        blockchain_address: "0xlearn",
        chain_id: 31337,
        contract_address: "0xtoken",
        event_type: "reward_payout",
        external_transaction_id: 20,
        from_address: "0xtreasury",
        log_index: 1,
        reward_candidate_id: 91,
        to_address: "0xstudent",
        transaction_hash: "0xabc123456789def0abc123456789def0abc12345",
        transaction_id: 19,
      },
    ],
    internal_transactions: [
      {
        amount: "125",
        created_at: "2026-06-09T10:00:00Z",
        internal_transaction_id: 18,
        transaction_id: 17,
        transaction_type: "organization_budget_adjustment",
      },
    ],
    reward_records: [
      {
        approved_amount: "40",
        candidate_status: "token_confirmed",
        created_at: "2026-06-09T09:00:00Z",
        external_transaction_id: 20,
        internal_transaction_id: null,
        notification_id: null,
        notified_at: null,
        payout_record_id: 21,
        payout_transaction_id: 19,
        reconciliation_status: "needs_wallet_credit",
        reward_candidate_id: 91,
        updated_at: "2026-06-09T11:00:00Z",
        wallet_credit_record_id: null,
        wallet_credit_transaction_id: null,
      },
    ],
    wallet: {
      id: 4,
      organization_id: 7,
      owner_type: "organization",
      user_id: null,
      value: "125",
    },
  };
}

function organizationCoursesFixture() {
  return {
    courses: [
      {
        content: {
          chapter_count: 2,
          content_count: 4,
          content_types: ["article", "video"],
          has_content: true,
        },
        id: 9,
        lifecycle_status: "published",
        permissions: {
          can_create_courses: true,
          can_manage_course_settings: true,
          can_manage_enrollments: true,
          can_manage_reward_budget: true,
          can_submit_reward_events: true,
          can_view_courses: true,
          can_view_reward_reports: true,
        },
        reward_queue: {
          failed_count: 0,
          pending_teacher_count: 1,
          teacher_approved_count: 2,
        },
        rewards: {
          active_policy_count: 1,
          available: true,
          event_types: ["manual_completion"],
          payment_strategies: ["mint"],
          token_amounts: ["25"],
        },
        roster: {
          enrolled_student_count: 8,
          pending_join_request_count: 2,
          waitlisted_join_request_count: 0,
        },
        teachers: [{ id: 4, name: "Ada Teacher" }],
        title: "Rust Ownership Lab",
      },
    ],
    lifecycle_status: "published",
    limit: 6,
    offset: 6,
    organization: {
      id: 7,
      name: "Ferris Academy",
    },
    reward_available: true,
    search: "rust",
    total: 1,
  };
}

function organizationMembersFixture() {
  return {
    limit: 8,
    members: [
      {
        delegated_permission_count: 1,
        delegated_permissions: ["VIEW_ORG_REWARD_REPORTS"],
        direct_permission_count: 3,
        direct_permissions: [
          "INVITE_USER_TO_ORGANIZATION",
          "MANAGE_ORG_MEMBERS",
          "VIEW_ORGANIZATION",
        ],
        effective_permission_count: 4,
        effective_permissions: [
          "INVITE_USER_TO_ORGANIZATION",
          "MANAGE_ORG_MEMBERS",
          "VIEW_ORGANIZATION",
          "VIEW_ORG_REWARD_REPORTS",
        ],
        email: "ada@example.test",
        email_verified: true,
        id: 14,
        joined_at: "2026-06-09T10:00:00",
        kyc_verified: true,
        name: "Ada Admin",
        roles: ["ADMIN"],
      },
    ],
    offset: 8,
    operator_permissions: {
      can_assign_roles: false,
      can_invite_members: true,
      can_manage_members: true,
      can_manage_settings: true,
      can_view_members: true,
    },
    organization: {
      id: 7,
      name: "Ferris Academy",
    },
    permission: "VIEW_ORGANIZATION",
    role: "ADMIN",
    search: "ada",
    total: 1,
  };
}

function organizationTeacherApplicationsFixture() {
  return {
    applications: [
      {
        applicant: {
          email: "ada.applicant@example.test",
          id: 44,
          name: "Ada Applicant",
        },
        audit: {
          event_count: 1,
          latest_event_at: "2026-06-09T10:30:00Z",
          latest_event_type: "organization_nominated",
          latest_reason: "Sponsored by Ferris Academy",
        },
        created_at: "2026-06-09T10:30:00Z",
        decided_at: null,
        decision_reason: null,
        experience_summary: "Ada teaches Rust ownership and project reviews.",
        id: 91,
        portfolio_links: ["https://example.test/portfolio"],
        requested_course: null,
        requested_for_this_organization: true,
        requested_organization: {
          id: 7,
          name: "Ferris Academy",
        },
        requested_scope: "organization",
        reviewer: null,
        sponsored_by_this_organization: true,
        status: "submitted",
        updated_at: "2026-06-09T10:30:00Z",
      },
    ],
    limit: 6,
    offset: 6,
    operator_permissions: {
      can_nominate_teachers: true,
      can_view_applications: true,
    },
    organization: {
      id: 7,
      name: "Ferris Academy",
    },
    search: "ada",
    status: "submitted",
    summary: {
      approved: 1,
      needs_changes: 0,
      rejected: 0,
      submitted: 1,
      total: 2,
    },
    total: 1,
  };
}

function courseCatalogItemFixture() {
  return {
    access: {
      can_request_join: true,
      can_view_content: true,
      can_view_course: true,
      can_view_rewards: false,
    },
    content: {
      chapter_count: 1,
      content_count: 1,
      content_types: ["video"],
      has_content: true,
    },
    description: null,
    enrollment: {
      can_request_join: true,
      reason: "Enrollment can be requested.",
      request_id: null,
      roles: [],
      state: "available",
    },
    id: 7,
    lifecycle_status: "published",
    organizations: [{ id: 1, name: "Rust Org" }],
    rewards: {
      active_policy_count: 1,
      available: true,
      event_types: ["course_completion"],
      payment_strategies: ["treasury_transfer"],
      token_amounts: ["25"],
    },
    teachers: [{ id: 2, name: "Teacher One" }],
    title: "Rust Ownership",
    topics: [],
  };
}

function teacherApplicationSnapshotFixture() {
  return {
    application: {
      applicant_user_id: 11,
      created_at: "2026-01-01T10:00:00Z",
      decided_at: null,
      decision_reason: null,
      experience_summary: "I teach Rust fundamentals and review project work.",
      id: 44,
      idempotency_key: "teacher-application-retry-key",
      organization_sponsor_id: null,
      portfolio_links: ["https://example.test/portfolio"],
      requested_course_id: null,
      requested_organization_id: null,
      requested_scope: "platform",
      reviewer_id: null,
      status: "submitted",
      updated_at: "2026-01-01T10:00:00Z",
    },
    audit_events: [
      {
        actor_user_id: 11,
        application_id: 44,
        created_at: "2026-01-01T10:00:00Z",
        event_type: "submitted",
        from_status: null,
        id: 101,
        reason: null,
        to_status: "submitted",
      },
    ],
  };
}

function teacherCoursesFixture() {
  return {
    courses: [
      {
        content: {
          chapter_count: 2,
          content_count: 4,
          content_types: ["article", "video"],
          has_content: true,
        },
        id: 9,
        lifecycle_status: "published",
        organizations: [{ id: 7, name: "Ferris Academy" }],
        permissions: {
          can_approve_reward_candidates: true,
          can_manage_content: true,
          can_manage_enrollments: true,
          can_manage_reward_rules: true,
          can_manage_settings: true,
          can_view_reward_candidates: true,
        },
        reward_queue: {
          failed_count: 0,
          pending_teacher_count: 1,
          teacher_approved_count: 3,
        },
        rewards: {
          active_policy_count: 1,
          available: true,
          event_types: ["course_completion"],
          payment_strategies: ["treasury_transfer"],
          token_amounts: ["25"],
        },
        roster: {
          enrolled_student_count: 12,
          pending_join_request_count: 2,
          waitlisted_join_request_count: 1,
        },
        title: "Rust Ownership Lab",
      },
    ],
    lifecycle_status: "published",
    limit: 5,
    offset: 0,
    search: "Rust",
    total: 1,
  };
}

function teacherCourseWorkspaceFixture() {
  return {
    chapters: [
      {
        contents: [
          {
            content_type: "article",
            data_present: true,
            display_state: "ready",
            id: 31,
            order: 1,
            processing_error: null,
            processing_status: null,
            publication_status: "inherits_course_published",
          },
          {
            content_type: "video",
            data_present: true,
            display_state: "failed_processing",
            id: 32,
            order: 2,
            processing_error: "Transcode failed",
            processing_status: "failed",
            publication_status: "inherits_course_published",
          },
        ],
        id: 14,
        order: 1,
        title: "Ownership basics",
      },
    ],
    course: teacherCoursesFixture().courses[0],
    publication: {
      content_publication_status_supported: false,
      course_lifecycle_status: "published",
    },
    teacher_roles: ["TEACHER"],
  };
}

function teacherCourseEnrollmentFixture() {
  return {
    course: teacherCoursesFixture().courses[0],
    join_requests: {
      limit: 10,
      offset: 0,
      requests: [
        {
          can_decide: true,
          created_at: "2026-01-02T10:00:00Z",
          decided_at: null,
          decision_reason: null,
          id: 55,
          requester: {
            email: "ada@example.test",
            email_verified: true,
            id: 88,
            kyc_verified: true,
            name: "Ada Learner",
          },
          reviewer: null,
          status: "pending",
          updated_at: "2026-01-02T10:00:00Z",
        },
        {
          can_decide: true,
          created_at: "2026-01-03T10:00:00Z",
          decided_at: null,
          decision_reason: "Capacity review",
          id: 56,
          requester: {
            email: "grace@example.test",
            email_verified: true,
            id: 89,
            kyc_verified: false,
            name: "Grace Learner",
          },
          reviewer: null,
          status: "waitlisted",
          updated_at: "2026-01-03T10:00:00Z",
        },
      ],
      status: "open",
      total: 2,
    },
    progress_supported: false,
    reward_eligibility_supported: false,
    roster: {
      learners: [
        {
          access_state: "enrolled",
          can_remove: true,
          latest_join_request_status: "approved",
          progress_supported: false,
          reward_eligibility_supported: false,
          roles: ["STUDENT"],
          user: {
            email: "linus@example.test",
            email_verified: true,
            id: 77,
            kyc_verified: true,
            name: "Linus Learner",
          },
        },
      ],
      total: 1,
    },
    teacher_roles: ["TEACHER"],
  };
}

function teacherCourseStudentsFixture() {
  return {
    course: teacherCoursesFixture().courses[0],
    progress_supported: false,
    reward_evidence_supported: true,
    students: [
      {
        access_state: "enrolled",
        latest_join_request_status: "approved",
        progress: {
          completed_content_count: null,
          completion_percentage: null,
          last_activity_at: null,
          note: "Persisted lesson progress is not tracked yet.",
          supported: false,
          total_content_count: 4,
        },
        rewards: {
          completed_count: 0,
          failed_count: 0,
          latest_candidate: {
            created_at: "2026-01-04T10:00:00Z",
            event_type: "course_completion",
            evidence: { completion_percentage: 100 },
            id: 71,
            status: "pending_teacher_approval",
            teacher_decision_reason: null,
            updated_at: "2026-01-04T10:00:00Z",
          },
          pending_teacher_count: 1,
          reward_candidate_count: 1,
          teacher_approved_count: 0,
          teacher_rejected_count: 0,
        },
        roles: ["STUDENT"],
        user: {
          email: "linus@example.test",
          email_verified: true,
          id: 77,
          kyc_verified: true,
          name: "Linus Learner",
        },
      },
    ],
    teacher_roles: ["TEACHER"],
    total: 1,
  };
}

function teacherRewardCandidatesFixture() {
  return [
    {
      amount_decided_at: null,
      amount_decision_reason: null,
      amount_reviewer_user_id: null,
      approved_amount: null,
      course_id: 9,
      created_at: "2026-01-04T10:00:00Z",
      event_type: "course_completion",
      evidence: { completion_percentage: 100, lesson_id: 14 },
      id: 71,
      idempotency_key: "course-9-student-77-completion",
      source_organization_id: null,
      source_scope: "course",
      status: "pending_teacher_approval",
      student_user_id: 77,
      submitter_user_id: 42,
      teacher_approver_user_id: null,
      teacher_decided_at: null,
      teacher_decision_reason: null,
      updated_at: "2026-01-04T10:00:00Z",
    },
    {
      amount_decided_at: null,
      amount_decision_reason: null,
      amount_reviewer_user_id: null,
      approved_amount: null,
      course_id: 9,
      created_at: "2026-01-05T10:00:00Z",
      event_type: "assessment_completion",
      evidence: { score_percentage: 94 },
      id: 72,
      idempotency_key: "course-9-student-90-assessment",
      source_organization_id: 7,
      source_scope: "organization",
      status: "pending_teacher_approval",
      student_user_id: 90,
      submitter_user_id: 42,
      teacher_approver_user_id: null,
      teacher_decided_at: null,
      teacher_decision_reason: null,
      updated_at: "2026-01-05T10:00:00Z",
    },
  ];
}
