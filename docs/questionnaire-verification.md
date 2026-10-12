# Questionnaire verification —2026-10-12

## Scope and reference

Read rendered official [component catalogue](https://m3.material.io/components),
[accessibility principles](https://m3.material.io/foundations/overview/principles)
and [assistive technology](https://m3.material.io/foundations/overview/assistive-technology)
on2026-10-12. None of these application blocks has a dedicated official Material
component page/spec. Geometry comes from upstream; Material guidance informs
semantic roles, keyboard access, named controls, targets and readable text.
Existing button/icon-button/TextField reports cover reused primitive contracts.

Actual [upstream source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/questionnaire.tsx), revision
c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972, retrieved2026-10-12.

## Reference contract and implementation

Sources: questionnaire.rs, questionnaire.js and application-blocks.css.
Inspected styled source and actual @shadcn/react0.3.1 registry JS/declarations:
https://unpkg.com/@shadcn/react@0.3.1/dist/questionnaire/index.js
Local data API: stable unique-ID definitions, single/multiple/text, disabled
questions/choices/root, required flag, descriptions, optional letters/numbers,
typed Vec<QuestionAnswer> callback. Retains answers when navigating back. Source
form/fieldset/legend/native radio/checkbox semantics; progress role with numbered
position and polite updates. Inactive steps hidden and inert; disabled questions
excluded from progress. Missing required answer reports alert/focuses first input;
optional empty question requires an answer or explicit Skip. Final submit validates
all steps and returns to first invalid. Skip clears current optional answer.
No network transmission occurs; app owns submission and persistence.

Root gap24px, item gap20px, choices gap12px; choice corners16px, padding14×16px,
16px indicator, native invisible full-card input. Checked state uses Primary10%
fill and40% border. Disabled opacity .38. Secondary3px focus ring. Local48px
choice minimum instead of source44; actions44px,48px on coarse pointer. Source
color transition150ms. Local Material TextField56px replaces the primitive's
unlabeled bare input. Next/previous/skip/submit are existing Material Buttons.
Enter on answer advances, Meta/Ctrl+Enter advances, native radio arrows/Space
select, optional shortcuts preserve disabled-choice indexing and ignore text entry,
composition/repeats/modifiers. Inert uses attribute presence/absence, not false string.

Immutable definitions are the current contract: remount to replace them. Native
input type/constraint APIs, controlled item/answers, reset/default answers,
custom slot rendering and advanced validators are extensions. Disabled text
questions are omitted from progress; hidden retained answers remain in Rust state.

## Runtime/build evidence

Desktop raw measurements and tested actions are saved in
[application-blocks-samples.json](application-blocks-samples.json). Screenshots
are in docs/evidence/application-blocks. Results below describe sampled states,
not every permutation. The live Wasm build and fresh independent export are
checked before completion; five copyable usages are included in the gallery.

## Platform scope and gaps

Android remains paused. Desktop/390px geometry does not establish mobile touch,
on-screen keyboard, native Android package behavior or audible screen-reader
announcements. Reduced-motion permutations beyond the sampled states, forced colors, text zoom, full RTL and
all dynamic/unmount/interruption permutations require wider evidence. No claim
of full upstream engine/API parity follows from the sampled core behavior.

## Reproduce

Run ./scripts/dev.sh, wait for successful build, reload the appropriate catalogue
route. Exercise pointer and native keyboard actions, inspect measured state and
focus, and use390px width separately for bounds. Copy the usage and compile a
fresh exported library. Compare the source contract before modifying behavior.

Token-resolution correction: source `m3e.css` overrides Tailwind radii.
`rounded-xl/2xl/3xl/lg` resolve to20/28/32/16px, not stock Tailwind
12/16/24/8px. The final CSS uses the inspected Material token values. Source:
https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/styles/m3e.css

The local action height token tracks the44px minimum (48px coarse), keeping
round resting corners at half actual height while preserving existing pressed
corners, press duration and ripple behavior. This avoids stale40px height tokens
when a composed action is enlarged.

## Final build and evidence

Live Wasm serve: initial42.08s success; final Rust rebuild32.64s success, then
reload before the final inspection. One .app-shell measured. Fresh editable
export /private/tmp/m3e-application-export-1012 compiles its library and all103
examples in12.81s. The only warning is a pre-existing unused ChipSize import.
Five clipboard comparisons are exact: attachment615, bubble512, message417,
scroller527 and questionnaire673 characters. Rust/UI copy instructions and all
required JS/CSS/dependencies are in docs/copy-components.md.

Pilot typography remains Inter; upstream uses Roboto Flex. Full font/pixel
parity is not claimed. Android and audible screen readers remain open. Actual
OS Reduce Motion was sampled for shimmer/jump and restored; other reduced-motion
permutations and forced-colors/zoom/full RTL checks remain.

## Measured desktop and compact results

Next without a required answer exposes an alert and focuses web radio. Actual
DOM active inert attribute is absent. Pointer checks web; Next Enter advances
to question2 of3, skipping disabled question. Space checks offline/collaboration.
Previous retains web. Native ArrowDown selects mobile and ignores disabled
choice; C shortcut does not activate that choice. Enter advances; Skip clears
optional choices and marks skippedtrue. Missing required text focuses textbox.
Typing remains focused (letter shortcut does not intercept text). Enter submits
local typed result: mobile, skipped features, retained description. Disabled form
sets native input/fieldset disabled; Tab from disable control goes to Copy code.
Re-enable preserves state. Callback fires once in the recorded submit check.

The first failed pointer test was a real inert bug: inert="false" still disabled
the active subtree. Final code uses Some("")/None and pointer/keyboard activation
was rerun successfully. Protocol documents the pitfall. Final choices16px corner,
74/50/50px high; action44px/rest corner22px. At390px form277px/input245px plus
16px side insets, body375px; no overflow. Native M3 field spans full form width.
Screenshots: questionnaire-desktop.png/questionnaire-390.png. Actual numeric
shortcuts, Ctrl/Meta+Enter, submit-all validation after later mutation, empty
question list, reset/custom validators and dynamic replacement remain gaps.
