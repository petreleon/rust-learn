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
const teacher = await importTranspiled("src/lib/teacher.ts");

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

function textResponse(body, { status = 200 } = {}) {
  return new Response(body, {
    headers: { "content-type": "text/plain" },
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
