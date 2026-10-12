# Copy components into a Dioxus project

Desktop copy/paste checked on 2026-10-10: nine composite/card/divider controls
showed success, and native paste exactly matched the 509-character Divider usage.
The updated export includes 16px card slot padding and the dialog focus/label
paint gutter. API/source completeness is checked separately by compiling all
40 exported examples; runtime styling in a separate consuming app remains open.
See [current browser evidence](resumed-browser-verification.json).

The Help and selection family adds eight independently checked copy controls:
all returned the exact snippet. A native paste into a single-line field matched
the Autocomplete snippet after native newline removal. [Family evidence](help-selection-verification.md)
records all 45 independently compiled examples and the current desktop checks.

The gallery's **Copy Rust code** buttons copy real usage examples. Component
implementations remain editable Rust and CSS files in this repository.

## Export the complete source kit

Run from this repository, choosing a new destination:

```sh
python3 scripts/export-kit.py /path/to/my-project/ui-kit
```

The export is an ordinary local Rust library, with all component sources,
semantic theme generation, icon data, loading shape data, motion helpers and
component CSS. It excludes the gallery. Add it to your application's Cargo.toml:

```toml
[dependencies]
m3e-rust-ui = { path = "ui-kit" }
```

Copy the exported CSS into your application's assets directory. Include the
stylesheets for the components you use with Dioxus `document::Stylesheet` and
`asset!`. Interactive components below also require **ripple.css**. For example:

```rust
rsx! {
    document::Stylesheet { href: asset!("/assets/icon.css") }
    document::Stylesheet { href: asset!("/assets/ripple.css") }
    document::Stylesheet { href: asset!("/assets/icon-button.css") }
    IconButton { icon: icons::FAVORITE, aria_label: "Save" }
}
```

Components consume `--md-sys-color-*` properties. Generate them through
`m3e_rust_ui::theme::{ThemePreview, css_variables}` and apply the resulting inline
style on an ancestor, as the gallery does. The exported theme module uses
`material-colors`; the components themselves do not depend on its API.

## Dependencies for a smaller source copy

Keep the module layout shown in `src/components/mod.rs` and exports in
`src/lib.rs`; component imports use `super` and generated data uses `crate`.
If copying only selected files, retain the corresponding module declarations.

| Component Rust file | CSS | Additional Rust source |
| --- | --- | --- |
| `tooltip.rs` (plain/rich) | `help.css`, `composite-motion.css` | `anchored.rs` + **`anchored.js`**, `motion.rs`; supply chosen action components for rich help |
| `popover.rs` (Popover/HelpTrigger) | `help.css`, `composite-motion.css`; HelpTrigger also needs `action-control.css`, `ripple.css`, `icon.css` | `anchored.rs` + **`anchored.js`**, `motion.rs`; HelpTrigger needs `action_control.rs`, `button.rs` enums, `icon.rs`, `icons.rs`, `ripple.rs` |
| `hover_card.rs` | `help.css`, `composite-motion.css` | `anchored.rs` + **`anchored.js`**, `motion.rs` |
| `select.rs` (Select/NativeSelect), `combobox.rs` (Combobox/Autocomplete) | `select.css`, `help.css`, `composite-motion.css`, `ripple.css`, `icon.css` | `anchored.rs` + **`anchored.js`**, **`selection.js`**, `motion.rs`, `ripple.rs`, `icon.rs`, `icons.rs`, `text_field.rs` (variant enum and Rust dependencies); Combobox also needs `select.rs` |
| `standard_button_group.rs` | `standard-button-group.css`, `action-control.css`, `composite-motion.css`, `ripple.css`, `icon.css` | `action_control.rs`, `button.rs` enums, `icon.rs`, `icons.rs`, `ripple.rs`, `motion.rs` |
| `split_button.rs` | `split-button.css` and all menu/shared action CSS below | `menu.rs` + `menu.js`, `action_control.rs`, `button.rs` enums, `icon.rs`, `icons.rs`, `ripple.rs`, `motion.rs` |
| `menu.rs` | `menu.css`, `action-control.css`, `composite-motion.css`, `ripple.css`, `icon.css` | **`menu.js` in the same directory**, `action_control.rs`, `button.rs` enums, `icon.rs`, `icons.rs`, `ripple.rs`, `motion.rs` |
| `dialog.rs` | `dialog.css`, `action-control.css`, `composite-motion.css`, `ripple.css`, `icon.css` | **`dialog.js` in the same directory**, `compact.rs`, `action_control.rs`, `button.rs` enums, `icon.rs`, `icons.rs`, `ripple.rs`, `motion.rs` |
| `button_group.rs` | `button-group.css`, `ripple.css`, `icon.css` | `button.rs` enums, `icon.rs`, `icons.rs`, `ripple.rs`, `motion.rs` |
| `button.rs` | `button.css` | None; leading icons may need Icon |
| `icon.rs` | `icon.css` | `icons.rs` for supplied generated glyphs |
| `icon_button.rs`, `fab.rs` | matching CSS, `icon.css`, `ripple.css` | `icon.rs`, `icons.rs`, `ripple.rs`, `motion.rs` |
| `fab_menu.rs`, `chip.rs`, `segmented_button.rs`, `tabs.rs` | matching CSS, `icon.css`, `ripple.css` | `icon.rs`, `icons.rs`, `ripple.rs`, `motion.rs` |
| `checkbox.rs`, `switch.rs`, `radio.rs`, `card.rs` | matching CSS, `ripple.css` | `ripple.rs`, `motion.rs`; card actions may use Button |
| `progress.rs` | `progress.css` | `motion.rs` |
| `loading_indicator.rs` | `loading-indicator.css` | `motion.rs`, `loading_shapes.rs` |
| `text_field.rs` | `text-field.css`, `icon.css` | `motion.rs`, `icon.rs`, `icons.rs` |
| `divider.rs`, `badge.rs` | matching CSS | Badge anchor icons may use Icon |

