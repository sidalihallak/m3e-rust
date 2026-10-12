# Item compositions verification

## Scope and references

2026-10-12. `src/components/item.rs`, `item.js`, `assets/item.css`, and
`src/item_gallery.rs`. Item/Group/Media/Content/Title/Description/Actions/Header/
Footer/Separator are copyable Dioxus components.

Read the current rendered official Material list [overview](https://m3.material.io/components/lists/overview),
[specs](https://m3.material.io/components/lists/specs),
[guidelines](https://m3.material.io/components/lists/guidelines), and
[accessibility](https://m3.material.io/components/lists/accessibility).
Inspected actual [item.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/item.tsx), shared
[m3e.css](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/styles/m3e.css), and the existing kit List/Divider/Ripple.
Upstream revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

## Contract and choices

| Property | Source and implementation |
| --- | --- |
| Variants | Default transparent, Outlined outline-variant, Muted surface-container, Segmented surface |
| Default | min-height56px, padding10px16px, gap12px, corners16px |
| Small | min-height48px, padding8px12px, gap12px |
| XS | padding10px12px, gap10px; local interactive minimum48px for Material targets |
| Group gaps |16px; any small10px; any XS8px; segmented2px |
| Segmented corners |4px inner,16px ends; hover12px; focus/press/selected16px |
| Media | icon16px; image56/40/24px,12px corners; align at top+2px with descriptions |
| Content | flexible width, gap4px (XS2px), title16/24; description14/20, maximum2 lines |
| Actions |8px gap; outside primary actions in noninteractive rows |
| Header/Footer |full-width flex slots,8px gap |
| Separator |existing1px outline-variant Divider,8px block margins |
| Motion |DefaultEffects240ms corners, FastEffects150ms colors, existing Material ripple |

The current official list color diagram specifies primary-container selection;
upstream Item uses secondary-container. This port follows Material and the kit's
existing List, with on-primary-container content. Source static Item has no ripple;
interactive rows add the existing kit ripple to follow official list accessibility
feedback, retaining its generation-owned press lifecycle. Static source button
render overrides are replaced by typed interactive/href props and native controls
inside listitem wrappers. Span-based slots retain valid phrasing content inside
buttons. Interactive rows must not contain nested controls. Put ItemActions in a
noninteractive row; use existing List for primary+secondary row layouts.

ItemGroup selection=None uses list/listitem semantics; Single/Multiple use
listbox/options with aria-selected and aria-multiselectable. Caller controls
selection and includes an additional visible checkmark. Groups use one enabled
Tab stop, arrow navigation across controls (RTL-aware), Home/End and skip disabled
controls. Text entry controls retain their own arrow behavior. On exit, the Tab
entry resets to a selected option or the first enabled action. Selection groups
expect every Item to have selected=Some(bool), no href, and no secondary controls.

## Runtime checks

- Default composite134px high; small link66px; XS content68px. Two-line segmented
  rows70px, showing source padding/content sizing rather than claiming baseline72px.
- Pointer Team selection, Space/Enter Organization selection/deselection passed.
  ArrowDown Team→Organization; secondary Save→link navigation passed. Selected
  Team became the single Tab entry again after leaving from Organization.
- Native disabled row is disabled and excluded from focus. Tab from Organization reached the Copy Rust code button; Home/End reached Personal project/Organization. Save brief pointer→true and Space→false were confirmed. No nested buttons or
  anchor-buttons were present in the showcase.
-40 actual input samples: middle row corners4px→5.758px at44ms→12.095px at93ms→
  15.964px at242ms→16px at309ms. Width778.609375px and height70px stayed constant.
  Primary-container background resolved rgb(233,221,255) selected.
- Actual Reduce Motion: native Space selected Team,16px corners, transition:none;
  restoration confirmed preference=false.
-390px viewport: body375px; rows279px wide ending at327px; compact two-line rows
 70/90px high. Icon media16px and XS image24px measured. No page overflow.
- Exact copies: Item501 characters and Segmented Item617. An early clipboard read
  returned the previous406-character snippet; retry and a subsequent tool round
  trip confirmed exact Item content. Raw attempts are retained.

## Remaining scope

Full React render/mergeProps forwarding, virtualization, swipe/expansion engines,
all image sizes and multiple-selection permutations are not runtime verified.
Stable keys are required for dynamic rows. Caller owns validation of selection
mode/value consistency. Native anchor URL and unsupported nesting combinations
remain caller composition constraints.

## Build and evidence

Live Dioxus Wasm builds succeeded; the latest clean serve build took14.71s. The
standalone exported source kit and all112 usage examples passed the offline Wasm
check in11.93s, with only the pre-existing ChipSize unused-import warning.

Raw browser evidence: [samples](item-spinner-samples.json). Desktop and390px
screenshots are in `docs/evidence/item-spinner/`. One showcase-shell was confirmed.
The OS Reduce Motion preference was tested off→on→off and the original settings
panel restored. Desktop compact-width evidence does not establish Android touch
behavior. Android remains paused; native packages and audible screen readers are
not verified. Inter differs from upstream Roboto Flex. All theme/contrast, RTL,
held-pointer/cancellation and dynamic nested/unmount permutations remain broader
coverage tasks.

## Reproduce

Run `./scripts/dev.sh`, wait for its successful build and reload the preview.
Use native pointer/Space/Enter and arrow/Tab input, compare copies against the
rendered code, sample computed styles during interaction, then resize to390px
and restore the viewport. For reduced motion, toggle the actual OS preference,
confirm matchMedia and sampled animation/state behavior, and restore it.
Preview: `http://127.0.0.1:8080/#items`.
