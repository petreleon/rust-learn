import { spawn } from "node:child_process";
import readline from "node:readline";
import fs from "node:fs";
import path from "node:path";

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function runMcpClient() {
  console.log("Starting browsermcp server...");
  const child = spawn("npx", ["@browsermcp/mcp@latest"], {
    stdio: ["pipe", "pipe", "inherit"],
    shell: true
  });

  const rl = readline.createInterface({
    input: child.stdout,
    output: child.stdin,
    terminal: false
  });

  let messageId = 1;
  const pendingRequests = new Map();

  function sendRequest(method, params = {}) {
    const id = messageId++;
    const request = {
      jsonrpc: "2.0",
      id,
      method,
      params
    };
    return new Promise((resolve, reject) => {
      pendingRequests.set(id, { resolve, reject });
      child.stdin.write(JSON.stringify(request) + "\n");
    });
  }

  rl.on("line", (line) => {
    try {
      const response = JSON.parse(line);
      if (response.id && pendingRequests.has(response.id)) {
        const { resolve } = pendingRequests.get(response.id);
        pendingRequests.delete(response.id);
        resolve(response.result || response.error);
      }
    } catch (err) {
      console.error("Failed to parse server line:", line, err);
    }
  });

  // 1. Initialize
  const initResult = await sendRequest("initialize", {
    protocolVersion: "2024-11-05",
    capabilities: {},
    clientInfo: { name: "custom-mcp-client", version: "1.0.0" }
  });
  console.log("Initialized:", JSON.stringify(initResult));

  // Send initialized notification
  child.stdin.write(JSON.stringify({
    jsonrpc: "2.0",
    method: "notifications/initialized"
  }) + "\n");

  console.log("Waiting 3 seconds for Chrome extension to connect...");
  await sleep(3000);

  // 2. Navigate
  console.log("Navigating to http://localhost:3000...");
  const navResult = await sendRequest("tools/call", {
    name: "browser_navigate",
    arguments: { url: "http://localhost:3000/learn" }
  });
  console.log("Navigation Result:", JSON.stringify(navResult));

  await sleep(2000);

  // 3. Take Screenshot
  console.log("Taking screenshot...");
  const screenshotResult = await sendRequest("tools/call", {
    name: "browser_screenshot",
    arguments: {}
  });

  if (screenshotResult && screenshotResult.content && screenshotResult.content[0]) {
    const content = screenshotResult.content[0];
    if (content.type === "image" || (content.text && content.text.startsWith("data:image"))) {
      // Find base64 data
      let base64Data = "";
      if (content.type === "image") {
        base64Data = content.data;
      } else {
        base64Data = content.text.split(",")[1];
      }
      const buffer = Buffer.from(base64Data, "base64");
      const outPath = path.join(process.cwd(), "browsermcp_screenshot.png");
      fs.writeFileSync(outPath, buffer);
      console.log(`Screenshot saved to ${outPath}`);
    } else {
      console.log("Unexpected screenshot content format:", JSON.stringify(screenshotResult));
    }
  } else {
    console.log("Screenshot call failed or returned empty:", JSON.stringify(screenshotResult));
  }

  // 4. Close connection cleanly
  console.log("Closing child process...");
  child.kill();
  process.exit(0);
}

runMcpClient().catch((err) => {
  console.error("Client error:", err);
  process.exit(1);
});
