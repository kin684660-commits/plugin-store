**Overview**

Add concentrated liquidity to Meteora DLMM pools on Solana — earning fees only on actively-traded price bins — with support for SOL-only, token-only, or two-sided deposits.

**Prerequisites**
- onchainos agentic wallet connected with a Solana wallet (chain 501)
- SOL balance for transaction fees (minimum ~0.01 SOL)
- For two-sided liquidity: both tokens of the pair in your wallet

**How it Works**
1. **Find high-volume pools**: Browse pools sorted by volume, TVL, or APR. `meteora-plugin get-pools --search-term SOL-USDC --sort-key volume --order-by desc`
2. **Get pool details**: See active bin price, fee tier, TVL, and bin step for a specific pool. `meteora-plugin get-pool-detail --address <pool>`
3. **Check existing positions**: List open positions with bin ranges and accrued fees. `meteora-plugin get-user-positions`
4. **Get a swap quote**: Check expected output before committing — no gas. `meteora-plugin get-swap-quote --pool <address> --input-mint <token> --amount <n>`
5. **Swap**: Execute the swap against the DLMM pool. `meteora-plugin swap --pool <address> --input-mint <token> --amount <n> --confirm`
6. **Add liquidity**: Deposit into price bins to earn fees — omit one side for single-token deposit. `meteora-plugin add-liquidity --pool <address> --amount-x <SOL> --amount-y <USDC> --confirm`
7. **Remove liquidity**: Withdraw your position and collect accrued fees. `meteora-plugin remove-liquidity --pool <address> --position <address> --confirm`