Keep the existing notices on generated Material Symbols and shape data.

## Verification scope

The exported library and all 45 gallery usage examples were compiled for
`wasm32-unknown-unknown` in a separate directory on 2026-10-10. This establishes
that the source bundle includes its Rust dependencies and that the examples
use the current API. Runtime styling and interaction checks were performed in
the original live gallery. A consumer still needs to include its chosen CSS,
provide semantic color roles, and supply its own media assets and application
state. See [the fidelity fix report](component-fidelity-fixes.md).

`TextField` fills its parent width. Constrain the containing layout for narrower
fields; dialog content automatically receives the full available field width.

Menus and dialogs include their JavaScript runtime through Rust `include_str!`;
retain those files when making a smaller copy. The complete exporter includes
them automatically. The shared action styles use finite shape radii, semantic
roles and native button semantics. Their motion stylesheet supplies the actual
spring curves used by the popup runtime. See [composite evidence](composite-verification.md).

`StandardButtonGroup`, `SplitButton`, `DropdownMenu`, `ContextMenu`, `Dialog` and
`AlertDialog` expose controlled state: update it in `onchange`/`onopenchange`.
For a bare `Menu`, supply a stable existing `anchor_id`. Menu selection reports a
path through `MenuEntry` indices; checkbox selection stays open. Your app owns
radio exclusivity, validation and side effects. Use `DialogVariant::Adaptive` for
compact full-screen/desktop basic presentation. Intercept its `onopenchange`
before discarding an unsaved draft. Include additional component CSS (for example
TextField) when using those controls in dialog content.

For text-only tab panels, include `tabindex: "0"` and a visible focus style.
Give range sliders an `aria_label`. Badge `aria_label` supplies full announcement
context; copy `badge.css` too so its announcement text is visually hidden.

## Help and selection state and semantics

`Popover`, `RichTooltip`, `Select`, `Combobox` and `Autocomplete` are controlled:
update `open` through `onopenchange`. Select owns a String option ID; Combobox
owns a Vec of IDs plus a separate `input_value` draft. Autocomplete owns arbitrary
text; choosing a suggestion writes its label and reports its ID through
`onselect`. It exposes `name` for its native form input. Custom Select/Combobox
`required` is an ARIA state: validate committed choices in the consuming form.
NativeSelect and free-text Autocomplete expose native required validation.

