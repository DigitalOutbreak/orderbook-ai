#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

TARGET="${1:-order_flow}"
RUNS="${RUNS:-0}"

if ! cargo fuzz list >/dev/null 2>&1; then
  echo "cargo-fuzz is not installed. Run: cargo install cargo-fuzz" >&2
  exit 1
fi

if ! rustup toolchain list | grep -q '^nightly'; then
  echo "nightly toolchain is not installed. Run: rustup toolchain install nightly --profile minimal" >&2
  exit 1
fi

CORPUS_DIR="fuzz/corpus/${TARGET}"
mkdir -p "$CORPUS_DIR"

exec cargo +nightly fuzz run "$TARGET" "$CORPUS_DIR" -- -runs="${RUNS}"
