
# velodrome-v2 -- Skill Summary

## Overview
This skill provides comprehensive access to Velodrome V2's classic AMM functionality on Optimism, enabling token swaps, liquidity management, and rewards claiming. It supports both volatile (constant-product) and stable (low-slippage) pools, with automatic pool selection for optimal routing. All write operations use a secure two-step confirmation process through the onchainos wallet system.

## Usage
Ensure onchainos CLI is installed and your wallet is configured. Run commands first without `--confirm` to preview transactions, then add `--confirm` to broadcast. The `velodrome-v2` binary must be available in your PATH.

## Commands
- `quote` - Get swap quotes without executing transactions
- `swap` - Execute token swaps via Router (requires --confirm)
- `pools` - Query pool information and reserves
- `positions` - View LP token balances and positions
- `add-liquidity` - Add liquidity to pools (requires --confirm)
- `remove-liquidity` - Remove liquidity from pools (requires --confirm)
- `claim-rewards` - Claim VELO gauge emissions (requires --confirm)

## Triggers
Activate this skill when users want to swap tokens, manage liquidity positions, or claim rewards on Velodrome V2's classic AMM pools on Optimism. Use for DeFi operations involving WETH, USDC, VELO, and other major Optimism tokens.
