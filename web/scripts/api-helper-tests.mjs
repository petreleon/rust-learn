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
