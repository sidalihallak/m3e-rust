# Toast verification — 2026-10-11

`Toast` in `src/components/feedback.rs` forwards to the same single-message
Material Snackbar; it is naming compatibility, not a separate stacked-toast
implementation. [Upstream sonner](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/sonner.tsx) and
[upstream Base UI toast](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/toast.tsx) were inspected at `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.
Read [snackbar verification](snackbar-verification.md) for official references,
values, native inputs, timing, focus, build results and gaps. The Toast example
compiled independently, and its copy control matched 476 characters. The gallery
uses one Snackbar instance for both persistent and timed states; it does not
mount concurrent hosts. Imperative queue/promise APIs remain unported.
