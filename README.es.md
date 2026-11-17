<p align="center"><img src="installer/linux/lj-hub.svg" width="112" alt=""></p>

<h1 align="center">LJ Hub</h1>

<p align="center"><b>Perfiles e inventario locales para dispositivos Logitech, con control experimental de DPI y frecuencia.</b></p>

<p align="center"><img src="https://img.shields.io/badge/%F0%9F%9B%A0%EF%B8%8F%20System%20%26%20hardware-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux-2f6fde?style=flat-square" alt="Sistema y hardware"> <a href="https://github.com/jeremdevgit/lj-hub/releases"><img src="https://img.shields.io/github/v/release/jeremdevgit/lj-hub?include_prereleases&style=flat-square&label=version" alt="version"></a> <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0--or--later-555?style=flat-square" alt="GPL-3.0-or-later"></a> <a href="https://github.com/jeremdevgit/lj-hub/actions/workflows/ci.yml"><img src="https://github.com/jeremdevgit/lj-hub/actions/workflows/ci.yml/badge.svg" alt="CI"></a></p>

<p align="center"><a href="README.md">English</a> · <a href="README.fr.md">Français</a> · <b>Español</b> · <a href="README.de.md">Deutsch</a> · <a href="README.it.md">Italiano</a> · <a href="README.pt.md">Português</a> · <a href="README.zh.md">简体中文</a> · <a href="README.ja.md">日本語</a></p>

<p align="center"><a href="https://github.com/jeremdevgit/lj-hub/releases"><b>⬇ Descargar</b></a></p>

![LJ Hub](docs/screenshots/home-en.png)

LJ Hub es un gestor ligero y local para dispositivos gaming de Logitech. **Es una alfa temprana, todavía no sustituye a G HUB**, y es un proyecto independiente, no afiliado a Logitech.

<p><img src="docs/screenshots/device-en.png" alt="Página de un dispositivo" width="49%"> <img src="docs/screenshots/settings-en.png" alt="Ajustes" width="49%"></p>

## De un vistazo

| | |
|---|---|
| **Categoría** | 🛠️ Sistema y hardware |
| **Plataformas** | Windows · macOS · Linux |
| **Interfaz** | 8 idiomas (EN, FR, ES, DE, IT, PT, ZH, JA) |
| **Precio** | Gratis, de código abierto, sin anuncios ni telemetría |
| **Estado** | Alfa |
| **Licencia** | [GPL-3.0-or-later](LICENSE) |

## Qué funciona hoy

| Función | Windows | macOS | Linux |
|---|:-:|:-:|:-:|
| Detectar dispositivos USB HID de Logitech | ✅ | ✅ | ✅ |
| Crear, editar, duplicar y respaldar perfiles locales (JSON) | ✅ | ✅ | ✅ |
| Panel con vistas por categoría y estado de capacidades | ✅ | ✅ | ✅ |
| Buscar 116 ID de Logitech (catálogo libratbag fijado) | ✅ | ✅ | ✅ |
| Inventario USB/PnP del sistema (cámaras, volantes…) | ✅ | ✅ | ✅ |
| Ajustar DPI y frecuencia de sondeo | 🧪 ratones con cable HID++ 2.0 | — | 🧪 con `ratbagd` |
| Color fijo en el LED 0 | — | — | 🧪 opcional, con `ratbagd` |

🧪 = experimental, aún no verificado en modelos físicos.

El RGB está **desactivado por defecto** para que [rJb](https://github.com/jeremdevgit/rjb) gestione la iluminación. Macros, reasignación de teclas, efectos LIGHTSYNC, audio, webcams, volantes, firmware, detección de juegos y cambio automático de perfil no están implementados. Ver un dispositivo en el inventario no significa que se pueda configurar.

## Instalar

Descarga la última versión preliminar en la [página de versiones](https://github.com/jeremdevgit/lj-hub/releases): instalador `.exe` por usuario para Windows, `.dmg` Apple Silicon para macOS y `.deb` para Ubuntu/Debian x64. Los paquetes de Windows y macOS no están firmados por un editor de confianza. En Linux, los controles de hardware requieren `ratbagd`, `ratbagctl` y un dispositivo compatible con libratbag.

## Seguridad y privacidad

- **Sin peticiones de red, sin cuenta.** Los perfiles se guardan en la carpeta de configuración del usuario.
- Las escrituras en hardware son opcionales, se comprueban con los valores anunciados y se vuelven a leer; se activarán dispositivo a dispositivo solo tras probarlas.

## Hoja de ruta

1. Verificar las capacidades HID++ en dispositivos reales en cada sistema.
2. Una tabla de capacidades probada para DPI, frecuencia, RGB y perfiles internos.
3. Reasignación de botones y un motor de macros acotado.
4. Auriculares, micrófonos, webcams y volantes donde los protocolos estén documentados.
5. Cambio de perfil por juego.

## Compilar

```sh
# Rust estable. En Linux también: libudev-dev libx11-dev libxi-dev libxcursor-dev
# libxrandr-dev libxkbcommon-dev libwayland-dev pkg-config
cargo run --locked --release
```

El catálogo procede de los [archivos de dispositivos de libratbag](https://github.com/libratbag/libratbag/tree/master/data/devices) en el commit fijado en `data/libratbag-logitech.json`; se actualiza con `python3 tools/refresh_libratbag_catalog.py /ruta/a/libratbag`. Cambios: [CHANGELOG.md](CHANGELOG.md).

## Licencia

GPL-3.0-or-later — ver [LICENSE](LICENSE). Logitech, G HUB y LIGHTSYNC son marcas de su propietario.

---

¿Has encontrado una vulnerabilidad? Infórmala en privado (consulta [SECURITY.md](SECURITY.md)) y no en una incidencia pública.

## Más software libre de jeremdevgit

Todos gratuitos, de código abierto y sin anuncios.

**🛠️ Sistema y hardware** — [Boîte à outils](https://github.com/jeremdevgit/boite-a-outils) · [rJb](https://github.com/jeremdevgit/rjb) · [Decklight](https://github.com/jeremdevgit/decklight)

**📒 Productividad y datos** — [Carnet de notes](https://github.com/jeremdevgit/carnet-rust) · [JMeca](https://github.com/jeremdevgit/jmeca) · [Suite Libre](https://jeremdevgit.github.io/suite-libre/)

**🎮 Música y juegos** — [Pomme](https://github.com/jeremdevgit/pomme) · [Ludothèque](https://github.com/jeremdevgit/ludotheque) · [AutoDex](https://github.com/jeremdevgit/autodex) · [Batailles de France](https://github.com/jeremdevgit/batailles-de-france)
