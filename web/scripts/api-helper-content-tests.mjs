import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import test from "node:test";
import { pathToFileURL } from "node:url";
import ts from "typescript";

const repoRoot = path.resolve(import.meta.dirname, "..");
const compiledDir = mkdtempSync(path.join(tmpdir(), "rustlearn-api-helper-content-tests-"));
const transpiledCache = new Map();
const teacher = await importTranspiled("src/lib/teacher.ts");

test("fetchTeacherContentProcessingHistory sends content-scoped GET", async () => {
  const calls = mockFetch((url, init) => {
    assert.equal(url, "/api/courses/5/chapters/3/contents/10/processing-history");
    assert.equal(init.method, "GET");
    assert.equal(init.headers.Authorization, "Bearer teacher-token");
    return jsonResponse({
      content_id: 10,
      jobs: [{
        attempts: 2,
        created_at: "2026-06-16T08:00:00Z",
        id: 44,
        last_error: "transcode timed out",
        status: "failed",
        updated_at: "2026-06-16T08:01:00Z",
      }],
      object_key: "courses/5/chapters/3/lecture.mp4",
    });
  });
  const result = await teacher.fetchTeacherContentProcessingHistory({
    chapterId: 3,
    contentId: 10,
    courseId: 5,
    token: "teacher-token",
  });
  assert.equal(calls.length, 1);
  assert.equal(result.object_key, "courses/5/chapters/3/lecture.mp4");
  assert.equal(result.jobs[0].last_error, "transcode timed out");
});

async function importTranspiled(relativePath) {
  return import(pathToFileURL(transpileSourceFile(path.join(repoRoot, relativePath))));
}

function transpileSourceFile(sourcePath) {
  const normalizedSourcePath = path.resolve(sourcePath);
  const cached = transpiledCache.get(normalizedSourcePath);
  if (cached) return cached;

  const outputPath = compiledPathFor(normalizedSourcePath);
  transpiledCache.set(normalizedSourcePath, outputPath);
  const source = rewriteLocalImports(readFileSync(normalizedSourcePath, "utf8"), normalizedSourcePath, outputPath);
  const output = ts.transpileModule(source, {
    compilerOptions: { module: ts.ModuleKind.ES2022, target: ts.ScriptTarget.ES2022 },
    fileName: normalizedSourcePath,
  });
  writeFileSync(outputPath, output.outputText);
  return outputPath;
}

function compiledPathFor(sourcePath) {
  const relative = path.relative(repoRoot, sourcePath).replaceAll(path.sep, "__").replace(/\.(tsx?|jsx?)$/, ".mjs");
  return path.join(compiledDir, relative);
}

function rewriteLocalImports(source, sourcePath, outputPath) {
  return source.replace(/(from\s+["'])(\.{1,2}\/[^"']+)(["'])/g, (_match, prefix, specifier, suffix) => {
    const dependencySource = resolveLocalSource(sourcePath, specifier);
    const dependencyOutput = transpileSourceFile(dependencySource);
    let rewritten = path.relative(path.dirname(outputPath), dependencyOutput).replaceAll(path.sep, "/");
    if (!rewritten.startsWith(".")) rewritten = `./${rewritten}`;
    return `${prefix}${rewritten}${suffix}`;
  });
}

function resolveLocalSource(sourcePath, specifier) {
  const base = path.resolve(path.dirname(sourcePath), specifier);
  const candidates = [base, `${base}.ts`, `${base}.tsx`, `${base}.js`, `${base}.jsx`, path.join(base, "index.ts")];
  for (const candidate of candidates) {
    try {
      readFileSync(candidate);
      return candidate;
    } catch {}
  }
  throw new Error(`Unable to resolve local import ${specifier} from ${sourcePath}`);
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
