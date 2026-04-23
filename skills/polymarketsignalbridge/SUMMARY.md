# Polymarket Signal Bridge

## Overview
Cross-analyze Polymarket prediction odds with Hyperliquid funding rates and OKX smart-money signals to surface convergence and divergence opportunities.

## Quick Start
1. Install the OKX OnchainOS CLI:
npx skills add okx/onchainos-skills
2. Install this skill:
npx skills add okx/plugin-store --skill polymarketsignalbridge
3. Ask your agent: "Scan for Polymarket and Hyperliquid convergence signals today"

## Prerequisites
- OKX OnchainOS CLI installed
- Internet access to Polymarket and Hyperliquid public APIs
- No API keys or private keys required

## What This Plugin Does
- Scans trending Polymarket prediction markets
- Fetches Hyperliquid funding rates and open interest
- Pulls OKX smart-money signals and leaderboard data
- Analyzes wallet positions on both Polymarket and Hyperliquid
- Generates convergence and divergence signal reports
