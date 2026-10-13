# GitHub-hosted registry releases

GitHub is sufficient: use the repository for editable source and GitHub Releases
for immutable version choices. No separate website, npm registry or crates.io
publication is required for this copyable workflow. GitHub Pages remains the
showcase. Public release assets can be downloaded without credentials; private
repository assets require access and authentication, for example through `gh`.

## Consume a release

After a release is published (the following version is an example):

```sh
gh release download v0.1.0 --repo sidalihallak/m3e-rust \
  --pattern 'm3e-registry-0.1.0.tar.gz' --pattern SHA256SUMS \
  --dir /path/to/registry-download
cd /path/to/registry-download
# macOS:
shasum -a 256 -c SHA256SUMS
# Linux: sha256sum -c SHA256SUMS
tar -xzf m3e-registry-0.1.0.tar.gz
cd m3e-registry-0.1.0
python3 scripts/verify-distribution.py
python3 scripts/m3e.py add button text-field material-symbol --target /path/to/app
```

Use Python 3.11+. Follow the printed path dependency instruction, render
`UiStyles {}` once and apply theme variables. Components are editable files.
Pin a version tag or commit for reproducible projects rather than tracking main.
An authenticated clone of a private tag works too; then run the same installer
from that checkout. Never put access tokens in project source or URLs.

## Prepare a release

1. Update Cargo.toml version and release notes, then regenerate the registry.
2. Run CI on the commit that will be tagged.
3. Create and push a version tag matching Cargo.toml, such as v0.1.0.
4. The release workflow reruns CI on that tag, packages the exact tagged sources,
   and uploads a **draft** release with tar.gz, SHA256SUMS and release.json.
5. Review the draft and publish it in GitHub when ready.

No tag, push or hosted release is created merely by adding these workflows.
Existing published assets are not overwritten by the draft workflow.

Local packaging:

```sh
python3 scripts/build-registry.py --check
python3 scripts/package-registry.py --version 0.1.0 --output /tmp/m3e-release
```

The archive is deterministic (sorted files, normalized permissions and timestamps)
and carries every registry-referenced source, the installer and license notices.
SHA256SUMS detects corruption; it is not a signature or proof of publisher identity.
The release.json records source hashes and registry snapshot. Keep archives and
checksums from the same version. The installer rejects stale source hashes.

## CI scope

CI checks pinned upstream SVG/font artwork, source/sprite consistency, installer conflict/preservation behavior,
each installable entry for Wasm, a complete install, and the independent consumer.
The browser job builds actual Dioxus Wasm and compares live external sprites
with direct SVG paths in Chromium and WebKit, including all families/fill states.
Evidence is uploaded as workflow artifacts. CI does not establish Android touch
or audible screen-reader support. Workflows need a pushed commit to execute on
GitHub; local validation cannot establish a hosted run has passed.
