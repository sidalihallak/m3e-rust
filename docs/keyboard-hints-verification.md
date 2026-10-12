# Keyboard hints — verification,2026-10-12

## Scope and references

`src/components/kbd.rs`, `assets/expression-helpers.css`, keyboard-hints page in
`src/expression_helpers_gallery.rs`: Kbd and KbdGroup are static native keyboard
notation. The app owns any shortcut bindings and platform-specific key choice.

Inspected actual [kbd.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/kbd.tsx)
at `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`. Material has no separate Kbd
component pages. Applicable official guidance: [typography](https://m3.material.io/styles/typography/applying-type),
[current corner scale](https://m3.material.io/styles/shape/corner-radius-scale),
[accessibility principles](https://m3.material.io/foundations/overview/principles).
Typography/accessibility were reviewed during this session; corner scale was read
rendered on2026-10-12. The4dp corner is ExtraSmall. Source supplies the hint geometry.

## Contract and runtime evidence

| Property | Source and measured implementation |
| --- | --- |
| Native elements | kbd for individual key and outer group; no added focus stop |
| Box |24px height,24px minimum width,6px inline padding,4px corners |
| Type |LabelMedium50012/16, sans-serif |
| Color |SurfaceContainerHighest / OnSurfaceVariant |
| Spacing |4px group gap;12px optional icon size |
| Interaction |Keycaps pointer-events:none; consuming control owns activation |

Optional key_label exposes readable modifier names through visually hidden text
while hiding the corresponding decorative glyph. The composed native button is
named “Demo action Enter.” Pointer click, Enter and Space each activated it;
status reached3. Kbd itself neither registers a hotkey nor acts as a button.

Light roles measured rgb230,224,233 background /73,69,78 text; dark roles
54,52,58 /202,196,207. Light restored. At390px viewport, body/client375px, no
horizontal overflow. Copy control exactly matches212 characters after completion.

[Raw evidence](expression-helpers-samples.json),
[desktop](evidence/expression-helpers/kbd-desktop.png),
[390px](evidence/expression-helpers/kbd-390.png).
This helper has no intrinsic motion. Existing Button feedback is retained.

## Build, gaps and reproduction

Actual Dioxus preview build38.43s succeeded. All95 independent exported usages
compile for Wasm; existing unused ChipSize import warning only. Later exporter
refresh includes third-party notices and shape dependencies.

Android is paused. Audible screen-reader pronunciation, zoom/forced colors,
RTL key notation, platform detection and real hotkey bindings remain outside
this sampled helper scope. Glyph names are app supplied. No OS preference is
needed for this static component.

Run `./scripts/dev.sh`, wait for successful build, reload Foundations → Keyboard
hints. Inspect native keycaps, activate the parent demo button with pointer,
Enter and Space, check390px layout, then copy/compile the example.