Combobox `show_clear` applies to single-value selection. Multiple selection
keeps the dropdown affordance and uses each input chip's remove action.

Keep anchors mounted with stable, unique IDs. `HelpTrigger` supplies a named
native action with such an ID; a native link/button can also be used as the
anchor. Tooltip is supplementary text; RichTooltip provides persistent help
opened explicitly. HoverCard is deliberately hidden from assistive technology:
its preview contents are inert and must also be available at the linked
destination. Use Popover for interactive content.

Include TextField/Checkbox/action CSS when using the Popover form example.
Native Popover API and modern CSS support are required; this pilot does not ship
a legacy-browser polyfill. Preserve `.js` runtime files with the Rust sources;
the exporter includes them automatically. [Each component report](help-selection-verification.md)
records tested states and intentional differences from upstream.

## Navigation dependencies and state

| Rust sources | Required CSS/runtime dependencies |
| --- | --- |
| `app_bar.rs` | `navigation.css`, `composite-motion.css`; include chosen action/icon CSS for slots |
| `navigation.rs` | `navigation.css`, `composite-motion.css`, `ripple.css`, `icon.css`, `badge.css`, `icon-button.css`; `navigation.js`, `modal_navigation.js`, `motion.rs`, `ripple.rs`, `badge.rs`, `icon.rs`, `icon_button.rs`, `icons.rs` and their enum dependencies |
| `toolbar.rs` | `navigation.css`, `composite-motion.css`; `navigation.rs`/`navigation.js` keyboard helper and its dependencies; include CSS for supplied child controls |

Navigation selected state is controlled: update it in `onchange`. Modal rail
`open` is controlled through `onopenchange`; destination selection also requests
closure. AppBar `scrolled` is supplied by the consuming application's scroll
observer. Expanded rail supports 220–360px widths and the permitted full-width pill.
Use named native controls inside Toolbar and named leading/trailing actions in
AppBar. Preserve both outlined and filled destination icons.

`showcase.rs`, `showcase.js` and `showcase.css` belong to the demo application,
not the reusable kit. All five navigation clipboard examples matched exactly;
all 45 independent examples compiled. See [navigation verification](navigation-verification.md)
for runtime scope and unresolved verification gaps.

## Surfaces dependencies and state

`Sheet`, `BottomSheet` and `SideSheet` share `sheet.rs` and **`sheet.js`** in the
same directory. Include `sheet.css`, `composite-motion.css`, `icon-button.css`,
`icon.css` and `ripple.css`; retain `motion.rs`, `icon_button.rs`, `button.rs` enum
dependencies, `icon.rs`, `icons.rs` and `ripple.rs`. Include additional TextField,
Checkbox or Button CSS when supplying those controls. The complete exporter
includes component sources and runtime assets automatically. Keep global
`box-sizing: border-box` in the consuming app, as the gallery does.

All sheets have controlled `open` and `onopenchange`. BottomSheet also has
controlled `expanded` and `onexpandedchange`; update both callbacks for working
handle expansion. Dragging is confined to the handle; native body scrolling is
independent. `SheetVariant::Standard` avoids native modality. Put standard side
sheets beside main content in a flex row and standard bottom sheets inside a
positioned pane with a defined height. `SideSheet` defaults to Standard and End;
`BottomSheet` defaults to Modal. Logical Start/End follows direction. App state
owns unsaved data and responsive bottom-to-side adaptation. Do not use top sheets
as a claimed official Material variant.

Five new copy controls matched exactly. [Surfaces verification](surfaces-verification.md)
records all 45 independently compiled examples, actual desktop/compact interactions,
reduced motion and the paused Android scope.

## Feedback and content dependencies and state

| Sources | Required styles and runtime files |
| --- | --- |
| `feedback.rs` (Snackbar, Toast, Alert) | `feedback.css`, `composite-motion.css`, `ripple.css`, `icon.css`; `snackbar.js`, `motion.rs`, `ripple.rs`, `icon.rs`, `icons.rs` |
| `search.rs` | `search.css`, `feedback.css`, `composite-motion.css`, `ripple.css`, `icon.css`; `search.js` and shared motion/ripple/icon sources |
| `list.rs` | `list.css`, `composite-motion.css`, `ripple.css`, `icon.css`; `list.js` and shared motion/ripple/icon sources |
| `content.rs` | `content.css`; `icon.css` and icon sources for EmptyState icons; include CSS for supplied actions/media |

