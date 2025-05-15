# Solana Hackathon Project - Dissident Finance

## Overview

This part of the project that monitors the yield on crypto products from the Meteora and Orderly protocols.

A Rust-based service that aggregates crypto yield products from multiple protocols (Meteora pools and Orderly vaults) with CLI and API access. Developed for [La Familia Barcelona Hackathon](https://linktr.ee/lafamilia.so) Demo Day, this project showcases real-time data aggregation capabilities for decentralized finance protocols.

## Features

- **Multi-protocol Connectors**:
  - Meteora Pool Connector (part of Jupiter Aggregator - [jup.ag](https://jup.ag)): Fetches concentrated liquidity pools with APR/TVL metrics
  - Orderly Vault Connector: Retrieves structured vault products and LP performance data using Orderly's [Strategy Vault API](https://orderly.network/docs/strategy-vault/liquidity-provider/public/lp-performance-chart), including:
    - 30-day APY metrics
    - TVL history
    - Profit & Loss tracking
    - Liquidity provider statistics
  
- **Data Storage**:
  - PostgreSQL database with automatic schema migrations
  - Time-series tracking of pool/vault metrics

- **Command Line Interface**:
  ```bash
  # Sync data from all protocols
  cargo run -- sync
  
  # List products sorted by APR
  cargo run -- list
  
  # Start API server (default port 3000)
  cargo run -- server
  ```
  
  ## API Endpoints
  Get /pools - Returns aggregated yield products

  ## Response Structure
  ```json
  {
    "data": [
      {
        "type": "meteora_pool",
        "data": {
          "pool_address": "Ft8FD9gg1TdawhWijNzdkpYTiVL8ETfY6gAwS24bwYio",
          "pool_name": "SOL-USDC",
          "token_a_symbol": "SOL",
          "token_b_symbol": "USDC",
          "apr": 15.23,
          "tvl": 450000.50,
          "volume24h": 37436.45,
          "fee24h": 97.52
        }
      },
      {
        "type": "vault",
        "data": {
          "vault_address": "0x123...abc",
          "vault_id": "ETH-USD-PERP",
          "apy_30d": 12.45,
          "tvl": 1200000.00,
          "vault_lifetime_net_pnl": 45000.00
        }
      }
    ]
  }