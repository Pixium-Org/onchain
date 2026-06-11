# Pixium Smart Contracts

Soroban smart contracts powering the Pixium collaborative pixel canvas on the Stellar network.

---

## Overview

This repository contains all on-chain logic for Pixium, written in Rust using the [Soroban SDK](https://soroban.stellar.org). The contracts are the source of truth for the game — pixel placement, ownership, cooldown enforcement, quests, factions, color voting, and round management all live here.

---

## Contracts

| Contract | Description |
|---|---|
| `canvas` | Core contract — stores pixel state, enforces cooldowns, tracks ownership |
| `quests` | Quest definitions, completion verification, and extra pixel rewards |
| `factions` | Faction creation, membership, and shared pixel allocations |
| `votes` | Daily color voting — tallies votes and updates the active palette |
| `rounds` | Round lifecycle — start, end, canvas snapshot triggers |

---

## Tech Stack

- **Rust** — systems language for safe, performant smart contracts
- **Soroban SDK** — Stellar's native smart contract framework
- **Stellar Testnet / Mainnet** — deployment targets

---

## Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add the wasm32 target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features opt
```

### Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Test

```bash
cargo test
```

### Deploy (Testnet)

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/canvas.wasm \
  --network testnet \
  --source <YOUR_ACCOUNT>
```

---

## Contract Interface (Canvas)

```rust
// Initialize a new canvas
fn initialize(env: Env, width: u32, height: u32, palette: Vec<Color>, round_duration: u64);

// Place a pixel — enforces cooldown on-chain
fn place_pixel(env: Env, player: Address, x: u32, y: u32, color: u32);

// Read the color of a pixel
fn get_pixel(env: Env, x: u32, y: u32) -> Pixel;

// Get the owner of a pixel
fn get_owner(env: Env, x: u32, y: u32) -> Address;

// Check remaining cooldown for a player (in seconds)
fn get_cooldown(env: Env, player: Address) -> u64;
```

---

## Events

The contracts emit the following events that the [indexer](../indexer) listens to:

| Event | Payload | Description |
|---|---|---|
| `PixelPlaced` | `(player, x, y, color)` | A pixel was successfully placed |
| `QuestCompleted` | `(player, quest_id)` | A player completed a quest |
| `FactionCreated` | `(faction_id, name, founder)` | A new faction was created |
| `VoteCast` | `(player, color)` | A vote was cast for a color |
| `RoundEnded` | `(round_id, timestamp)` | A game round ended |

---

## Project Structure

```
onchain/
├── contracts/
│   ├── canvas/
│   ├── quests/
│   ├── factions/
│   ├── votes/
│   └── rounds/
├── tests/
└── Cargo.toml
```

---

## Contributing

See the root [contributing guide](#). For contract work, use `rustfmt` and `clippy` before submitting a PR.

```bash
cargo fmt
cargo clippy -- -D warnings
```

Branch format: `feature/<issue-number>-short-description`

---

## Related Repos

- [`backend`](https://github.com/Pixium-Org/backend) — API and WebSocket services
- [`indexer`](https://github.com/Pixium-Org/indexer) — Stellar event indexer
- [`frontend`](https://github.com/Pixium-Org/frontend) — Next.js player interface
