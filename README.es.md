# LJ Hub

Gestor ligero y local de perfiles para dispositivos Logitech en Windows, macOS y Linux. **Versión alfa: aún no reemplaza G HUB.** Proyecto independiente, sin afiliación con Logitech.

La versión actual detecta dispositivos USB HID Logitech, guarda perfiles locales y ofrece una interfaz en ocho idiomas: francés, inglés, español, alemán, italiano, portugués, chino simplificado y japonés. En Linux, permite configurar DPI y el intervalo de informe de ratones compatibles con `ratbagd`. El color RGB se guarda en el perfil, pero todavía no se aplica al hardware.

Macros, asignación de botones, iluminación, audio, cámaras, volantes, firmware y cambio automático de perfil siguen pendientes. Detectar un dispositivo USB no garantiza que pueda configurarse.

## Compilación

Instala Rust estable y las dependencias de desarrollo indicadas en [README.md](README.md). Ejecuta:

```sh
cargo run --locked --release
```

Los perfiles se guardan localmente. No se necesita una cuenta ni acceso a Internet. Mantenido por [jeremdevgit](https://github.com/jeremdevgit), bajo GPL-3.0-or-later.
