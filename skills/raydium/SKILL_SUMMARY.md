
# raydium -- Skill Summary

## Overview
The Raydium plugin enables AI agents to interact with the Raydium automated market maker (AMM) on Solana mainnet. It provides comprehensive functionality for token swapping, price discovery, and liquidity pool analysis through direct integration with Raydium's REST APIs and transaction infrastructure.

## Usage
Install the plugin via the auto-injected dependencies, then use commands like `raydium get-swap-quote` for price discovery or `raydium swap` for executing trades. Always run swaps with `--dry-run` first and confirm with the user before executing real transactions.

## Commands
| Command | Description |
|---------|-------------|
| `get-swap-quote` | Get expected output amount and price impact for a token swap |
| `get-price` | Calculate price ratio between two tokens |
| `get-token-price` | Fetch USD prices for one or more tokens |
| `get-pools` | Query pool information by mint addresses or pool IDs |
| `get-pool-list` | Browse paginated list of all Raydium pools |
| `swap` | Execute token swap on Raydium (requires user confirmation) |

## Triggers
Activate this skill when users want to swap tokens on Raydium, check token prices, query pool information, or get swap quotes on Solana. Trigger phrases include "swap on raydium", "raydium price", "raydium pool", and "get swap quote raydium".
