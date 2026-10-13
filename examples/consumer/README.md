# Independent consuming-app verification

This is an ordinary Dioxus application, not a gallery module. Its application
CSS only sets page layout; `UiStyles` supplies registry-installed component CSS.

From the kit checkout:

```sh
python3 scripts/m3e.py add button icon-button material-symbol text-field combobox dialog tabs switch card sidebar-provider data-table message-scroller theme-scope --target examples/consumer
cd examples/consumer
dx serve --platform web --addr 127.0.0.1 --port 8081 --open false
```

The generated `ui-kit/` is ignored and remains editable. The app's Cargo.toml
uses that path dependency. The consumer is checked independently from the main
showcase. Run the installer again after updating the source registry; it preserves
unchanged-source local edits and reports conflicts before writing.
