---
name: wallet-tracker-mcap
version: "1.0.0"
description: >
  Wallet Copy-Trade Bot v1.0 — monitors target Solana wallets for meme token
  trades and auto-mirrors buys/sells with safety gates. Two follow modes:
  MC_TARGET (wait for market cap proof) or INSTANT. Tiered take-profit,
  trailing stop, mirror sell, time stop, and 4-tier risk grading.
  onchainos CLI + Agentic Wallet TEE signing.
updated: 2026-04-14
triggers: >
  wallet tracker, copy trade, wallet copy, follow wallet, mirror trade,
  wallet monitor, 跟单, 钱包跟踪, 钱包监控, 抄单, 跟买跟卖,
  wallet sniper, smart money follow, whale tracker, mcap target
---

# 钱包跟单策略 v1.0 — Wallet Copy-Trade Bot

> 本策略为真实交易机器人。使用前确保已理解风险，建议先以 PAPER 模式测试。

---

## 文件结构

```
WalletTracker/
├── skill.md              ← 本文件（策略说明）
├── config.py             ← 所有可调参数（修改参数只改这里）
├── wallet_tracker.py     ← 策略主程序
├── risk_check.py         ← 共享风控模块
├── dashboard.html        ← Web Dashboard UI
└── state/                ← [自动生成] 运行时数据
    ├── positions.json
    ├── trades.json
    ├── tracked_tokens.json   ← 正在跟踪的代币列表
    └── wallet_snapshots.json ← 目标钱包持仓快照
```

---

## 策略逻辑

### 核心思路

```
     ┌──────────────────────────────┐
     │  Poll target wallet holdings │
     │  (every POLL_INTERVAL sec)   │
     └──────────────┬───────────────┘
                    │
                    ▼
     ┌──────────────────────────────┐
     │  Compare with last snapshot  │
     │  Detect: NEW buys / SELLs   │
     └──────┬───────────────┬───────┘
            │               │
       NEW TOKEN         TOKEN SOLD
       detected          by wallet
            │               │
            ▼               ▼
     ┌─────────────┐ ┌──────────────┐
     │ Add to       │ │ If we hold   │
     │ tracked list │ │ same token → │
     │              │ │ mirror sell  │
     └──────┬──────┘ └──────────────┘
            │
            ▼
     ┌──────────────────────────────┐
     │  Safety checks:              │
     │  - risk_check pre-trade      │
     │  - MC / Liquidity / Holders  │
     │  - Dev / Bundler / Honeypot  │
     └──────────────┬───────────────┘
                    │
              ┌─────┴─────┐
              │           │
         MODE: INSTANT  MODE: MC_TARGET
              │           │
              ▼           ▼
     ┌────────────┐ ┌────────────────┐
     │ Buy now    │ │ Add to watch.  │
     │ immediately│ │ Buy when MC    │
     │            │ │ hits target    │
     └────────────┘ └────────────────┘
                    │
              (price monitor loop
               checks MC every
               MONITOR_INTERVAL)
                    │
                    ▼
              MC hits target → BUY
```

### 两种跟单模式

| 模式 | 说明 | 适合场景 |
|------|------|---------|
| **INSTANT** | 目标钱包买入 → 安全检查通过 → 立即跟买 | 信任目标钱包的判断，想第一时间跟进 |
| **MC_TARGET** | 目标钱包买入 → 安全检查通过 → 加入观察列表 → MC 达到目标值时才买入 | 想等代币有一定热度/验证后再进场（九三的需求） |

### 卖出逻辑（三种触发）

| 触发 | 说明 |
|------|------|
| **MIRROR_SELL** | 目标钱包卖出该代币 → 我们也卖（可配置：跟卖 100% 或按比例） |
| **STOP_LOSS** | 持仓亏损超过 STOP_LOSS_PCT → 止损卖出 |
| **TAKE_PROFIT** | 持仓盈利超过 TP 阈值 → 止盈卖出（梯度止盈） |
| **TIME_STOP** | 持仓超过 MAX_HOLD_HOURS → 时间止损 |

---

## 前置要求

