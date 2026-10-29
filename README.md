# php-to-rust

[![CI](https://github.com/stuarttodd-dev/php-to-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/stuarttodd-dev/php-to-rust/actions/workflows/ci.yml)

Companion crates for the [PHP to Rust](https://php2rust.dev) course: small, runnable examples that match the lessons on ownership, traits, concurrency, Axum APIs, and WebAssembly.

## Setup

**Local:** install [rustup](https://rustup.rs), then:

```bash
git clone https://github.com/stuarttodd-dev/php-to-rust.git
cd php-to-rust
rustup target add wasm32-unknown-unknown   # for ch13
export DATABASE_URL=postgres://shop:shop@127.0.0.1:54329/shop  # for shop-api
```

**Devcontainer / Codespaces:** open this repo in a Codespace (or VS Code Dev Containers). Postgres 16, wasm32, trunk, wasm-pack, and sqlx-cli are installed; ports 8090 (API) and 8091 (Trunk) are forwarded.

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://github.com/codespaces/new?hide_repo_select=true&ref=main&repo=stuarttodd-dev%2Fphp-to-rust)

## Lesson → crate map

| Chapter | Lesson | Folder | Command |
|---|---|---|---|
| Getting started | Hello Rust | `1-hello-world` | `rustc hello.rs && ./hello` |
| Getting started | CLI calculator | `2-cli_calculator` | `cargo run` |
| Rust basics | Input classifier | `3-user-input-classifier` | `cargo test` |
| Ownership | Ownership exercise | `5-basic-cart` | `cargo test` |
| Structs | Shop domain | `ch04-structs-enums-modules/shop` | `cargo test` |
| Lifetimes (depth) | Lifetimes in practice | `ch04-lifetimes/lifetimes-practice` | `cargo test` |
| Error handling | Shop / orders | `ch05-error-handling/*` | `cargo test --manifest-path …` |
| Traits | Discount rules | `ch06-traits/discounts` | `cargo test` |
| Macros (depth) | Declarative macros | `ch06-traits/macros` | `cargo test` |
| Closures | Collection exercise | `4-laravel-collection-clone` | `cargo test` |
| Testing | Find the bugs | `ch08-testing/find-the-bugs` | `cargo test` |
| Smart pointers | Category tree | `ch09-smart-pointers/category-tree` | `cargo test` |
| CLI | Shop CLI | `ch10-cli/shop-cli` | `cargo test` |
| CLI | Paths / inventory | `29-inventory` | `cargo test` |
| Concurrency | Order pipeline | `ch11-concurrency/order-pipeline` | `cargo test` |
| Concurrency | Async in depth | `ch11-concurrency/async-depth` | `cargo test` |
| API | shop-api | `ch12-building-an-api/shop-api` | `cargo test -p shop-api` (from `ch12-building-an-api`) |
| API | Reviews capstone | `ch12-building-an-api/reviews` | `cargo test` |
| WASM | shop-pricing / shop-front | `ch13-wasm/*` | `cargo test` / `trunk serve` |

Historical numbered folders that are no longer linked from the course live under [`archive/`](archive/README.md).

## How CI checks the course

GitHub Actions (`.github/workflows/ci.yml`) runs:

1. **crates** — fmt/clippy/test for the ch12 workspace (with Postgres 16 + sqlx migrate) and `cargo test --manifest-path` for other chapter crates.
2. **wasm** — `wasm32-unknown-unknown` builds for `shop-pricing` and `shop-front`, plus native tests for shop-pricing.

The LMS repo separately runs a site build and `scripts/check-snippets.py` over lesson fences.

## License

See repository license file (MIT unless noted otherwise in individual crates).
