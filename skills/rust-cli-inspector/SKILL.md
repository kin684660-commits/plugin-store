---
name: rust-cli-inspector
description: "Rust CLI Inspector - querying ETH price via OnchainOS"
version: "1.0.0"
author: "E2E Test"
tags: [test, rust, onchainos]
---


## Pre-flight Dependencies (auto-injected by Plugin Store CI)

> Run once per session before first use. These checks ensure required tools are installed.

### Install onchainos CLI + Skills (auto-injected)

```bash
# 1. Install onchainos CLI
onchainos --version 2>/dev/null || curl -fsSL https://raw.githubusercontent.com/okx/onchainos-skills/main/install.sh | sh

# 2. Install onchainos skills (enables AI agent to use onchainos commands)
npx skills add okx/onchainos-skills --yes --global

# 3. Install plugin-store skills (enables plugin discovery and management)
npx skills add okx/plugin-store --skill plugin-store --yes --global
```

### Install rust-cli-inspector binary (auto-injected)

```bash
if ! command -v rust-cli-inspector >/dev/null 2>&1; then
  OS=$(uname -s | tr A-Z a-z)
  ARCH=$(uname -m)
  case "${OS}_${ARCH}" in
    darwin_arm64)  TARGET="aarch64-apple-darwin" ;;
    darwin_x86_64) TARGET="x86_64-apple-darwin" ;;
    linux_x86_64)  TARGET="x86_64-unknown-linux-gnu" ;;
    linux_aarch64) TARGET="aarch64-unknown-linux-gnu" ;;
  esac
  curl -fsSL "https://github.com/okx/plugin-store/releases/download/plugins/rust-cli-inspector@1.0.0/rust-cli-inspector-${TARGET}" -o ~/.local/bin/rust-cli-inspector
  chmod +x ~/.local/bin/rust-cli-inspector
fi
```

### Report install (auto-injected, runs once)

```bash
REPORT_FLAG="$HOME/.plugin-store/reported/rust-cli-inspector"
if [ ! -f "$REPORT_FLAG" ]; then
  mkdir -p "$HOME/.plugin-store/reported"
  # Generate device ID
  DEV_RAW="$(hostname):$(uname -s):$(uname -m):$HOME"
  DEV_ID=$(echo -n "$DEV_RAW" | shasum -a 256 | head -c 32)
  # Report to Vercel stats
  curl -s -X POST "https://plugin-store-dun.vercel.app/install" \
    -H "Content-Type: application/json" \
    -d '{"name":"rust-cli-inspector","version":"1.0.0"}' >/dev/null 2>&1 || true
  # Report to OKX API
  curl -s -X POST "https://www.okx.com/priapi/v1/wallet/plugins/download/report" \
    -H "Content-Type: application/json" \
    -d '{"pluginName":"rust-cli-inspector","divId":"'"$DEV_ID"'"}' >/dev/null 2>&1 || true
  touch "$REPORT_FLAG"
fi
```

---


# Test Rust CLI

## Overview
E2E test plugin with Rust CLI binary. Queries ETH price via OnchainOS.

## Pre-flight Checks
1. Install onchainos CLI: `curl -sSL https://raw.githubusercontent.com/okx/onchainos-skills/main/install.sh | sh`
2. Ensure rust-cli-inspector binary is installed

## Commands

### Query ETH Price via CLI
```bash
rust-cli-inspector --query eth-price
```
**When to use:** When user asks about ETH price.
**Output:** Calls `onchainos token price-info --address 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2 --chain ethereum` and formats the result.

### Direct OnchainOS Query
```bash
onchainos token price-info --address 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2 --chain ethereum
```

## Error Handling
| Error | Cause | Resolution |
|-------|-------|------------|
| Binary not found | CLI not installed | Install via plugin-store |
| Command not found | onchainos not installed | Run pre-flight install |
