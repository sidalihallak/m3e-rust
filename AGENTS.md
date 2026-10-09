# Agent instructions

This repository is a Dioxus Material 3 Expressive UI kit, porting
[shadcn-m3e](https://github.com/Crysta1221/shadcn-m3e) with high visual and
interaction fidelity and a Rust/UI style copyable component workflow.

Read [the component development protocol](docs/component-development.md) before
adding or changing a component. Apply it to **every component**, including
follow-up fixes. Create or update that component's verification report.

## Project boundaries

- Work in this repository. The existing
  `/Users/sidalihallak/Desktop/dev/rskit/crates/rskit-ui` implementation is outside
  this pilot's scope unless the user explicitly changes that instruction.
- Use Dioxus and Rust for the component API. Use the existing Material color
  roles and motion tokens. Keep component code and its required CSS copyable.
- Read the current official Material overview, specs, guidelines and
  accessibility guidance for the component, and inspect its actual shadcn-m3e
  source. Keep source links and observed values in the verification report.
- Preserve native button semantics, keyboard behavior, reduced motion,
  disabled state and selection semantics. Test the interactions relevant to
  the component; do not infer them from CSS declarations.
- Keep desktop and mobile evidence separate. A desktop browser result does
  not establish Android touch behavior. State gaps accurately.
- Do not tune motion by guessing durations or exaggerating scale changes.
  Diagnose events, state, CSS interpolation, layout and reference tokens first.

## Preview and tools

Run `./scripts/dev.sh` to serve the live Dioxus preview on
`http://127.0.0.1:8080/`. Verify that Dioxus reports a successful build, then
reload the browser before measuring.

The pilot's existing isolated tools are in `/private/tmp/m3e-cargo-home`,
`/private/tmp/m3e-rustup` and `/private/tmp/m3e-dx`. The script selects them when
present. Check these paths before declaring that Rust, the wasm target or
Dioxus CLI is missing. Temporary directories may disappear after a restart.

For a Rust/Wasm check with those tools:

```sh
env CARGO_HOME=/private/tmp/m3e-cargo-home \
    RUSTUP_HOME=/private/tmp/m3e-rustup \
    /private/tmp/m3e-cargo-home/bin/cargo check --locked --offline --target wasm32-unknown-unknown
```

Current button evidence:
[report](docs/button-motion-verification.md),
[raw browser samples](docs/button-motion-samples.json).
Android preview setup and current mobile scope:
[Android preview](docs/android-preview.md).

Before calling a component verified, record the build result, tested input
types and states, measured motion, reference comparison, and unresolved gaps.
Keep the preview running for the user when requested.