### 1. 安装 onchainos CLI (>= 2.1.0)

```bash
onchainos --version
```

### 2. 登录 Agentic Wallet (TEE 签名)

```bash
onchainos wallet login <your-email>
onchainos wallet status        # → loggedIn: true
onchainos wallet addresses --chain 501   # 确认 Solana 地址
```

### 3. 无需 pip install

本策略仅依赖 Python 标准库 + onchainos CLI。

---

## Claude 启动交互协议

> **当用户要求启动本策略时，Claude 必须按以下流程执行，不得跳过直接启动。**

### Step 1: 展示策略简介

```
👁️ 钱包跟单策略 v1.0 — Wallet Copy-Trade Bot

本策略监控指定钱包地址的 meme 代币持仓变化。
当目标钱包买入新代币时，经过安全检查后自动跟买。
当目标钱包卖出时，可同步卖出。

支持两种模式：
  即时跟买 (INSTANT)：钱包买了就跟
  市值触发 (MC_TARGET)：等代币 MC 达到目标值再买

⚠️ 风险提示：跟单策略依赖目标钱包的判断，
   目标钱包亏损你也会亏损。建议先用 Paper 模式观察。

默认参数：
  模式：      PAPER（模拟，不花钱）
  跟单模式：  MC_TARGET（市值触发）
  目标市值：  $500,000
  单笔买入：  0.03 SOL
  最大持仓：  5 个
  止损：      -20%
  止盈：      +15% / +30% / +50%（梯度）
  最大持仓时间：6 小时
```

### Step 2: 询问用户配置（4 个问题）

使用 AskUserQuestion 依次确认：

**Q1 — 目标钱包地址**
- 用户提供要跟踪的 Solana 钱包地址
- 支持多个地址（逗号分隔）

→ 映射：`TARGET_WALLETS = ["地址1", "地址2"]`

**Q2 — 运行模式**
- 🧪 模拟模式 (PAPER)：只看信号，不花钱（推荐新手）
- 💰 实盘模式 (LIVE)：真实 SOL 交易

→ 映射：`MODE = "paper"/"live"`

**Q3 — 跟单模式**
- ⏳ 市值触发 (MC_TARGET)：等 MC 达到目标再买（更安全）
- ⚡ 即时跟买 (INSTANT)：钱包买了就跟（更快但风险更高）

→ 映射：`FOLLOW_MODE = "mc_target"/"instant"`

如果选了 MC_TARGET，追问目标市值（默认 $500K）

**Q4 — 风险偏好**
- 🛡️ 保守：小仓位，紧止损
- ⚖️ 默认：平衡配置（推荐）
- 🔥 激进：大仓位，宽止损

→ 映射预设：

| 偏好 | BUY_AMOUNT | STOP_LOSS_PCT | TP_TIERS | MAX_HOLD_HOURS |
|------|-----------|---------------|----------|----------------|
| 保守 | 0.02 SOL  | -12%          | (10,0.30),(20,0.40),(30,1.00) | 4 |
| 默认 | 0.03 SOL  | -20%          | (15,0.30),(30,0.40),(50,1.00) | 6 |
| 激进 | 0.05 SOL  | -30%          | (20,0.25),(40,0.35),(80,1.00) | 10 |

### Step 3: 应用配置并启动

1. 根据用户回答修改 `config.py`
2. 检查前置条件：`onchainos --version`、`onchainos wallet status`
3. 验证目标钱包地址有效：`onchainos portfolio token-balances --address <addr> --chains solana`
4. 启动 bot：`python3 wallet_tracker.py`
5. 展示 Dashboard 链接

---

## config.py 参数

