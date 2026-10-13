# Registry and installer verification — 2026-10-13

The local Python CLI installs editable Dioxus sources in the Rust/UI copyable
workflow. The content-addressed registry contains 82 modules and 76 public
entries, including internal dependencies, CSS, include_str JavaScript, theme
helpers and icon assets. Dependency resolution is conservative; it can include
extra helpers. No gallery module or Tailwind dependency is installed.

Every public entry was independently installed and Cargo checked for
wasm32-unknown-unknown: **76/76 pass**. [Raw results](registry-compile-checks.json)
record snapshot 982de1273d43. The final manifest additionally removes an extra
blank line at the end of the foundation lib and adds Google license attribution;
component implementations are unchanged from that selective compilation run.
The final complete export with 126 usages passed independently.

[Safety checks](installer-safety-checks.json) demonstrate preservation of a
local source edit when adding dialog, preflight rejection of conflicting edited
generated modules, and no mutation in dry-run. All-plan resolves 76 public
roots to 82 modules. Path traversal and symlink destinations are guarded.
Sources are checked against registry hashes before installation.

Actual installed-app evidence is in [the consumer report](consumer-app-verification.md).
For reproduction run `scripts/verify-registry.py --cargo /path/to/cargo
--target-dir /path/to/cache`; set CARGO_HOME/RUSTUP_HOME for the isolated tools.
Use [installation instructions](registry-installer.md) for a project.

Remaining distribution work: remote fetching/publication, versioned releases,
CI automation and registry schema stability. Per-file atomic writes do not
provide a whole-install transaction. Platform/API parity limitations remain
listed in the individual component reports.

The final manifest snapshot is recorded in [build results](distribution-build-results.json).
A fresh --all install wrote194 files and its generated library passed Wasm Cargo
check on that exact final snapshot.

## Release preparation —2026-10-13

Source integrity checks pass for the final registry, including pinned package
metadata. Four installer tests pass: edit preservation/conflict preflight/dry-run,
unsafe destination rejection, all-entry installation and deterministic archive
roundtrip. Repeated archives have identical SHA256. The extracted archive verifies
and installs its full editable kit. Its Wasm library check passed in16.41s before
the addition of the package provenance JSON; component Rust is unchanged.

GitHub Actions CI/release files parse and actionlint1.7.12 passes semantic
validation (optional shellcheck/pyflakes integrations disabled). Upstream package
and font download checks were exercised locally; font regeneration matches and
all25,806 paths compare. The drafted browser job is prepared for Chromium and
WebKit. Hosted CI is pending a push; publishing and tag creation are not performed.

[Release procedure](github-registry-release.md). Archive, sums and release.json
are in /private/tmp/m3e-release-final. The workflow prepares a draft after CI,
checks the exact tag/version and refuses overwriting an existing published release.

## First hosted CI run —2026-10-13

Commit990e3c3: Pages and the Chromium/WebKit icon job passed. The source job
passed pinned-artwork, registry and installer checks, then failed at the offline
showcase check because async-tungstenite0.31.0 was absent from the fresh cache.
The target-restricted prefetch did not cover the full compilation dependency set.
CI now fetches the locked dependency set without target filtering and separately
fetches the independent consuming app's lockfile before its offline check.
This changes CI dependency preparation; component sources are unchanged.
