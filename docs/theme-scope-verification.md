# Theme scope and color mode verification — 2026-10-12

## Scope and references

Sources: `src/theme.rs`, `src/components/theme_scope.rs`, `theme_scope.js`,
`assets/theme-helpers.css`, `src/theme_helpers_gallery.rs`.

Read rendered official [color overview](https://m3.material.io/styles/color/system/overview),
[how the system works](https://m3.material.io/styles/color/system/how-the-system-works)
and [color roles](https://m3.material.io/styles/color/roles) on 2026-10-12.
These are foundations, not a component with separate size/motion specs. Guidance
covers dynamic source colors, semantic pairings, automatic light/dark roles,
standard/medium/high contrast and fixed/surface roles. Pair on-primary with
primary and on-container with its respective container; arbitrary combinations
can lose accessible contrast. The current overview describes the Expressive
more-colorful light on-container update.

Actual upstream revision: c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972:
[theme scope](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/m3-theme-scope.tsx),
[color mode](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/color-mode-provider.tsx),
[theme provider](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/m3-theme-provider.tsx),
[combined provider](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/m3e-provider.tsx),
[color generation](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/lib/color.ts).

## Contract

ThemeScope controls an ARGB seed, ColorMode Light/Dark/System, seven typed
ThemeVariant choices and ThemeContrast Standard/Medium/High (0/0.5/1). The Rust
MCU port material-colors 0.5 generates Spec2021, Phone light/dark schemes. The
browser media listener resolves System and is removed on unmount. Every scope
writes complete roles and CSS color-scheme locally. Children inherit roles;
nested scopes and siblings override independently. Native dialog/popover DOM
inheritance applies; externally portalled content needs its own scope.

New role output includes error containers, fixed accents, inverse-primary,
surface dim/bright/lowest/tint and legacy background aliases. Four *-dim aliases
use the corresponding accent role, as upstream does for Spec2021. Standard light
on-container text follows tone30 for ordinary variants, matching upstream's
Expressive adjustment. **Content/Fidelity retain MCU's contrast-aware text tone**:
blindly assigning tone30 to their seed-toned containers can break contrast.
This is an intentional difference from the current upstream unconditional
standard-light tone30 override. The previous global pilot roles now also use
this Expressive light-container adjustment; no press/ripple tokens changed.

ThemeProvider initializes ThemeSettings once and supplies a Dioxus Signal through
use_theme(). Consumers mutate settings or persist them as needed. This preserves
independent roots rather than mutating a singleton HTML theme/style tag.
System follows OS, not the gallery's separate page appearance toggle. The
provider does not install upstream's default d shortcut, storage/cross-tab
synchronization, pre-paint storage script, shape scaling or motion-scheme switch.
These are optional extensions, not claims of literal React provider parity.

## Runtime evidence

`docs/theme-helpers-samples.json` records desktop states and computed roles.
Initial teal light primary #006a60/surface rgb(244,251,248); dark pointer activation
changes these to #82d5c8/rgb(14,21,19). The nested light violet remains #65558f
and the sibling dark rose remains #ffb3b3 throughout. Enter restores Light;
Space activates High contrast (#00332d teal primary). Rose high gives #51191d.
Provider Enter/Space independently changes its violet primary from #cfbdfe dark
to #65558f light. System resolves light against the actual false browser dark
media query. Medium and Standard also exercised. No OS appearance change was
made. The mode and contrast demo buttons use native toggle semantics.

Final build, clipboard, compact layout and contrast results are recorded below
and in the raw evidence. Actual dark OS change events, first-paint system flash,
RTL, forced colors, text zoom, audible readers, all variant/seed permutations and
Android are not established by the sampled desktop results. Android remains
paused. Theme selection has no new choreography; inherited button motion is the
existing accepted component, not reverified for all themed states here.

## Reproduce

Run ./scripts/dev.sh, wait for a successful Wasm build, reload /#theme-scopes.
Activate Light/Dark/System, contrast and seed controls using pointer/Enter/Space;
compare nested/sibling roles and the page theme. Toggle provider appearance.
Copy both snippets and compile the source export from its project directory.
Use 390px width separately for responsive geometry; this is not Android touch.

Theme swaps suspend descendant transitions for two animation frames, matching the
source pause strategy locally. A style MutationObserver owns the pause; repeated
swaps cancel prior frame callbacks, and unmount disconnects both observer/media
listener. This avoids animating role pairs through intermediate colors. Native
button press transitions resume after the swap. Final measured frames are in the
raw samples.

## Final results

Final live Wasm rebuild:19.29s, success; reloaded and measured one .app-shell.
Independent refreshed final library and all98 examples:8.72s, success after the
transition-pause addition.
The sole warning is the pre-existing unused ChipSize import in a prior example.

Actual swap sampling observed light button background rgb(204,232,226) changing
directly to dark rgb(51,75,71) at136ms relative to sampling start. At136/175ms,
transitionProperty was none and the pause class was present; by953ms the normal
button transition list returned. Scope width stayed401.30px in all35 samples.
These external DOM samples do not measure a display-refresh-exact two-frame
duration; they demonstrate that the source-style pause ran and resumed.

All six light/dark × Standard/Medium/High sampled schemes have paired primary,
three accent containers, error container and surface contrast saved in the raw
JSON. Nested/sibling roles remain independent. At390px, outer scopes are327px
wide and nested277px; body scrollWidth375, no horizontal overflow. Screenshots
are in docs/evidence/theme-helpers/scopes-desktop.png and scopes-390.png.

Clipboard comparisons succeeded exactly: scope513 characters, provider534.
Earlier stale clipboard reads were retried using native Enter activation before
recording success. The first attempted in-page rAF sampler was unavailable in
the read-only inspection context; successful samples use external DOM reads.
The browser session reset discarded earlier unsaved arrays; the JSON contains
fresh six-state measurements and explicitly labeled image-result transcriptions
from successful tool output, rather than claiming those are full lost arrays.

Sampled paired-role contrast: standard minimum6.50:1, medium minimum5.16:1,
high minimum8.97:1 across the six checked pairs. This is the teal seed matrix,
not an assertion for every seed/variant, state overlay or component.
