#!/usr/bin/env bash
set -euo pipefail
cd /workspace
export DATABASE_URL="${DATABASE_URL:-postgres://shop:shop@postgres:5432/shop}"
if [ -d ch12-building-an-api/shop-api/migrations ]; then
  (cd ch12-building-an-api/shop-api && sqlx migrate run) || true
fi
cargo fetch --manifest-path ch12-building-an-api/Cargo.toml || cargo fetch || true
echo "Devcontainer ready. API :8090, Trunk :8091, DATABASE_URL=$DATABASE_URL"
