# Android component preview

## Environment — 2026-10-09

- Device: isolated Pixel 8 AVD, `m3e-pixel8` / `emulator-5554`.
- OS: Android 15 / API 35, ARM64 Google Play system image.
- Image: existing installation at
  `/Users/sidalihallak/Library/Android/sdk/system-images/android-35/google_apis_playstore/arm64-v8a`.
- Emulator: existing Homebrew Android Emulator 36.6.11.
- AVD data: `/private/tmp/m3e-android-avds/m3e-pixel8.avd`.
- SDK facade: `/private/tmp/m3e-android-sdk`; it reuses the installed command-line
  tools and system images. No system image was downloaded for this preview.
- Temporary macOS wrapper: `/private/tmp/M3E Android Emulator.app`. It contains
  the installed QEMU executable and links to its libraries, with
  `NSHighResolutionCapable=true`, so the computer-control tool can address the
  standalone emulator window. The wrapper does not change the installed SDK.

This preview exercises the **web/Wasm component in Android Chrome**. It is not
a native Dioxus Android application build.

## Connect to the development server

Keep `./scripts/dev.sh` running. After each emulator restart, connect the local
development port:

```sh
/opt/homebrew/share/android-commandlinetools/platform-tools/adb \
  -s emulator-5554 reverse tcp:8080 tcp:8080
```

Then open `http://127.0.0.1:8080/` in Android Chrome. Port reversal keeps the
preview and live-reload WebSocket on the same localhost address.

## Restart the existing virtual device

Run the following in a terminal and keep it running:

```sh
env ANDROID_HOME=/private/tmp/m3e-android-sdk \
    ANDROID_SDK_ROOT=/private/tmp/m3e-android-sdk \
    ANDROID_AVD_HOME=/private/tmp/m3e-android-avds \
    ANDROID_USER_HOME=/private/tmp/m3e-android-user \
    /opt/homebrew/share/android-commandlinetools/emulator/emulator \
    -avd m3e-pixel8 -no-boot-anim -no-metrics -gpu host \
    -netdelay none -netspeed full
```

The `/private/tmp` setup is temporary. Check whether it still exists before
reusing these commands after a system restart. To automate the macOS window,
use the temporary app wrapper with the installed emulator's library path and
`ANDROID_EMULATOR_LAUNCHER_DIR` pointing to its installation.

## Verification status

The device booted successfully (`sys.boot_completed=1`), ADB lists it as a
device, and port reversal completed. Chrome is installed and opened.

The Android test was stopped at the user's request. Chrome's first launch
requires accepting its Terms of Service even when choosing “Use without an
account.” Setup was not completed; the local page and touch behavior have not
yet been verified on Android. Resume this work when the user requests it.
The [setup screenshot](android-chrome-setup.jpg) records this state.

Once the preview opens, follow the
[component development protocol](component-development.md), including quick
taps, repeated taps, selection, disabled controls and scrolling that starts on
a button. Update the [button report](button-motion-verification.md) with actual
Android results. The upstream 150ms touch ripple delay remains an implementation
gap until separately addressed and measured.
