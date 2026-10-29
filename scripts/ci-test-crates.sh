#!/usr/bin/env bash
# Discover and test every package Cargo.toml outside archive/, excluding
# workspace roots/members that are covered elsewhere and shop-front (wasm job).
set -euo pipefail

workspace_member_manifests() {
  local root="$1"
  python3 - "$root" <<'PY'
import pathlib, re, sys
root = pathlib.Path(sys.argv[1])
text = (root / "Cargo.toml").read_text()
if not re.search(r"(?m)^\[workspace\]", text):
    raise SystemExit(0)
m = re.search(r"(?ms)^members\s*=\s*\[(.*?)\]", text)
if not m:
    raise SystemExit(0)
for raw in re.findall(r'"([^"]+)"', m.group(1)):
    path = (root / raw / "Cargo.toml").as_posix()
    if not path.startswith("./"):
        path = "./" + path.lstrip("./")
    print(path)
PY
}

EXCLUDE_FILE=$(mktemp)
trap 'rm -f "$EXCLUDE_FILE"' EXIT

{
  echo "./ch13-wasm/shop-front/Cargo.toml"
  for ws in ./ch12-building-an-api; do
    if [ -f "$ws/Cargo.toml" ]; then
      echo "$ws/Cargo.toml"
      workspace_member_manifests "$ws"
    fi
  done
} > "$EXCLUDE_FILE"

is_excluded() {
  grep -qxF "$1" "$EXCLUDE_FILE"
}

has_package() {
  grep -q '^\[package\]' "$1"
}

while IFS= read -r manifest; do
  if is_excluded "$manifest"; then
    continue
  fi
  if ! has_package "$manifest"; then
    continue
  fi

  dir=$(dirname "$manifest")
  echo "::group::$manifest"

  if [ -d "$dir/solution" ]; then
    echo "starter: cargo test --no-run"
    cargo test --no-run --manifest-path "$manifest"
    echo "apply solution/*.rs → src/"
    cp "$dir"/solution/*.rs "$dir"/src/
    echo "solution: cargo test"
    cargo test --manifest-path "$manifest"
    echo "solution: cargo clippy -D warnings"
    cargo clippy --manifest-path "$manifest" --all-targets -- -D warnings
  else
    echo "cargo test"
    cargo test --manifest-path "$manifest"
    echo "cargo clippy -D warnings"
    cargo clippy --manifest-path "$manifest" --all-targets -- -D warnings
  fi

  echo "::endgroup::"
done < <(
  find . \
    \( -path './archive' -o -path '*/target' -o -path './.git' \) -prune -o \
    -name Cargo.toml -print \
    | sort
)