Use global `box-sizing: border-box`. All interactive slots must use named native
controls. ListItem leading/trailing slots are decorative; independent controls
belong in `secondary`, as siblings of the primary button. Selection lists must
pair `ListSelection::Single`/`Multiple` with `selected: Some(bool)` on every item.
The app owns exclusivity and selected IDs. Selection rows do not support secondary
actions. Provide a group name explaining single or multiple selection.

SearchView requires a mounted, unique SearchBar `anchor_id`. `open` and `value`
are controlled; update `onopenchange` and `oninput`. Supply filtered results,
commit `onselect`, and implement `onsubmit` in the consuming app. Adaptive uses
full-screen search below 600px. Query composition is respected; enabled results
are previewed by arrows and committed with Enter. Native dialog provides modality.

Mount one Snackbar/Toast host per app. Reuse it for updated information; `revision`
restarts its timer. Queue unrelated messages in app state and show the next after
exit. This controlled API does not implement upstream's imperative toast store,
promise API or a queue manager. An action always makes the snackbar persistent.
Timed messages are opt-in, clamped to 4–10 seconds, and require equivalent inline
feedback on the web. Pause on focus/hover/hidden document is included. Alt+G moves
focus to the snackbar; Escape dismisses while focused. Set `--m3-snackbar-bottom`
to raise it above navigation/FABs. Never put critical errors only in a snackbar.

Avatar label identifies the entity; `decorative: true` hides duplicates beside
an existing name. Images have empty alt inside the named avatar, with initials
fallback on loading/error. Skeleton is decorative. LoadingPlaceholder applies
`aria-busy` to the content container and supplies a loading status. Your app owns
loading completion. Alert defaults to polite status; choose Alert for urgent
updates and Note for static information. These helpers and EmptyState are upstream
extensions, not standalone official M3 components.

Latest 2026-10-11 check: all **53** independent library usage examples compile.
All eight feedback/content copy controls matched exactly. Previous dated counts
above describe their respective audits. See [feedback/content evidence](feedback-content-verification.md).

## Scheduling: date picker and calendar

The four Scheduling examples bring the independent export to **57 compiled
usage examples**. See [date-picker evidence](date-picker-verification.md) and
[calendar evidence](calendar-verification.md).

Copy `date.rs`, `calendar.rs`, **`calendar.js`**, `date_picker.rs`,
**`date_input.js`** and `date-picker.css`. Keep JS beside its Rust source.
Also include Icon/icons, Ripple/motion, Tooltip/anchored runtime, TextField,
Button, Dialog and compact layout helper with their CSS/runtime dependencies.
Styles required: `date-picker.css`, `text-field.css`, `button.css`,
`dialog.css`, `action-control.css`, `help.css`, `composite-motion.css`,
`icon.css`, `ripple.css`. Include shared semantic color and motion variables.

`CivilDate::new(year, month, day)` validates Gregorian dates without timezones.
`DatePicker` owns editing text but its date value is controlled: update
`onchange`. `DatePickerModal` owns a draft while open; update `onopenchange`
and save `onconfirm`. `DateSelection::Range(DateRange::default())` starts a
range. Calendar month and selection are controlled separately. Constraints
default to years 1900–2100; override them for other domains. Range-interior
disabled-day exclusion is opt-in. Labels/month/weekdays/date format can be
configured; full locale sentence ordering is not yet an internationalization API.

## Time picker

Scheduling adds `TimePicker` with dial/manual entry, 12/24-hour formats and
controlled native modal draft state. The complete source export now has
**66 independently compiled usage examples**.