```python
# ── 运行模式 ────────────────────────────────────────────────────────────
MODE              = "paper"       # "paper" / "live"
PAUSED            = True          # True=暂停（不开新仓），False=正常交易

# ── 目标钱包 ────────────────────────────────────────────────────────────
TARGET_WALLETS    = []            # 要跟踪的 Solana 钱包地址列表

# ── 跟单模式 ────────────────────────────────────────────────────────────
FOLLOW_MODE       = "mc_target"   # "mc_target" / "instant"
MC_TARGET_USD     = 500_000       # MC_TARGET 模式下的目标市值 ($)
MC_MAX_USD        = 50_000_000    # 市值上限 — 超过此值不跟买 ($)

# ── 卖出跟踪 ────────────────────────────────────────────────────────────
MIRROR_SELL       = True          # 目标钱包卖出时是否同步卖出
MIRROR_SELL_PCT   = 1.00          # 跟卖比例 (1.00=全卖, 0.50=卖一半)

# ── 仓位 ────────────────────────────────────────────────────────────────
BUY_AMOUNT        = 0.03          # 单笔买入 (SOL)
MAX_POSITIONS     = 5             # 最多同时持仓数
TOTAL_BUDGET      = 0.50          # SOL 总预算
SLIPPAGE_BUY      = 5             # 买入滑点 (%)
SLIPPAGE_SELL     = 15            # 卖出滑点 (%)
GAS_RESERVE       = 0.01          # 保留 gas (SOL)
MIN_WALLET_BAL    = 0.05          # 最低钱包余额才开仓 (SOL)

# ── 安全过滤（跟单仍需安全检查，不能盲跟）──────────────────────────────
MIN_LIQUIDITY     = 10_000        # 最小流动性 ($)
MIN_HOLDERS       = 30            # 最少持有者
MAX_TOP10_HOLD    = 60            # Top10 持仓上限 (%)
MAX_DEV_HOLD      = 30            # Dev 持仓上限 (%)
MAX_BUNDLE_HOLD   = 20            # Bundler 持仓上限 (%)
MAX_DEV_RUG_COUNT = 3             # Dev rug 次数上限
BLOCK_HONEYPOT    = True          # 拦截蜜罐
RISK_CHECK_GATE   = 3             # risk_check severity >= 此值则拒绝 (G3/G4 block)

# ── 止盈（梯度）────────────────────────────────────────────────────────
TP_TIERS = [
    (15, 0.30),   # +15% 卖 30%
    (30, 0.40),   # +30% 卖 40%
    (50, 1.00),   # +50% 卖剩余全部
]

# ── 止损 ────────────────────────────────────────────────────────────────
STOP_LOSS_PCT     = -20           # 硬止损 (%)
TRAILING_ACTIVATE = 10            # 追踪止损: 盈利超过 N% 激活
TRAILING_DROP     = 15            # 追踪止损: 从峰值回撤 N% 触发
MAX_HOLD_HOURS    = 6             # 时间止损: 最大持仓小时数

# ── Session 风控 ────────────────────────────────────────────────────────
MAX_CONSEC_LOSS   = 3             # 连续亏损 N 次 → 暂停
PAUSE_CONSEC_SEC  = 600           # 暂停时长 (秒)
SESSION_STOP_SOL  = 0.10          # 累计亏损 → 停止交易

# ── 轮询 ────────────────────────────────────────────────────────────────
POLL_INTERVAL     = 30            # 钱包监控轮询周期 (秒) — 别太频繁, 避免限流
MONITOR_INTERVAL  = 15            # 持仓 + MC 检查周期 (秒)
HEALTH_CHECK_SEC  = 300           # 钱包审计周期 (秒)

# ── Dashboard ──────────────────────────────────────────────────────────
DASHBOARD_PORT    = 3248
```

---

## 策略架构

