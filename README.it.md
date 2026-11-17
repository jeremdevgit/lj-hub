<p align="center"><img src="installer/linux/lj-hub.svg" width="112" alt=""></p>

<h1 align="center">LJ Hub</h1>

<p align="center"><b>Profili e inventario locali per i dispositivi Logitech, con controllo sperimentale di DPI e frequenza.</b></p>

<p align="center"><img src="https://img.shields.io/badge/%F0%9F%9B%A0%EF%B8%8F%20System%20%26%20hardware-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux-2f6fde?style=flat-square" alt="Sistema e hardware"> <a href="https://github.com/jeremdevgit/lj-hub/releases"><img src="https://img.shields.io/github/v/release/jeremdevgit/lj-hub?include_prereleases&style=flat-square&label=version" alt="version"></a> <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0--or--later-555?style=flat-square" alt="GPL-3.0-or-later"></a> <a href="https://github.com/jeremdevgit/lj-hub/actions/workflows/ci.yml"><img src="https://github.com/jeremdevgit/lj-hub/actions/workflows/ci.yml/badge.svg" alt="CI"></a></p>

<p align="center"><a href="README.md">English</a> · <a href="README.fr.md">Français</a> · <a href="README.es.md">Español</a> · <a href="README.de.md">Deutsch</a> · <b>Italiano</b> · <a href="README.pt.md">Português</a> · <a href="README.zh.md">简体中文</a> · <a href="README.ja.md">日本語</a></p>

<p align="center"><a href="https://github.com/jeremdevgit/lj-hub/releases"><b>⬇ Scarica</b></a></p>

![LJ Hub](docs/screenshots/home-en.png)

LJ Hub è un gestore leggero e locale per i dispositivi gaming Logitech. **È un'alpha iniziale, non ancora un sostituto di G HUB**, e un progetto indipendente non affiliato a Logitech.

<p><img src="docs/screenshots/device-en.png" alt="Pagina di un dispositivo" width="49%"> <img src="docs/screenshots/settings-en.png" alt="Impostazioni" width="49%"></p>

## In breve

| | |
|---|---|
| **Categoria** | 🛠️ Sistema e hardware |
| **Piattaforme** | Windows · macOS · Linux |
| **Interfaccia** | 8 lingue (EN, FR, ES, DE, IT, PT, ZH, JA) |
| **Prezzo** | Gratuito, open source, senza pubblicità né telemetria |
| **Stato** | Alpha |
| **Licenza** | [GPL-3.0-or-later](LICENSE) |

## Cosa funziona oggi

| Funzione | Windows | macOS | Linux |
|---|:-:|:-:|:-:|
| Rilevare i dispositivi USB HID Logitech | ✅ | ✅ | ✅ |
| Creare, modificare, duplicare e salvare profili locali (JSON) | ✅ | ✅ | ✅ |
| Dashboard con anteprime per categoria e stato delle funzionalità | ✅ | ✅ | ✅ |
| Cercare 116 ID Logitech (catalogo libratbag fissato) | ✅ | ✅ | ✅ |
| Inventario USB/PnP del sistema (webcam, volanti…) | ✅ | ✅ | ✅ |
| Impostare DPI e frequenza di polling | 🧪 mouse cablati HID++ 2.0 | — | 🧪 tramite `ratbagd` |
| Colore fisso sul LED 0 | — | — | 🧪 facoltativo, tramite `ratbagd` |

🧪 = sperimentale, non ancora verificato su modelli reali.

L'RGB è **disattivato per impostazione predefinita** per lasciare l'illuminazione a [rJb](https://github.com/jeremdevgit/rjb). Macro, rimappatura dei tasti, effetti LIGHTSYNC, audio, webcam, volanti, firmware, rilevamento dei giochi e cambio automatico di profilo non sono implementati. Vedere un dispositivo nell'inventario non significa poterlo configurare.

## Installazione

Scarica l'ultima pre-release dalla [pagina delle versioni](https://github.com/jeremdevgit/lj-hub/releases): installer `.exe` per utente per Windows, `.dmg` Apple Silicon per macOS e `.deb` per Ubuntu/Debian x64. I pacchetti Windows e macOS non sono firmati da un editore attendibile. Su Linux i controlli hardware richiedono `ratbagd`, `ratbagctl` e un dispositivo supportato da libratbag.

## Sicurezza e privacy

- **Nessuna richiesta di rete, nessun account.** I profili restano nella cartella di configurazione utente del sistema.
- Le scritture sull'hardware sono facoltative, verificate rispetto ai valori dichiarati e poi rilette; saranno abilitate dispositivo per dispositivo solo dopo i test.

## Roadmap

1. Verificare le funzionalità HID++ su dispositivi reali per ogni sistema.
2. Una tabella di funzionalità testata per DPI, polling, RGB e profili integrati.
3. Rimappatura dei pulsanti e un motore di macro limitato.
4. Cuffie, microfoni, webcam e volanti dove i protocolli sono documentati.
5. Cambio di profilo per gioco.

## Compilare

```sh
# Rust stable. Su Linux anche: libudev-dev libx11-dev libxi-dev libxcursor-dev
# libxrandr-dev libxkbcommon-dev libwayland-dev pkg-config
cargo run --locked --release
```

Il catalogo deriva dai [file dei dispositivi di libratbag](https://github.com/libratbag/libratbag/tree/master/data/devices) al commit fissato in `data/libratbag-logitech.json`; si aggiorna con `python3 tools/refresh_libratbag_catalog.py /percorso/libratbag`. Modifiche: [CHANGELOG.md](CHANGELOG.md).

## Licenza

GPL-3.0-or-later — vedi [LICENSE](LICENSE). Logitech, G HUB e LIGHTSYNC sono marchi del rispettivo proprietario.

---

Hai trovato una vulnerabilità? Segnalala in privato (vedi [SECURITY.md](SECURITY.md)) e non in una issue pubblica.

## Altro software libero di jeremdevgit

Tutti gratuiti, open source e senza pubblicità.

**🛠️ Sistema e hardware** — [Boîte à outils](https://github.com/jeremdevgit/boite-a-outils) · [rJb](https://github.com/jeremdevgit/rjb) · [Decklight](https://github.com/jeremdevgit/decklight)

**📒 Produttività e dati** — [Carnet de notes](https://github.com/jeremdevgit/carnet-rust) · [JMeca](https://github.com/jeremdevgit/jmeca) · [Suite Libre](https://jeremdevgit.github.io/suite-libre/)

**🎮 Musica e giochi** — [Pomme](https://github.com/jeremdevgit/pomme) · [Ludothèque](https://github.com/jeremdevgit/ludotheque) · [AutoDex](https://github.com/jeremdevgit/autodex) · [Batailles de France](https://github.com/jeremdevgit/batailles-de-france)