For a smaller copy, include `time.rs`, `time_picker.rs`, **`time_picker.js`**
beside the Rust file, `time-picker.css`, and the shared Dialog/JS, Button,
ActionControl, Icon/icons, Ripple and motion sources. Include `dialog.css`,
`button.css`, `action-control.css`, `composite-motion.css`, `ripple.css`
and `icon.css`, plus semantic color variables.

`TimeValue::new(hours, minutes)` validates 24-hour values. The picker uses
`open/onopenchange` and commits only through `onconfirm`; its draft is internal.
Supply `hour24` from application/system preference. `default_mode` chooses
Dial or Input when opening. Adaptive layout changes presentation to fit the
viewport; it may require input mode even when Dial was requested.
See [reference and runtime evidence](time-picker-verification.md).


## Disclosure and data compositions

New copyable components:

| Component | Required files and dependencies |
| --- | --- |
| Accordion | `accordion.rs`, `disclosure.js`, `accordion.css`; motion IDs, Icon/icons, shared Ripple |
| Collapsible | `collapsible.rs`, Accordion runtime helper, `disclosure.js`, `accordion.css` |
| Table primitives | `table.rs`, `table.css`; native HTML semantics |
| Pagination | `pagination.rs`, `pagination.css`; Icon/icons, Ripple, NativeSelect and its Select/field CSS dependencies |
| DataTable | `data_table.rs`, `data-table.css`, Table/Pagination plus TextField, NativeSelect, Checkbox and their CSS/runtime dependencies |

Use the full source exporter to retain dependencies. Accordion is controlled by
unique string values, Collapsible by `open`. DataTable selection is controlled by
stable row IDs; its query/sort/page/visibility state is internal. Columns require
unique keys, rows unique IDs, and cells follow column order. A formatted numeric
cell needs `DataCell::formatted_number(value, display)` to sort numerically.
Custom cells may set `DataCell.content` while retaining searchable text.

Pagination callbacks are for local page state, with one-based page numbers.
Use real anchors for URL navigation. DataTable is client-side; server queries,
virtualization, column resizing and editing are separate extensions.

Seven new gallery snippets have exact clipboard evidence and compile with the
independent export; all 66 current usage snippets compile. See the
[accordion](accordion-verification.md), [collapsible](collapsible-verification.md),
[table](table-verification.md), [pagination](pagination-verification.md) and
[data-table](data-table-verification.md) verification reports.


## Charts and carousels

| Component | Required copy files |
| --- | --- |
| Charts | `chart.rs`, `chart.js`, `chart.css`, motion ID helper; SVG rendering has no external chart dependency |
| Carousels | `carousel.rs`, `carousel.js`, `carousel.css`, shared Icon/icons, Ripple and motion CSS/runtime |

ChartSeries keys and CarouselEntry keys must be unique/stable. ChartPoint values
follow series order. Missing/nonfinite values stay missing; formatted chart
numbers retain original raw values in the data table. Donut uses one datum with
non-negative values. Overlaid area supports at most two visible series.

CarouselEntry content must be presentational: do not nest buttons/links in the
owned item button. Handle `onactivate` in app state. Expressive `onshowall`
should navigate/open a vertically scrolling view of every item. FullScreen
omits that control and uses a vertical snap region whose height the app supplies.

The refreshed independent library and all **75** snippets compile. The nine new
visual-family copy controls match the displayed code exactly. Desktop and
responsive evidence is recorded; Android remains paused. See
[charts](chart-verification.md), [carousel](carousel-verification.md) and
[expressive carousel](expressive-carousel-verification.md) for current scope.

## Commands and breadcrumbs

Copy `command.rs`, `command.js`, `command.css` with the shared Dialog, action
controls, Icon/icons and motion ID/runtime dependencies for CommandDialog.
Copy `breadcrumb.rs`, `breadcrumb.css` and Icon/icons for breadcrumbs. The
exporter includes these files. Four new examples bring the independent export
to **79** compiling usages. Command query and actions belong to the caller;
entry values must be unique. Shortcut text does not register a hotkey.
Breadcrumb parent links are native URLs; its last entry is current-page text.
See [command](command-verification.md) and [breadcrumb](breadcrumb-verification.md)
reports for source comparisons and runtime scope.

## Navigation extensions

