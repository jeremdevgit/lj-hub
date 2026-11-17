<p align="center"><img src="installer/linux/lj-hub.svg" width="112" alt=""></p>

<h1 align="center">LJ Hub</h1>

<p align="center"><b>Lokale Profile und Inventar für Logitech-Geräte, mit experimenteller DPI- und Abfrageraten-Steuerung.</b></p>

<p align="center"><img src="https://img.shields.io/badge/%F0%9F%9B%A0%EF%B8%8F%20System%20%26%20hardware-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux-2f6fde?style=flat-square" alt="System & Hardware"> <a href="https://github.com/jeremdevgit/lj-hub/releases"><img src="https://img.shields.io/github/v/release/jeremdevgit/lj-hub?include_prereleases&style=flat-square&label=version" alt="version"></a> <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0--or--later-555?style=flat-square" alt="GPL-3.0-or-later"></a> <a href="https://github.com/jeremdevgit/lj-hub/actions/workflows/ci.yml"><img src="https://github.com/jeremdevgit/lj-hub/actions/workflows/ci.yml/badge.svg" alt="CI"></a></p>

<p align="center"><a href="README.md">English</a> · <a href="README.fr.md">Français</a> · <a href="README.es.md">Español</a> · <b>Deutsch</b> · <a href="README.it.md">Italiano</a> · <a href="README.pt.md">Português</a> · <a href="README.zh.md">简体中文</a> · <a href="README.ja.md">日本語</a></p>

<p align="center"><a href="https://github.com/jeremdevgit/lj-hub/releases"><b>⬇ Herunterladen</b></a></p>

![LJ Hub](docs/screenshots/home-en.png)

LJ Hub ist eine schlanke, lokale Verwaltung für Logitech-Gaming-Geräte. **Es ist eine frühe Alpha und noch kein Ersatz für G HUB**, ein unabhängiges Projekt ohne Verbindung zu Logitech.

<p><img src="docs/screenshots/device-en.png" alt="Geräteseite" width="49%"> <img src="docs/screenshots/settings-en.png" alt="Einstellungen" width="49%"></p>

## Auf einen Blick

| | |
|---|---|
| **Kategorie** | 🛠️ System & Hardware |
| **Plattformen** | Windows · macOS · Linux |
| **Oberfläche** | 8 Sprachen (EN, FR, ES, DE, IT, PT, ZH, JA) |
| **Preis** | Kostenlos, quelloffen, ohne Werbung und Telemetrie |
| **Status** | Frühe Alpha |
| **Lizenz** | [GPL-3.0-or-later](LICENSE) |

## Was heute funktioniert

| Funktion | Windows | macOS | Linux |
|---|:-:|:-:|:-:|
| Logitech-USB-HID-Geräte erkennen | ✅ | ✅ | ✅ |
| Lokale Profile erstellen, bearbeiten, duplizieren und sichern (JSON) | ✅ | ✅ | ✅ |
| Dashboard mit Kategorievorschau und Fähigkeitsstatus | ✅ | ✅ | ✅ |
| 116 Logitech-IDs durchsuchen (fixierter libratbag-Katalog) | ✅ | ✅ | ✅ |
| USB/PnP-Inventar des Systems (Kameras, Lenkräder …) | ✅ | ✅ | ✅ |
| DPI und Abfragerate einstellen | 🧪 kabelgebundene HID++-2.0-Mäuse | — | 🧪 über `ratbagd` |
| Feste Farbe auf LED 0 | — | — | 🧪 optional, über `ratbagd` |

🧪 = experimentell, noch nicht an echten Modellen geprüft.

RGB ist **standardmäßig aus**, damit [rJb](https://github.com/jeremdevgit/rjb) die Beleuchtung übernimmt. Makros, Tastenbelegung, LIGHTSYNC-Effekte, Audio, Webcams, Lenkräder, Firmware, Spielerkennung und automatischer Profilwechsel sind nicht umgesetzt. Ein Gerät im Inventar ist nicht automatisch konfigurierbar.

## Installation

Lade die neueste Vorabversion von der [Release-Seite](https://github.com/jeremdevgit/lj-hub/releases): `.exe`-Installer pro Benutzer für Windows, Apple-Silicon-`.dmg` für macOS und `.deb` für Ubuntu/Debian x64. Die Windows- und macOS-Pakete sind nicht mit einem vertrauenswürdigen Herausgeberzertifikat signiert. Unter Linux brauchen Hardware-Einstellungen `ratbagd`, `ratbagctl` und ein von libratbag unterstütztes Gerät.

## Sicherheit und Datenschutz

- **Keine Netzwerkanfragen, kein Konto.** Profile liegen im Benutzer-Konfigurationsordner des Systems.
- Schreibzugriffe auf Hardware sind optional, werden gegen angekündigte Werte geprüft und danach zurückgelesen; sie werden erst nach Tests gerätespezifisch freigeschaltet.

## Fahrplan

1. HID++-Fähigkeiten auf echten Geräten für jedes System prüfen.
2. Eine getestete Fähigkeitstabelle für DPI, Abfragerate, RGB und Onboard-Profile.
3. Tastenbelegung und eine begrenzte Makro-Engine.
4. Headsets, Mikrofone, Webcams und Lenkräder, wo Protokolle dokumentiert sind.
5. Profilwechsel pro Spiel.

## Selbst bauen

```sh
# Rust stable. Unter Linux zusätzlich: libudev-dev libx11-dev libxi-dev libxcursor-dev
# libxrandr-dev libxkbcommon-dev libwayland-dev pkg-config
cargo run --locked --release
```

Der Katalog stammt aus [libratbags Gerätedateien](https://github.com/libratbag/libratbag/tree/master/data/devices) beim in `data/libratbag-logitech.json` fixierten Commit; aktualisieren mit `python3 tools/refresh_libratbag_catalog.py /pfad/zu/libratbag`. Änderungen: [CHANGELOG.md](CHANGELOG.md).

## Lizenz

GPL-3.0-or-later – siehe [LICENSE](LICENSE). Logitech, G HUB und LIGHTSYNC sind Marken ihres Inhabers.

---

Eine Sicherheitslücke gefunden? Bitte privat melden (siehe [SECURITY.md](SECURITY.md)), nicht in einem öffentlichen Issue.

## Weitere freie Software von jeremdevgit

Alle kostenlos, quelloffen und werbefrei.

**🛠️ System & Hardware** — [Boîte à outils](https://github.com/jeremdevgit/boite-a-outils) · [rJb](https://github.com/jeremdevgit/rjb) · [Decklight](https://github.com/jeremdevgit/decklight)

**📒 Produktivität & Daten** — [Carnet de notes](https://github.com/jeremdevgit/carnet-rust) · [JMeca](https://github.com/jeremdevgit/jmeca) · [Suite Libre](https://jeremdevgit.github.io/suite-libre/)

**🎮 Musik & Spiele** — [Pomme](https://github.com/jeremdevgit/pomme) · [Ludothèque](https://github.com/jeremdevgit/ludotheque) · [AutoDex](https://github.com/jeremdevgit/autodex) · [Batailles de France](https://github.com/jeremdevgit/batailles-de-france)
