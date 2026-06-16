#!/usr/bin/env node

import { readdirSync, readFileSync, statSync } from "node:fs";
import path from "node:path";

const webRoot = path.resolve(import.meta.dirname, "..");
const srcRoot = path.join(webRoot, "src");
const strict = process.argv.includes("--strict");
const maxLines = Number.parseInt(process.env.WEB_ARCH_MAX_LINES || "180", 10);
const maxLineLength = Number.parseInt(process.env.WEB_ARCH_MAX_LINE_LENGTH || "220", 10);
const sourceExtensions = new Set([".css", ".ts", ".tsx"]);

const findings = {
  apiImports: [],
  denseLines: [],
  longFiles: [],
  viewSideEffects: [],
};

function walk(dir) {
  return readdirSync(dir).flatMap((entry) => {
    const filePath = path.join(dir, entry);
    const stats = statSync(filePath);
    if (stats.isDirectory()) {
      return walk(filePath);
    }
    return sourceExtensions.has(path.extname(filePath)) ? [filePath] : [];
  });
}

function rel(filePath) {
  return path.relative(webRoot, filePath).split(path.sep).join("/");
}

function sourceLineCount(lines) {
  return lines.at(-1) === "" ? lines.length - 1 : lines.length;
}

function isApiModule(filePath) {
  return /\/(?:api|shared\/api)(?:\/|$)/.test(filePath);
}

function isViewModule(filePath) {
  return /\/views?\//.test(filePath) || /View\.tsx$/.test(filePath);
}

function scanFile(filePath) {
  const relativePath = rel(filePath);
  const content = readFileSync(filePath, "utf8");
  const lines = content.split(/\r?\n/);
  const lineCount = sourceLineCount(lines);

  if (lineCount > maxLines) {
    findings.longFiles.push(`${relativePath}:${lineCount}`);
  }

  lines.forEach((line, index) => {
    if (line.length > maxLineLength) {
      findings.denseLines.push(`${relativePath}:${index + 1}:${line.length}`);
    }
  });

  if (isApiModule(relativePath) && importsUiSurface(content)) {
    findings.apiImports.push(relativePath);
  }

  if (isViewModule(relativePath) && hasViewSideEffect(content)) {
    findings.viewSideEffects.push(relativePath);
  }
}

function importsUiSurface(content) {
  return [
    /from\s+["']react["']/,
    /from\s+["']next\//,
    /from\s+["'].*\.module\.css["']/,
    /from\s+["']@\/components/,
    /from\s+["']@\/features\/.*\/(?:route|views?|components)\//,
  ].some((pattern) => pattern.test(content));
}

function hasViewSideEffect(content) {
  return [
    /\bfetch\s*\(/,
    /\breadStoredSessionToken\b/,
    /\blocalStorage\b/,
    /\bsessionStorage\b/,
    /\bnormalizeRouteError\b/,
  ].some((pattern) => pattern.test(content));
}

function printGroup(title, items) {
  console.log(`${title}: ${items.length}`);
  items.slice(0, 40).forEach((item) => console.log(`  ${item}`));
  if (items.length > 40) {
    console.log(`  ... ${items.length - 40} more`);
  }
}

walk(srcRoot).forEach(scanFile);

console.log("Web architecture scan");
console.log(`Root: ${path.relative(process.cwd(), webRoot) || "."}`);
console.log(`Line limit: ${maxLines}`);
console.log(`Dense-line limit: ${maxLineLength}`);
printGroup("Long files", findings.longFiles);
printGroup("Dense lines", findings.denseLines);
printGroup("API modules importing UI/React surfaces", findings.apiImports);
printGroup("View modules with API/storage/error side effects", findings.viewSideEffects);

const findingCount = Object.values(findings).reduce((total, items) => total + items.length, 0);
if (strict && findingCount > 0) {
  console.error(`Architecture scan failed with ${findingCount} finding(s).`);
  process.exit(1);
}
