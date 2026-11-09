# LJ Hub

Gestor ligero y local de perfiles para dispositivos Logitech en Windows, macOS y Linux. **Versión alfa: aún no reemplaza G HUB.** Proyecto independiente, sin afiliación con Logitech.

La versión actual detecta dispositivos USB HID Logitech, guarda perfiles locales y ofrece una interfaz en ocho idiomas: francés, inglés, español, alemán, italiano, portugués, chino simplificado y japonés. También permite consultar 116 identificadores del catálogo de `libratbag`; la lista no es exhaustiva para G HUB ni garantiza compatibilidad. En Windows, intenta configurar el DPI y el intervalo de informe en ratones con cable que anuncian las funciones HID++ 2.0 correspondientes y comprueba el resultado mediante lectura; esta función sigue siendo experimental y aún no se ha validado con modelos físicos. En Linux, estos ajustes pasan por `ratbagd` para ratones compatibles.

El inventario del sistema también muestra dispositivos Logitech USB/PnP no HID, como cámaras y volantes, aunque aún no permite configurarlos. El control RGB de LJ Hub está **desactivado por defecto** para dejar la iluminación a rJb. En Linux puede activarse manualmente para enviar un color fijo al LED 0 de dispositivos compatibles con `ratbagd`. Macros, asignación de botones, otros efectos de iluminación, audio, cámaras, volantes, firmware y cambio automático de perfil siguen pendientes. Detectar un dispositivo no garantiza que pueda configurarse.

## Compilación

Instala Rust estable y las dependencias de desarrollo indicadas en [README.md](README.md). Ejecuta:

```sh
cargo run --locked --release
```

Los perfiles se guardan localmente. No se necesita una cuenta ni acceso a Internet. Mantenido por [jeremdevgit](https://github.com/jeremdevgit), bajo GPL-3.0-or-later.
