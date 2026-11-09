# Device compatibility / Compatibilité des périphériques

LJ Hub aims to recognize all currently available Logitech G devices. A device can be **detected**, **listed in an external driver catalog**, or **actually configurable**; these are different states.

| State | Meaning |
|---|---|
| USB detected | The operating system exposes Logitech vendor ID `046d` to HID discovery. |
| System inventory | The OS lists a present Logitech USB/PnP device, including non-HID classes such as cameras and wheels. This provides identification only. |
| Catalog entry | An ID exists in the pinned [libratbag](https://github.com/libratbag/libratbag/tree/master/data/devices) source. Its driver is a Linux reference only. |
| ratbagd available | A running Linux `ratbagd` exposes the device through `ratbagctl list`; LJ Hub enables its DPI and report interval action. |
| Windows HID++ candidate | A vendor HID interface is present. LJ Hub can query features `0x2201` and `0x8060` on directly connected devices, validate supported values and read back writes. This is experimental and does not cover receiver children. |
| Verified | The exact device model, firmware, connection mode and each function have been tested on physical hardware. This project has not yet published such a verified matrix. |

The bundled catalog contains 116 Logitech transport and product ID pairs from libratbag commit `8235b5bb6032dea15901ab58a8218956f4494d08`. It includes some non-gaming Logitech models. It is not Logitech's G HUB device list, and it does not cover every Logitech G device. The application still detects USB devices absent from the catalog. Wireless receivers may expose a receiver ID instead of the paired device model.

For full G HUB functionality, separate adapters and validation are needed for mice, keyboards, headsets and microphones, webcams and lights, racing wheels/pedals/shifters, and wireless receivers. An adapter must query capabilities and refuse unsupported writes. A feature will be marked supported only after read-back and testing with its exact model.

## Français

LJ Hub vise tous les périphériques Logitech G disponibles. **Détecté**, **référencé dans le catalogue** et **pilotable** sont trois états distincts. Les 116 identifiants inclus viennent de libratbag et ne constituent ni la liste exhaustive de G HUB ni une preuve que LJ Hub pilote ces modèles. Sous Linux, seuls les périphériques réellement présentés par `ratbagd` peuvent recevoir le DPI et l'intervalle. Chaque fonction devra être validée sur le modèle et le micrologiciel correspondants avant d'être déclarée compatible.
