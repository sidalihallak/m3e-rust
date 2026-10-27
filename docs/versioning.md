# Versioning and releases

`m3e-rust-ui` follows [Semantic Versioning](https://semver.org/). The public API is everything
exported from the crate root (`src/lib.rs`) and the items they reach.

## Version numbers

- **0.x.y (current):** the API may change between minor versions. A breaking change bumps `x`
  (for example 0.1.0 to 0.2.0) and is listed under a `Changed` or `Removed` heading in
  [CHANGELOG.md](../CHANGELOG.md). A fix or a compatible addition bumps `y`.
- **1.0.0 and later:** breaking changes bump the major version, additions bump the minor version,
  and fixes bump the patch version.

## Releasing

1. Move the `Unreleased` entries in [CHANGELOG.md](../CHANGELOG.md) under a new version heading.
2. Set `version` in `Cargo.toml` to the same number.
3. Run the checks in [docs/quick-start.md](quick-start.md#verify-the-crate) and the CI workflow.
4. Commit, then tag the commit: `git tag -a vX.Y.Z -m "m3e-rust-ui X.Y.Z"` and
   `git push origin vX.Y.Z`.

Apps pin a version with a Git tag:

```toml
m3e-rust-ui = { git = "https://github.com/sidalihallak/m3e-rust", tag = "v0.1.0" }
```

Publishing to crates.io is not set up yet. A crates.io release needs a license file, which the
repository does not have yet.