```
wallet_tracker.py（单文件 Bot）
├── onchainos CLI（数据 + 执行 + 安全 — 无 API Key）
│
├── wallet_poll_loop()        ← 后台线程, 每 POLL_INTERVAL 秒
│   ├── get_wallet_holdings()      获取目标钱包当前持仓
│   │   └── onchainos portfolio token-balances
│   ├── diff_snapshot()            对比上次快照, 检测变化
│   │   ├── NEW tokens → _on_wallet_buy()
│   │   └── REMOVED tokens → _on_wallet_sell()
│   │
│   ├── _on_wallet_buy(token)      目标钱包买入了新代币
│   │   ├── safety_check()         安全过滤 (MC/Liq/Holders/Dev/Bundler)
│   │   ├── risk_check.pre_trade_checks()   风控模块检查
│   │   ├── if INSTANT → _execute_buy()
│   │   └── if MC_TARGET → add to watch_list
│   │
│   └── _on_wallet_sell(token)     目标钱包卖出了代币
│       └── if MIRROR_SELL and we hold → _execute_sell()
│
├── monitor_loop()            ← 后台线程, 每 MONITOR_INTERVAL 秒
│   ├── check_mc_targets()         检查观察列表中代币的 MC
│   │   └── onchainos token price-info
│   │   └── MC >= MC_TARGET_USD → _execute_buy()
│   │
│   ├── check_positions()          持仓退出决策
│   │   ├── STOP_LOSS: PnL <= STOP_LOSS_PCT
│   │   ├── TRAILING: peak PnL >= TRAILING_ACTIVATE, drop >= TRAILING_DROP
│   │   ├── TIME_STOP: held >= MAX_HOLD_HOURS
│   │   └── TAKE_PROFIT: 梯度止盈
│   │
│   └── risk_check.post_trade_flags()  后台风控监控
│       └── EXIT_NOW → 立即卖出
│
├── _execute_buy(token)       买入执行
│   ├── onchainos swap quote        报价 + 蜜罐检测
│   ├── onchainos swap swap         构建未签名交易
│   ├── onchainos wallet contract-call  TEE 签名 + 广播
│   └── onchainos wallet history    确认交易状态
│
├── _execute_sell(token, pct) 卖出执行
│   ├── onchainos swap swap         构建卖出交易
│   ├── onchainos wallet contract-call  TEE 签名 + 广播
│   └── onchainos wallet history    确认交易状态
│
├── Dashboard (port 3248)     Web UI
│   ├── 目标钱包持仓一览
│   ├── 观察列表 (MC_TARGET 模式)
│   ├── 当前持仓 + PnL
│   └── 交易记录
│
└── 持久化文件（JSON, 原子写入）
    ├── positions.json
    ├── trades.json
    ├── tracked_tokens.json
    └── wallet_snapshots.json
```

---

## onchainos CLI 命令清单

| # | 命令 | 用途 | 频率 |
|---|------|------|------|
| 1 | `onchainos portfolio token-balances --address <wallet> --chains solana` | 获取目标钱包全部代币持仓 | 每 POLL_INTERVAL |
| 2 | `onchainos token price-info --chain solana --address <token>` | 获取代币 MC / 价格 / 流动性 | 每 MONITOR_INTERVAL |
| 3 | `onchainos token advanced-info --chain solana --address <token>` | Dev/Bundler/蜜罐/安全数据 | 每个新代币 1 次 |
| 4 | `onchainos market prices --tokens 501:<addr1>,501:<addr2>,...` | 批量价格查询（持仓监控） | 每 MONITOR_INTERVAL |
| 5 | `onchainos swap quote --from 1111...1 --to <token> --amount <lamports> --chain solana` | 报价 + 蜜罐检测 | 每次买入前 |
| 6 | `onchainos swap swap --from 1111...1 --to <token> --amount <lamports> --chain solana --wallet <addr> --slippage <pct>` | 构建买入交易 | 每次买入 |
| 7 | `onchainos swap swap --from <token> --to 1111...1 --amount <amount> --chain solana --wallet <addr> --slippage <pct>` | 构建卖出交易 | 每次卖出 |
| 8 | `onchainos wallet contract-call --chain 501 --to <router> --unsigned-tx <callData>` | TEE 签名 + 广播 | 每次买卖 |
| 9 | `onchainos wallet history --tx-hash <hash> --chain-index 501` | 交易确认 | 买卖后轮询 |
| 10 | `onchainos wallet addresses --chain 501` | 获取自己的 Solana 地址 | 启动时 1 次 |
| 11 | `onchainos wallet balance --chain 501` | SOL 余额 | 每次买入前 |

---

## 钱包变化检测逻辑

