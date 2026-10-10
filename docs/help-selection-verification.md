# Help and selection family — 2026-10-10

## Delivered scope

- [Plain tooltip](tooltip-verification.md) and [persistent rich tooltip](rich-tooltip-verification.md).
- [Interactive popover](popover-verification.md) and [supplementary hover card](hover-card-verification.md).
- [Select](select-verification.md) and [native select](native-select-verification.md).
- [Editable single/multiple combobox](combobox-verification.md) and [free text autocomplete](autocomplete-verification.md).
- [Stable native HelpTrigger wrapper](help-trigger-verification.md).

The preview anchors are `#help-and-selection` and `#selection-fields`. Eight
copyable Rust examples are included. Each report records its reference choices,
input checks and limitations. Android remains paused.

## Reference inspection

Read the rendered official tooltip, text-field and menu **overview, specs,
guidelines and accessibility** pages on 2026-10-10. Those source links are in the
reports. Popover, hover card and autocomplete have no separate M3 component page;
they are upstream primitives or convenience APIs governed by the related
Material contracts.

Actually inspected upstream tooltip, popover, hover-card, select, native-select,
combobox, input-group and input sources at
`c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`, plus its
[shared motion styles](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/styles/m3e.css).
Read Base UI Tooltip, PreviewCard, Popover and [Combobox](https://base-ui.com/react/components/combobox)
documentation and APG Combobox.
Anchor-relative transform origins were compared to
[Base UI positioning source](https://github.com/mui/base-ui/blob/ad7ecda5296b0295b41a51ecb01fce6204acd099/packages/react/src/internals/useAnchorPositioning.ts).
This additional current Base UI source is a behavior reference, not a claim
that the pilot reproduces every version or all Floating UI options.

## Motion contract and measurements

The new shared runtime uses existing Material spring tokens, native Popover API,
generation-owned WAAPI animations and owned cleanup. A superseded exit cannot
hide a newer open. Resize/scroll placement, viewport clamping, opposite-side
flip, logical RTL alignment and anchor-relative origins are implemented.

| Surface | Inspected upstream contract | Runtime evidence |
| --- | --- | --- |
| Tooltip, rich help, popover, hover card | Scale .85→1: FastSpatial 360ms; opacity: DefaultEffects 240ms | Intermediate scale and opacity actually sampled, then settled geometry |
| Same exit | Scale→.95 and opacity→0, 150ms emphasized accelerate | Popover, tooltip and hover-card exit states sampled, then hidden |
| Select/combobox | ScaleY .6→1: DefaultSpatial 440ms; opacity: FastEffects 150ms | Select starts .6, width 463.5px stays constant, small overshoot, settled height 248px |
| Selection exit | ScaleY→.9, opacity→0, 120ms emphasized accelerate | Intermediate closing opacity/scale sampled, then hidden |
| Rapid activation | New generation owns completion | Enter/Escape/Enter ended open; no stale hide |

The spring arrays come from the existing copied upstream tokens; they were not
retuned by guessing durations. Popup origin now includes the side gap, aligned
edge, and anchor center after collision shifting. The final origin check is in
the raw evidence. Local popups use 8px viewport gutters.

[Raw samples](help-selection-samples.json) retain all computed frames and input
states. Sampling runs concurrently with actual keyboard/pointer actions through
read-only DOM inspection. The browser backend does not expose performance,
requestAnimationFrame or getAnimations in its evaluator; the sampler uses
external round trips and an external clock. Samples therefore prove visible
interpolation and final state, not exact event-to-frame timing. Pointer hover
was produced by a native drag released over the trigger, not a synthetic event;
those timings include the drag and observation delays.

## Accessibility and platform evidence

Actual tests cover Tab/Shift+Tab, Escape, arrows, Home/End, type-ahead, Enter,
pointer selection, disabled option/trigger rejection, native editing,
filtering/empty status, multiple selections, chip removal, controlled error
clearing and nonmodal focus exit. Group names, active-descendant focus and
selection checks are recorded. Rendered list/selected text contrast measured
15.51/6.22 in light mode and 13.17/5.69 in dark mode; plain tooltip text measured
11.65/10.17, respectively. These sampled pairs exceed 4.5:1; they do not establish
contrast for every custom seed, overlay or embedded content. AX was inspected; **audible VoiceOver output is
not verified**.

Actual macOS Reduce Motion was temporarily enabled under the user's prior
approval and restored **OFF**, confirmed through System Settings AX. Tooltip,
rich tooltip and popover visible samples settled at opacity 1 / transform none.
Select also settled, but its first visible reduced-motion sample had significant
RPC latency, so it does not establish its earliest frame. Other reduced-motion
component permutations remain broader verification work. VoiceOver was not changed.

A real desktop viewport override to **390×844** succeeded this turn. The new
family containers and fields did not overflow horizontally; chip targets were
48×48px and wrapped without local overflow. Real scrolling moved a 301×248px
select popup from below to above its anchor. The 288×280px popover fit the compact
viewport. This is desktop responsive evidence, **not Android touch evidence**.
The whole gallery still measured 409px document width at 390px because of older
large controls in other sections; that is separately recorded scope.

## Source copying and builds

All eight family copy controls returned the exact displayed snippet and showed
Copied. A real native clipboard paste of the 572-character Autocomplete snippet
into a single-line field produced 557 characters, exactly matching native
line-break removal. The preview test value was restored.

The complete source export includes `anchored.js`, `selection.js`, both new
stylesheets and their Rust/icon/motion/ripple dependencies. All **35** independent
usage examples compile in the scratch export. This verifies source/API
completeness; a separately launched consuming-app runtime remains open.

- Final live `./scripts/dev.sh` build passed in **17.06s**, including input-click and keyboard-loop updates.
- Final locked/offline Wasm check passed in **3.04s**. JavaScript syntax checks passed.
- Final export check with all 35 examples passed in **6.16s**; only the existing unused ChipSize
  import warning remains.
- `git diff --check` passed. No standalone Dioxus build ran concurrently with serve.
- Reloaded successful build: one `.app-shell`; no captured error/warning logs.
- Preview left running at `http://127.0.0.1:8080/`.

Final build metadata is retained in the raw samples. An initial browser reload
was denied by its URL policy; after the user's continuation and restored HTTP
page state, the direct preview access and reload succeeded. No alternate browser
surface was used to bypass the denied action.

## Differences and open work

Intentional deviations include Material's longer tooltip leave delay, 48px
options/removal targets, a real fieldset notch, full-width fields, persistent
scrollbars and field-width combobox popups. Detailed upstream differences and
optional API features are in each component report. They are not claimed as
pixel-for-pixel or complete Base UI API parity.

Remaining: Android touch/keyboard/native packaging, audible screen-reader output,
forced colors, large text, extended held/cancelled option-ripple input,
dynamic mount/anchor replacement stress, nested popup
and modal combinations, expanded native picker appearance, long-list virtualization
and consuming-app runtime. Transient rich help, custom collision boundaries,
virtual anchors, chip Backspace navigation and async suggestion sources are
optional additions rather than delivered APIs.

## Reproduce

Run `./scripts/dev.sh`; wait for success, reload and check one `.app-shell`.
Visit the family anchors and follow each component's matrix. Use real browser
input and read-only style sampling concurrently. For compact layout, apply and
verify a viewport override, then reset it. For reduced motion, use the real OS
preference under authorization and restore its original state. Save raw data
immediately and keep desktop and Android evidence separate.

Screenshots: [finished family preview](help-family-preview.png), [desktop selection](help-selection-desktop.png),
[dark selection](help-selection-dark.png), [desktop popover](help-popover-desktop.png),
[compact popover](help-popover-compact.png).

Follow-up: [editable trailing-action spacing and replacement](selection-trailing-action-verification.md)
corrects the previous dual clear/dropdown layout and records new desktop geometry.

Latest: [official-reference multi-selection and filled-arrow correction](multi-select-filled-alignment-verification.md)
keeps per-chip removal and centers Select icons independently of the value baseline.
