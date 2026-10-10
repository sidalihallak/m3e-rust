# Menus — 2026-10-10

## Scope and references

Files: `src/components/menu.rs`, `menu.js`, shared action/ripple helpers,
`assets/menu.css`, `action-control.css`, `composite-motion.css`.
Supported: controlled Menu, native DropdownMenu trigger, focusable ContextMenu
region, standard/vibrant color, Expressive/baseline appearance, submenus, labels,
dividers, actions, checkbox/radio selection, supporting/trailing text and disabled items.

Read rendered Material [overview](https://m3.material.io/components/menus/overview),
[specs](https://m3.material.io/components/menus/specs),
[guidelines](https://m3.material.io/components/menus/guidelines), and
[accessibility](https://m3.material.io/components/menus/accessibility), including
the expressive measurement diagram. Inspected upstream
[dropdown-menu.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/dropdown-menu.tsx),
[context-menu.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/context-menu.tsx),
and [WAI keyboard pattern](https://www.w3.org/WAI/ARIA/apg/patterns/menubar/).
Revision: `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`; retrieval 2026-10-10.

## Contract and reference choices

Expressive panels use surface-container-low/on-surface. Selected items use
tertiary-container/on-tertiary-container; vibrant panels use tertiary-container,
with tertiary/on-tertiary selection. Rows are at least 48px tall (official target,
instead of upstream 44px), icons 20px, internal horizontal padding 12px and gap
8px. Panel padding is 2px/4px and corners 16px. Normal rows have 4px corners;
selected/expanded rows use 12px. Baseline retains 48px rows, 24px icons, 12px gaps,
8px vertical panel padding and 4px panel corners.

The web gallery uses dividers: gaps are recommended for Android, not web. The
optional `MenuEntry::Gap` is an explicit extension. Manual native popovers escape
ancestor clipping, retain theme inheritance and support nesting inside modal dialogs.
Anchors have 4px separation; panels flip and clamp to an 8px viewport margin.
Submenus use the whole parent edge; compact layouts reduce submenu width when
at least 112px remains, avoiding parent overlap in the measured 360px viewport.

Open motion uses upstream Expressive DefaultSpatial scaleY .6→1 over 440ms and
FastEffects opacity over 150ms. Close uses 120ms accelerated opacity/scaleY .9.
Generation guards prevent stale close completions from hiding a newer panel.

## Runtime checks and fixes

Checked Down/Up, Home/End, typeahead, disabled-item focus without activation,
submenu forward/back/Escape, RTL direction, root Escape focus return, checkbox
selection staying open, radio selection closing, and modal-menu Tab dismissal.
Shift+F10 and real secondary pointer click open the context menu with first-item
focus. Labels and optional descriptions use explicit ARIA associations; decorative
icons do not contribute names. Disabled items use `aria-disabled` rather than
native disabling, preserving the official focus behavior.

Runtime sampling caught and corrected CSS-second parsing, a 4px cascading overlap,
and RTL arrow lowering by the CSS bundler. Final menu entry shows scaleY .6, .657,
.766, .862 with increasing opacity, then settles. A 360px viewport yielded a
192px parent and a 123px submenu adjacent at x=229, ending at the 352px margin.
Actual OS Reduce Motion omits animated transforms and retains functional navigation.

## Evidence, builds and reproduction

[Raw samples](composite-samples.json), [build/copy results](composite-verification.md),
[copy dependencies](copy-components.md). Run the preview, wait for successful build,
reload, visit `/#menus`. Open Document actions; navigate Export→PDF and return.
Toggle Show details without dismissing; select Compact and reopen to check state.
Focus/right-click the context region and verify Escape returns focus.

## Remaining scope

Android/touch long-press, native packaging, audible screen-reader output, forced
colors, long scroll-menu fixtures and arbitrary dynamic-content/unmount races
remain unverified. Very narrow viewports or several cascading levels with less
than 112px side room may still require a consuming-app compact navigation pattern.
Bottom-sheet adaptation, badges/custom slots, autocomplete, select and combobox
are separate remaining components/extensions.
