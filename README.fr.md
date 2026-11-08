# LJ Hub

Gestionnaire léger et local de profils pour périphériques Logitech, sur Windows, macOS et Linux. **Préversion initiale : LJ Hub ne remplace pas encore G HUB.** Projet indépendant, sans affiliation avec Logitech.

La version actuelle détecte les périphériques USB HID Logitech, gère des profils locaux et propose une interface en huit langues : français, anglais, espagnol, allemand, italien, portugais, chinois simplifié et japonais. Elle permet aussi de consulter 116 identifiants issus du catalogue `libratbag` ; cette liste n'est ni exhaustive pour G HUB ni une garantie de compatibilité. Sous Linux, elle règle le DPI et l'intervalle de rapport des souris prises en charge par `ratbagd`. La couleur RVB est enregistrée dans le profil, mais n'est pas encore appliquée au matériel.

Macros, affectation des boutons, effets lumineux, audio, webcams, volants, micrologiciels et changement automatique de profil restent à développer. La détection USB ne garantit pas la compatibilité de configuration.

## Compilation

Installe Rust stable. Sous Linux, installe les paquets de développement `libudev`, X11, Wayland et `pkg-config` (détail dans [README.md](README.md)). Puis :

```sh
cargo run --locked --release
```

Les profils restent dans le dossier de configuration de l'utilisateur. Aucun compte ni accès réseau n'est nécessaire. Projet maintenu par [jeremdevgit](https://github.com/jeremdevgit), sous licence GPL-3.0-or-later.
