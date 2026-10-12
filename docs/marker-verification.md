# Markers — verification,2026-10-12

## Scope and sources

`src/components/marker.rs`, `assets/expression-helpers.css`, markers page in
`src/expression_helpers_gallery.rs`: Marker/MarkerIcon/MarkerContent, Default,
Separator and Border variants, optional native href composition.

Inspected [marker.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/marker.tsx)
at `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`. There are no standalone Material
Marker component pages. Applicable current [typography](https://m3.material.io/styles/typography/applying-type),
[usability](https://m3.material.io/foundations/usability/overview) and
[accessibility principles](https://m3.material.io/foundations/overview/principles)
were reviewed in this session. This component groups metadata and native links;
the separator lines are decorative rather than additional accessibility widgets.

## Contract and measured results

- BodyMedium14/20, OnSurfaceVariant, min-height16px,8px gap,16px decorative icons.
- Default div measured20px high. Separator has1px OutlineVariant lines and4px
  extra line margins. Border is1px OutlineVariant plus8px bottom padding,29px total.
- Native optional href creates an underlined anchor with3px underline offset,
  hover OnSurface and visible focus. Local link target minimum48px improves the
  standalone target; source wrapper permits arbitrary rendered element composition.
- At390px, card content279px. Long separator caption wraps into60px height with
 255px content width, preserving line spacing; body/client375px, no page overflow.
- Tab from marker link focuses the copy control. Enter follows href to
  /#keyboard-hints; names and native link semantics are present in the browser tree.
- Both copied examples match exactly412 and198 characters after completion.

[Raw evidence](expression-helpers-samples.json),
[desktop](evidence/expression-helpers/markers-desktop.png),
[390px](evidence/expression-helpers/markers-390.png).
Markers have no intrinsic press/selection/motion state. Decorative icons are hidden
from AT; content and links remain exposed. CSS wrapping reserves spacing for lines.

## Build, gaps and reproduction

Actual Dioxus preview38.43s and independent95-example Wasm check pass; existing
unused ChipSize warning only. Native link/action integration was exercised in the
live gallery. Early clipboard reads returned a previous value; completion rechecks
returned the exact snippets.

Android remains paused. Audible screen readers, RTL, forced colors/zoom,
icon+long-separator combinations and arbitrary nested custom links/content are
untested. React useRender/mergeProps polymorphism is represented by a typed href
composition; arbitrary tags/ref APIs remain extensions.

Run `./scripts/dev.sh`, confirm success and reload Content → Markers. Inspect the
three variants, keyboard-focus and activate View keyboard hints. At390px inspect
the long caption and page bounds. Copy both examples with required icon/CSS files.
