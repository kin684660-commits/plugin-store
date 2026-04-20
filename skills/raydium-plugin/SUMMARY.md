**Overview**

Swap tokens on Raydium — Solana's largest AMM — with live quotes, multi-mint price checks, pool browsing, and a preview-before-execute flow using your onchainos wallet.

**Prerequisites**
- onchainos agentic wallet connected
- At least 0.01 SOL in your wallet for gas plus the swap amount

**How it Works**
1. **Check wallet readiness**: Verify your wallet is connected and has enough SOL for gas. `raydium-plugin quickstart`
2. **Get a live quote**: See the expected output before committing — no gas. `raydium-plugin get-swap-quote --input-mint So11111111111111111111111111111111111111112 --output-mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v --amount <amount>`
3. **Check token prices**: Look up current prices for one or more token mints. `raydium-plugin get-token-price --mints So11111111111111111111111111111111111111112`
4. **Browse pools**: Find pools sorted by liquidity, volume, or APR. `raydium-plugin get-pool-list --sort-field liquidity --sort-type desc --page-size 5`
5. **Preview a swap**: See the full transaction details before signing — no gas, no transaction. `raydium-plugin swap --input-mint <mint> --output-mint <mint> --amount <amount> --slippage-bps 50`
6. **Execute the swap**: Broadcast the transaction after confirming the preview. `raydium-plugin swap --input-mint <mint> --output-mint <mint> --amount <amount> --slippage-bps 50 --confirm`
