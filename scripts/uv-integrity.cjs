#!/usr/bin/env node
// CI-only integrity checks. These do not execute uv or require Python.
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const targets = new Set([
  "x86_64-unknown-linux-gnu", "x86_64-unknown-linux-musl",
  "aarch64-unknown-linux-gnu", "aarch64-unknown-linux-musl",
  "aarch64-apple-darwin", "x86_64-pc-windows-msvc",
]);

function loadPins(filename, version) {
  const pins = JSON.parse(fs.readFileSync(filename, "utf8"));
  if (pins.schemaVersion !== 1 || pins.version !== version ||
      !/^\d+\.\d+\.\d+$/.test(version) ||
      pins.release !== `https://github.com/astral-sh/uv/releases/tag/${version}` ||
      !pins.targets || Object.keys(pins.targets).length !== targets.size) {
    throw new Error("The uv version or checksum inventory is not reviewed.");
  }
  for (const target of targets) {
    const pin = pins.targets[target];
    if (!pin || !/^[a-f0-9]{64}$/.test(pin.archiveSha256) ||
        !/^[a-f0-9]{64}$/.test(pin.binarySha256)) {
      throw new Error("A required uv checksum is missing or malformed.");
    }
  }
  return pins;
}

function resolveTarget(platform, architecture, musl) {
  const arch = { x64: "x86_64", arm64: "aarch64" }[architecture];
  const suffix = platform === "linux" ? `unknown-linux-${musl ? "musl" : "gnu"}` :
    { darwin: "apple-darwin", win32: "pc-windows-msvc" }[platform];
  const target = `${arch}-${suffix}`;
  if (!targets.has(target)) throw new Error("This uv host has no reviewed checksum.");
  return target;
}

function currentTarget() {
  let musl = false;
  if (process.platform === "linux") {
    // Match the pinned setup action's ldd-based libc selection.
    const result = spawnSync("ldd", ["--version"], {
      encoding: "utf8", timeout: 5000, maxBuffer: 65536,
    });
    if (result.error) throw new Error("Cannot establish the Linux libc for uv.");
    musl = (result.stdout + result.stderr).includes("musl");
  }
  return resolveTarget(process.platform, process.arch, musl);
}

function verifyBinary(filename, expected) {
  const info = fs.statSync(filename);
  if (!info.isFile() || info.size > 128 * 1024 * 1024) {
    throw new Error("The uv executable is not a bounded regular file.");
  }
  const actual = crypto.createHash("sha256").update(fs.readFileSync(filename)).digest("hex");
  if (actual !== expected) throw new Error("The installed uv executable checksum does not match.");
}

function main() {
  const args = process.argv.slice(2);
  const pins = loadPins(path.join(__dirname, "uv-checksums.json"), process.env.CI_UV_VERSION);
  const target = currentTarget();
  const pin = pins.targets[target];
  if (args.length === 1 && args[0] === "select") {
    const output = process.env.GITHUB_OUTPUT;
    const temporary = process.env.RUNNER_TEMP;
    if (!output || !temporary || /[\r\n]/.test(output + temporary)) {
      throw new Error("Run uv selection with runner output and temporary paths.");
    }
    // A fresh owned tool cache prevents the action from executing an old cache
    // entry before its download checksum check. Other actions keep their cache.
    const cache = fs.mkdtempSync(path.join(temporary, "scm-sdk-uv-"));
    fs.appendFileSync(output, `checksum=${pin.archiveSha256}\ntool-cache=${cache}\n`);
  } else if (args.length === 2 && args[0] === "verify") {
    verifyBinary(args[1], pin.binarySha256);
    console.log(`Verified uv ${pins.version} executable for ${target}.`);
  } else {
    throw new Error("Use uv-integrity.cjs select or verify EXECUTABLE.");
  }
}

module.exports = { loadPins, resolveTarget, verifyBinary };
if (require.main === module) {
  try { main(); } catch (error) {
    console.error(error instanceof Error ? error.message : "uv integrity check failed.");
    process.exitCode = 1;
  }
}
