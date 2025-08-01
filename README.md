## Introduction
An Automated Market Maker (AMM) program built on the Solana
  blockchain using the Anchor framework. The on-chain program is developed in Rust and supports core DeFi
  functionalities such as token swaps, as well as depositing and withdrawing liquidity. The project also includes a
  client-side setup in TypeScript for testing and interaction.

## What is an AMM?
An Automated Market Maker (AMM) is the core engine of a decentralized exchange (DEX). Instead of using
  a traditional order book where buyers and sellers set their own prices, an AMM works like this:


   1. **Liquidity Pools**: It relies on liquidity pools, which are smart contracts holding reserves of two or more
      different tokens.
   2. **Liquidity Providers (LPs)**: Anyone can deposit a pair of tokens into a pool to become a liquidity provider. In
      return for providing capital, they earn a small fee from every trade that happens in that pool.
   3. **Algorithmic Pricing**: The price of the tokens is determined by a mathematical formula. The most common is the
      constant product formula (x * y = k). This formula ensures that as the supply of one token in the pool
      decreases (because people are buying it), its price relative to the other token automatically increases.
   4. **Swapping**: Traders interact directly with the smart contract (the pool) to swap one token for another, with the
      price being quoted by the algorithm at the moment of the trade.

  In short, AMMs replace the order book with an algorithm and a pool of assets, enabling instant, automated, and
  permissionless trading.

  ## Account

  `Config` - Solana account to store config options for liquidity pool

  * **seed**: A unique number that helps create a distinct address for this AMM pool.
  * **authority**: The "admin" or owner of the pool. This person can change settings like the trading fee.
  * **mint_x**: The address of the first token in the trading pair (e.g., SOL).
  * **mint_y**: The address of the second token in the trading pair (e.g., USDC).
  * **fee**: The small percentage fee that is charged on every swap.
  * **locked**: An on/off switch. If it's "on," all trading and depositing is paused.
  * **config_bump** & **lp_bump**: Technical values that help Solana find the program's accounts correctly. You can
       think of them as part of the address.

## Instructions

* **Initialize**: Setup the Pool
       * Creates a new, empty trading pool for a specific pair of tokens.


* **Deposit**: Add Liquidity
       * A user deposits a pair of tokens into the pool and gets LP tokens back.

* **Swap**: Trade Tokens
       * A user exchanges a certain amount of one token for the other.


* **Withdraw**: Remove Liquidity
       * A user returns their LP tokens and gets their share of the two tokens back.

## Tools
* Anchor: 0.31.1
* Solana: 2.1.15
