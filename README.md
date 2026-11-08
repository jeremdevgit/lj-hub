# LJ Hub

Lightweight, local-first Logitech gaming device profile manager for Windows, macOS and Linux. **Early alpha:** this is not yet a G HUB replacement. It is an independent open-source project and is not affiliated with Logitech.

## What works today

| Feature | Windows | macOS | Linux |
|---|---|---|---|
| Discover Logitech USB HID devices | Yes | Yes | Yes |
| Create, edit and save local profiles | Yes | Yes | Yes |
| Interface in 8 languages | Yes | Yes | Yes |
| Search 116 Logitech IDs from a pinned libratbag catalog | Yes | Yes | Yes |
| Set DPI and report interval with `ratbagd` | — | — | Supported mice only |

The RGB color and notes fields are stored in the profile but are **not sent to hardware**. Device-specific controls, macros, key mapping, LIGHTSYNC effects, audio, webcam, wheels, firmware, game detection, cloud profiles and automatic profile switching are not implemented. USB discovery alone does not imply a device can be configured. Linux hardware controls require `ratbagd`, `ratbagctl` and a device supported by libratbag.

The bundled ID catalog is generated from [libratbag's device files](https://github.com/libratbag/libratbag/tree/master/data/devices) at the pinned commit recorded in `data/libratbag-logitech.json`. It is a Linux driver reference, **not** an exhaustive list of G HUB devices or a claim of hardware control. Connected Logitech USB devices are still shown even if absent from that catalog. Maintainers can refresh it with `python3 tools/refresh_libratbag_catalog.py /path/to/libratbag`.

## Build

Install the stable Rust toolchain. On Linux, install `libudev-dev`, `libx11-dev`, `libxi-dev`, `libxcursor-dev`, `libxrandr-dev`, `libxkbcommon-dev`, `libwayland-dev` and `pkg-config` or their distribution equivalents. Then run:

```sh
cargo run --locked --release
```

The application stores profiles in the operating system's user configuration directory. It makes no network requests and requires no account. No personal data or service credentials are included in the repository.

## Roadmap toward G HUB parity

1. Verify device-specific HID++ capabilities with physical Logitech devices on each system.
2. Add a safe capability table and tests for DPI, polling rate, RGB and onboard profiles.
3. Implement button mapping and a bounded macro engine with explicit user control.
4. Add headset, microphone, webcam and racing device adapters where documented protocols exist.
5. Add per-game profile switching, import/export and accessible packaging for all three platforms.

Hardware write support will be enabled per device only after testing. Claims of full G HUB parity would be premature because many functions depend on undocumented model-specific protocols.

## Project

Maintainer: [jeremdevgit](https://github.com/jeremdevgit). Licensed under GPL-3.0-or-later. See [SECURITY.md](SECURITY.md) and [CHANGELOG.md](CHANGELOG.md). Interface: français, English, español, Deutsch, italiano, português, 简体中文, 日本語.
