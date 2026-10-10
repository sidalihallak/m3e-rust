# HelpTrigger wrapper verification — 2026-10-10

`HelpTrigger` in `popover.rs` adds a stable caller-provided ID to the existing
native ActionControl used by composite buttons. It does not introduce another
button design or animation system. See [the existing action/button reference and
checks](composite-verification.md), [button baseline](button-motion-verification.md),
and [current family evidence](help-selection-verification.md).

Its API exposes label, optional icon/icon-only form, emphasis style, disabled
state and onclick. The native `type=button` has its own accessible name. The
popup runtime adds dialog/listbox expanded-controls semantics only where needed;
Tooltip merges describedby. Small controls retain the shared 48px pseudo-element
hit target around their 40px visual control. CSS/source dependencies are listed
in [the copy guide](copy-components.md).

Actual pointer/Enter/Tab actions opened associated popovers and rich help;
Escape restored the labeled trigger, plain click suppression and focus tooltips
were rechecked. The disabled example appears as a native disabled button in AX.
All eight family examples and all 35 exported examples compile. No new or
exhaustive standalone button-motion check is claimed for this wrapper; broader
button state/platform limits remain in the referenced reports. Android is paused.