Copy `menubar.rs`, `menubar.js`, `navigation_menu.rs`, `sidebar.rs`, and
`navigation-extensions.css` with shared Menu, Popover/anchored, compact breakpoint,
NavigationRail/ModalNavigationRail, Icon and motion/ripple dependencies.
The exporter carries these files. MenubarSelection identifies the parent menu
index and existing MenuSelection path. NavigationMenu uses native href links;
Sidebar requires independent controlled desktop/mobile state. Cookie persistence,
global hotkeys and the upstream React provider are not ported.
See the component reports for actual runtime scope and differences.

Latest navigation-family check: all **82** exported usages compile; the three
new controls have exact clipboard evidence. Shared source dependencies and
platform gaps are recorded in the component reports.

## Input groups and OTP codes

Copy `input_group.rs` and/or `input_otp.rs` with `input_otp.js`, `motion.rs`
and `input-extensions.css`. InputGroupButton additionally needs `button.rs`
and `button.css`; add `icon.rs`/`icon.css` for icons. The complete exporter
includes these automatically. Provide Material color variables.

Use one InputGroupInput or InputGroupTextarea inside each InputGroup: its
label/support IDs are supplied through Dioxus context. The enclosing disabled
fieldset disables nested native inputs and actions. Group error propagates
to the input; error supporting text belongs on the group.

InputOTP owns one native control and exposes decorative slots through length,
separator_after and pattern props. Update value in oninput; oncomplete reports
an incomplete-to-complete edit. The app owns code validation and submission.
Numeric codes use text inputs, so no HTML numeric spinner appears. Form name,
required, readonly, disabled and one-time-code autocomplete are native attributes.
Android/SMS autofill remain unverified.

All **86** exported usages compile independently; the four new clipboard controls
match exactly. See the component reports for desktop and390px evidence and gaps.

## Application layout helpers

Copy `aspect_ratio.rs`, `scroll_area.rs` + `scroll_area.js`, and/or `resizable.rs`
+ `resizable.js` with `motion.rs` and `layout-helpers.css`. The complete exporter
retains them automatically. Add semantic color variables. Gallery surfaces and
media assets are examples; consuming apps own their content and bounds.

AspectRatio takes a width/height ratio. ScrollArea takes a required accessible
label, CSS height and Vertical/Horizontal/Both scrollbar mode. Its native
viewport is the single keyboard scroll target; its bars are decorative to AT.

ResizablePanelGroup contains direct alternating ResizablePanel/ResizableHandle
children. Each panel needs a label; each handle should name its preceding pane.
All size props are percentages. Defaults initialize once; runtime owns sizes and
onlayoutchange reports initial/committed values. Include a bounded parent height.
Set adaptive=true to stack below840px; otherwise direction stays as supplied.
Collapse uses native inert; use Enter to restore the panel. Remount when replacing
default layout or child structure. Full upstream unit/persistence APIs are not ported.

All91 examples compile; five new clipboard controls match exact text after completion.
Platform scope and unported behavior are documented in the component reports.

## Keyboard hints, shapes and markers

Copy `kbd.rs`, `marker.rs` and/or `shape.rs` with `expression-helpers.css`.
Shape additionally requires `shape_library.rs` at the crate root, its public
module declaration, and `composite-motion.css`. MarkerIcon may contain Icon: retain
icon.rs/icon.css and your chosen icon data. Keep THIRD_PARTY_NOTICES.md with copied
shape data. The exporter now includes both shape_library.rs and notices.

Kbd/KbdGroup display key hints; supply key_label for readable glyph names and
implement the shortcuts in your app. They add no focus stops. Marker href uses
a native anchor; ordinary markers/lines/icons add no interactive roles.

ShapeName::ALL contains35 names. Shape name is optional; initialize an explicit
name before morphing between named polygons. Shape size defaults48px. Supplied
media children fill the mask; your app owns alt text and cropping. Optional label
names a composite image. Customize fill with --m3-shape-color in a class.

All95 usages compile and four new code copies match exactly. Shape coordinates
match upstream for all35 catalogue items; pointer/keyboard/interruption and actual
OS Reduce Motion have separate evidence in the component reports.
