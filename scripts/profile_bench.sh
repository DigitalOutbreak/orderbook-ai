#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

SAMPLE_SIZE="${SAMPLE_SIZE:-20}"

echo "Running throughput benchmark with sample size ${SAMPLE_SIZE}"

if [[ "$(uname -s)" == "Darwin" ]]; then
  /usr/bin/time -l cargo bench --bench throughput -- --sample-size "${SAMPLE_SIZE}"
else
  /usr/bin/time -v cargo bench --bench throughput -- --sample-size "${SAMPLE_SIZE}"
fi