```
每次 poll:
  current_holdings = get_wallet_holdings(target_wallet)
  prev_holdings    = load_snapshot()

  # 检测新买入
  for token in current_holdings:
      if token NOT in prev_holdings:
          → _on_wallet_buy(token)   # 新代币, 目标钱包刚买的

  # 检测卖出
  for token in prev_holdings:
      if token NOT in current_holdings:
          → _on_wallet_sell(token)  # 代币消失了, 目标钱包卖了
      elif current_holdings[token].amount < prev_holdings[token].amount:
          → _on_wallet_reduce(token)  # 部分卖出

  save_snapshot(current_holdings)
```

**重要:** `token-balances` 返回的是当前持仓，不是交易记录。所以我们通过快照对比来推断买卖行为。这意味着如果目标钱包在两次 poll 之间买入又卖出了同一个代币，我们会漏掉这笔交易。POLL_INTERVAL 不能太长。

---

## 安全检查（跟单不等于盲跟）

即使是跟踪信任的钱包，每个新代币仍然经过安全检查：

### 基础过滤

| 检查项 | 门槛 | 说明 |
|--------|------|------|
| 流动性 | >= $10,000 | 太低无法退出 |
| 持有者 | >= 30 | 太少可能是假币 |
| Top10 集中度 | <= 60% | 持仓太集中容易被砸 |
| Dev 持仓 | <= 30% | Dev 持仓太多可能 rug |
| Bundler 持仓 | <= 20% | Bundler 多说明不健康 |
| Dev Rug 次数 | <= 3 | Dev 有 rug 历史 |
| 蜜罐 | 必须非蜜罐 | 买了卖不出去 |

### risk_check.py 预检

| Grade | 处理 |
|-------|------|
| G0 (pass) | 正常买入 |
| G2 (caution) | 买入但记录警告, 收紧止损 |
| G3 (warning) | 拒绝买入 |
| G4 (block) | 拒绝买入 |

---

## Iron Rules（不可违反）

1. **NEVER** 盲跟 — 每个代币必须过安全检查, 不管目标钱包是谁。
2. **NEVER** 单次 balance=0 就认为钱包卖了。Solana RPC 有延迟, 必须连续 3 次确认。
3. **NEVER** poll 频率低于 15 秒。onchainos API 有限流, 太频繁会被 ban。
4. **写 positions.json 前必须持有 state lock。**
5. `contract-call` 返回 TIMEOUT 时, 总是创建 unconfirmed 仓位, 等后续确认。
6. 目标钱包地址变更需要重启 bot, 不支持热更新。
7. GAS_RESERVE 永不花在交易上。

---

## 故障排除

| 问题 | 解决 |
|------|------|
| "目标钱包无持仓" | 确认地址正确, 检查 `onchainos portfolio token-balances --address <addr> --chains solana` |
| 漏掉了目标钱包的一笔交易 | 钱包在两次 poll 之间买卖了同一代币。缩短 POLL_INTERVAL（但别低于 15s） |
| 买入失败 | 检查 SOL 余额 >= MIN_WALLET_BAL, 检查代币流动性 |
| Dashboard 打不开 | 检查端口 3248: `lsof -i:3248` |
| 登录过期 | `onchainos wallet login <email>` |
| API 限流 | POLL_INTERVAL 太短, 增加到 30-60 秒 |

---

## 参数调整

**所有可调参数都在 `config.py` 中**, 无需修改 `wallet_tracker.py`。

| 需求 | 修改 |
|------|------|
| 添加/更换目标钱包 | `TARGET_WALLETS = ["地址"]` (需重启) |
| 切换跟单模式 | `FOLLOW_MODE = "instant"/"mc_target"` |
| 调整目标市值 | `MC_TARGET_USD = 500_000` |
| 关闭跟卖 | `MIRROR_SELL = False` |
| 调整跟卖比例 | `MIRROR_SELL_PCT = 0.50` (卖一半) |
| 调整仓位大小 | `BUY_AMOUNT = 0.03` |
| 调整止盈 | `TP_TIERS = [(15,0.30),(30,0.40),(50,1.00)]` |
| 调整止损 | `STOP_LOSS_PCT = -20` |
| 调整轮询速度 | `POLL_INTERVAL = 30` (秒, 别低于 15) |
| 模拟交易 | `MODE = "paper"` |
