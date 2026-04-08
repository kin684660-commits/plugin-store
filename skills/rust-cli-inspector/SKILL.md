---
name: rust-cli-inspector
description: "Rust CLI querying ETH price via Onchain OS"
version: "1.1.0"
author: "yz06276"
tags: [rust, onchainos]
---

# Rust CLI Inspector

## Overview
Queries ETH price via Onchain OS token price-info.

## Pre-flight Checks
1. Ensure rust-cli-inspector binary is installed
2. Ensure onchainos CLI is available

## Commands

### Query ETH Price (default)
`rust-cli-inspector`

**When to use:** When user asks about ETH price.

### Query ETH Price (explicit)
`rust-cli-inspector --query eth-price`

### Help
`rust-cli-inspector --help`

## Error Handling
| Error | Cause | Resolution |
|-------|-------|------------|
| Binary not found | CLI not installed | Run pre-flight |
| onchainos not found | Onchain OS not installed | Install onchainos |
