#!/bin/bash
set -euo pipefail

echo starting process

# temporary ledger that will be auto-cleaned
DATA_DIR=$(mktemp -d)  

# fn to cleanup on exit
cleanup() {
    echo cleanup
    if [[ -n "${VALIDATOR_PID:-}" ]]; then
        kill "$VALIDATOR_PID" 2>/dev/null || true
    fi
    rm -rf "$DATA_DIR"
    echo "Done."
}
trap cleanup EXIT

# Start fresh validator as a child process
echo "Starting solana-test-validator..."
solana-test-validator --ledger "$DATA_DIR" --reset --quiet &
VALIDATOR_PID=$!

until solana cluster-version &>/dev/null; do
    sleep 0.05
done

echo setting configs...
solana config set --url http://127.0.0.1:8899

echo "Validator running (PID: $VALIDATOR_PID)"
echo "Press Ctrl+C to stop validator and cleanup"

cargo run --manifest-path ./gothmog/Cargo.toml

# Keep script alive so validator stays alive
wait $VALIDATOR_PID
