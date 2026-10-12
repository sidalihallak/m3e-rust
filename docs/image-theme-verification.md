# Image-derived theme verification — 2026-10-12

## Scope and reference contract

Sources: `components/image_theme.rs`, `image_theme.js`, `src/theme.rs`,
`theme-helpers.css`, gallery and fixtures in `docs/evidence/theme-helpers`.
Official current [color system](https://m3.material.io/styles/color/system/how-the-system-works)
and [color roles](https://m3.material.io/styles/color/roles), read rendered on
2026-10-12, describe quantizing imagery into a source color, generating tonal
palettes, and keeping accessible paired roles in content-based schemes.
No official ImageThemePicker component geometry/interaction specification exists.

Inspected actual upstream revision c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972:
[use-image-theme](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/use-image-theme.ts),
[vibrant](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/lib/vibrant.ts)
and [color](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/lib/color.ts).
Upstream uses node-vibrant/browser quality3, six named swatches, Vibrant-first
seed, Muted secondary, a hue-distant tertiary (>20 degrees), and DarkMuted neutral.
It accepts File/Blob/URL/image/canvas/bitmap/ImageData and reports loading/error,
preview and swatch choice. Local image extraction intentionally uses the already
adopted Rust Material Color Utilities rather than claiming Vibrant palette parity.

## Implemented contract

Native labeled file input with image accept hint, disabled state, 48px minimum
height, focus ring, status region and aria-busy. Browser createImageBitmap decodes
local files, maximum20MB; longest sample side128px, preserving aspect ratio.
Only alpha255 pixels contribute. Rust Celebi quantizes up to128 clusters; HCT
Score ranks up to4 candidates, baseline #6750a4 fallback for neutral images.
image_seed_candidates is also public for opaque decoded native pixels, with a
16384-pixel cap. The application decides whether/where to apply the emitted seed.
The gallery uses a Content scheme and appropriate paired text colors.

Generation ownership ignores stale async results. Bitmaps close in finally;
object URLs are revoked on replacement/unmount. A failed image preserves the
last usable preview/candidates/theme and reports the error. The file input resets
after each current attempt, allowing repeated selection of the same file. No
upload service, remote URL retrieval or image byte persistence is involved.

## Evidence and gaps

Raw desktop/compact/clipboard results: `docs/theme-helpers-samples.json`.
Fixture seed-fixture.png is96x64 with50% teal #006a60,25% rose #9c4146 and25%
violet #6750a4. The actual Wasm picker ranks teal, violet, rose and emits teal.
Preview natural dimensions96x64; aria-busy clears. Keyboard Enter chooses rose
and applies its scheme. Disabled input is native-disabled.

The transparent-error and corrected Content contrast results are recorded below.
Invalid decode, neutral browser input and repeated same-file selection remain untested. This desktop native chooser result is not an
Android document-picker/touch result. Android remains paused. Audible readers,
forced colors, zoom, rapid overlapping large decodes/unmount cancellation,
20MB boundary, EXIF/wide-gamut formats and native Dioxus image decoding remain
unverified. Arbitrary upstream image input objects, Vibrant multipalette parity,
async worker quantization and persistence are extensions.

## Reproduce

Wait for successful ./scripts/dev.sh build, reload /#image-themes. Select the
saved seed fixture, choose another swatch with Enter/Space, then try neutral,
transparent and invalid files. Check error/status and the retained scheme.
Use the same seed file again to check input reset; copy and compile the snippet.

## Final results

The corrected teal Content pair is #95e7da on #006a60; rose is #ffcdcc on
#9c4146. Their actual sRGB contrast ratios are4.56:1 (rounded), recorded in the raw JSON. The
selected candidate is a native aria-pressed button; Space chooses rose. A
fully transparent PNG reports “No opaque colors found. Choose another image.”,
clears busy, and preserves the preceding rose theme and visible valid preview.
These image results were captured before the final scoped color-transition pause;
seed generation and picker behavior did not change in that final addition.

390px testing initially found a343.5px intrinsic file input overflowing its277px
content track. Explicit minmax(0,1fr) grid tracks and width100%/min-width0 corrected
it. Final input277×48px, scope327px, body375px within390px; native disabled input
confirmed. Screenshot: docs/evidence/theme-helpers/image-390.png.
Exact native keyboard clipboard match:379 characters. Final Wasm/source-export
results are shared with the theme scope report. No Android or image pixel
algorithm parity with Vibrant is claimed.
