#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

# Reuse the isolated toolchain installed for this pilot when available.
if [[ -x /private/tmp/m3e-dx/bin/dx && -x /private/tmp/m3e-cargo-home/bin/rustup ]]; then
  export CARGO_HOME=/private/tmp/m3e-cargo-home
  export RUSTUP_HOME=/private/tmp/m3e-rustup
  export PATH="/private/tmp/m3e-cargo-home/bin:/private/tmp/m3e-dx/bin:$PATH"
fi

exec dx serve --platform web --addr 127.0.0.1 --port 8080 --open false --interactive false
