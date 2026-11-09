# LJ Hub

[English](README.md) · [**Français**](README.fr.md) · [Español](README.es.md)

Gestionnaire léger et local de profils pour périphériques Logitech, sur Windows, macOS et Linux. **Préversion initiale : LJ Hub ne remplace pas encore G HUB.** Projet indépendant, sans affiliation avec Logitech.

## Télécharger

Téléchargez la [préversion 0.1.3](https://github.com/jeremdevgit/lj-hub/releases/tag/v0.1.3-alpha.1). Sous Windows, extrayez le ZIP x64 et lancez `lj-hub.exe`. Sous macOS, extrayez le ZIP Apple Silicon et ouvrez `LJ Hub.app`. Sous Linux, extrayez le ZIP x64 et lancez `lj-hub`. L'application est portable ; aucun installateur n'est encore fourni.

La version actuelle détecte les périphériques USB HID Logitech, gère des profils locaux et propose une interface en huit langues : français, anglais, espagnol, allemand, italien, portugais, chinois simplifié et japonais. Elle permet aussi de consulter 116 identifiants issus du catalogue `libratbag` ; cette liste n'est ni exhaustive pour G HUB ni une garantie de compatibilité. Sous Windows, elle tente de régler le DPI et l'intervalle de rapport sur les souris filaires qui annoncent ces fonctions HID++ 2.0 ; chaque écriture est relue, mais cette prise en charge reste expérimentale faute de test sur des modèles physiques. Sous Linux, ces réglages passent par `ratbagd` sur les souris prises en charge.

L'inventaire système affiche aussi les périphériques USB/PnP Logitech non HID, notamment les caméras et volants. Il ne les configure pas encore. Le contrôle RVB de LJ Hub est **désactivé par défaut** pour laisser l'éclairage à rJb ; sous Linux, il peut être activé manuellement pour envoyer une couleur fixe à la LED 0 des modèles compatibles avec `ratbagd`. Macros, affectation des boutons, autres effets lumineux, audio, webcams, volants, micrologiciels et changement automatique de profil restent à développer. La détection ne garantit pas la compatibilité de configuration.

## Compilation

Installe Rust stable. Sous Linux, installe les paquets de développement `libudev`, X11, Wayland et `pkg-config` (détail dans [README.md](README.md)). Puis :

```sh
cargo run --locked --release
```

Les profils restent dans le dossier de configuration de l'utilisateur. Aucun compte ni accès réseau n'est nécessaire. Projet maintenu par [jeremdevgit](https://github.com/jeremdevgit), sous licence GPL-3.0-or-later.
