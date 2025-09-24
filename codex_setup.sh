#!/usr/bin/env bash
set -euo pipefail

# Ensure the vendored dependency tree is up to date so `cargo test --offline`
# works inside the evaluation environment.
if command -v rustup >/dev/null 2>&1; then
  rustup component add clippy >/dev/null
fi

# Pre-fetch and vendor all crate dependencies referenced by the lockfile.
rm -rf vendor
cargo fetch --locked
cargo vendor --locked vendor

# Configure Cargo to prefer the vendored sources over crates.io.
mkdir -p .cargo
cat > .cargo/config.toml <<'CFG'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
CFG

cat <<'MSG'
Vendored dependencies written to ./vendor
Cargo configured to operate in offline mode using vendored crates.
You can now run `cargo test --offline` without network access.
MSG
