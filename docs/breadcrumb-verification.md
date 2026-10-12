# Breadcrumb verification — 2026-10-12

## Scope and references

`breadcrumb.rs`, `breadcrumb.css`, `wayfinding_gallery.rs`.
Read the actual pinned [upstream breadcrumb source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/breadcrumb.tsx),
the rendered [Material catalog](https://m3.material.io/components), and
[WAI breadcrumb pattern](https://www.w3.org/WAI/ARIA/apg/patterns/breadcrumb/).
Material has no standalone breadcrumb overview/spec/guideline/accessibility
pages. This is an upstream navigation composition using Material roles.

## Contract

Named native nav/ol/li hierarchy; native ancestor links, text current page with
aria-current=page. Unlike upstream's disabled role=link span, the current page
is correctly non-interactive. No custom keyboard model: Tab/Enter and link
context menus/new-tab behavior are native. Separators are decorative, excluded
from announcements and mirrored in RTL. Ellipsis is explicitly decorative;
consumers must supply a named real control and accessible popup for hidden paths.

Upstream body-medium14px/20px, on-surface-variant ancestors/on-surface current;
6px compact/10px expanded gaps,14px chevrons and20px ellipsis. Added minimum48px
link heights and4px link padding for touch access, instead of source text-only
hit areas. Focus ring uses primary; hover underline/color has no new motion.
Long trails wrap rather than silently hide/truncate ancestors.

Primitive components are copyable; BreadcrumbTrail is an additional Rust data
convenience API. Its final entry is always the current page. Empty trail renders
an empty named navigation; missing parent hrefs render plain text.

## Verification

Independent export and all79 examples compile (8.01s). Browser, compact layout
and clipboard results are recorded below after the live preview rebuild. Android paused.

## Gaps

Automatic collapse/overflow menus, custom router render adapters and automatic
current-page inference are not implemented. RTL, forced colors, text enlargement,
audible screen readers and Android remain unverified unless recorded below.

## Desktop and responsive evidence

Live preview built successfully (35.66s /25.54s), then reloaded. Native Tab moves
Home→Projects; Enter on Projects navigates to `/#cards`. The current page is a
span with aria-current=page, with no interactive role. Both separators have
aria-hidden=true. Native links are48px tall and text14px.

At390px, long trail width/scroll width are both295px; body/client375px. Current
long title wraps to40px height and parent links retain48px targets. No page
horizontal overflow. Viewport override restored. Both breadcrumb copy controls
match exact clipboard text (338/450 characters). All79 independently exported
Rust usages compile. Browser checks establish desktop/responsive behavior only.

[Raw samples](breadcrumb-samples.json), [desktop proof](evidence/wayfinding/breadcrumb-desktop.png),
[390px proof](evidence/wayfinding/breadcrumb-390.png). Use `/#breadcrumbs`, Tab/Enter
through ancestor links, inspect current-page semantics, and resize to390px.
Remaining gaps are listed above; no Android/RTL or audible screen-reader claim.
