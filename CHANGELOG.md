# Changelog / Journal des modifications

## 0.2.0-alpha

- Fix the macOS launch crash: HID enumeration now runs off the UI thread / Correction du plantage au lancement sur macOS : la détection HID s'exécute hors du fil d'interface.
- Render Chinese and Japanese with a system CJK font loaded on demand, and replace icons missing from the default fonts / Affichage du chinois et du japonais avec une police CJK du système chargée à la demande, et icônes manquantes remplacées.
- Add a demonstration mode for screenshots / Mode démonstration pour les captures.
- Rebuild the interface around connected-device cards, category previews, device details and explicit capability states / Nouvelle interface avec cartes des appareils connectés, aperçus par catégorie, fiches et état explicite des fonctions.
- Add profile duplication and validated JSON copy/restore / Duplication des profils et sauvegarde JSON locale validée.
- Move hardware commands to a worker thread so the interface stays responsive / Commandes matérielles exécutées hors du fil d'interface.
- Package Windows installer, macOS disk image and Linux Debian package in CI / Création en CI d'un installateur Windows, d'une image disque macOS et d'un paquet Debian Linux.

## 0.1.3-alpha

- Add experimental Windows HID++ 2.0 DPI and report interval controls, with feature and value checks plus read-back / Ajout du réglage expérimental du DPI et de l'intervalle sous Windows avec vérification des fonctions, des valeurs et du résultat.
- Keep unsupported Logitech devices visible without claiming hardware control / Les périphériques non pilotables restent visibles sans promesse de contrôle matériel.
- Handle an empty Windows PnP inventory and test repeated profile saving on Windows / Gestion d'un inventaire PnP vide et test de sauvegardes répétées sous Windows.

## 0.1.2-alpha

- RGB control defaults to off so rJb can own lighting; old settings inherit this safe default / Contrôle RVB désactivé par défaut pour laisser l'éclairage à rJb, y compris pour les anciens réglages.
- Separate Linux DPI, report interval and optional LED 0 color commands / Commandes Linux distinctes pour le DPI, l'intervalle et la couleur facultative de la LED 0.
- System USB/PnP inventory for Logitech cameras, wheels and other device classes / Inventaire USB/PnP des caméras, volants et autres catégories Logitech.
- Preserve an invalid settings file instead of overwriting it / Conservation d'un fichier de réglages invalide au lieu de l'écraser.
- Preserve multiple connected devices of the same product ID / Conservation de plusieurs périphériques ayant le même identifiant produit.

## 0.1.1-alpha

- Searchable catalog of 116 Logitech identifiers from pinned libratbag data / Catalogue consultable de 116 identifiants Logitech issus d'une révision fixée de libratbag.
- Device discovery distinguishes a known Linux driver entry from actual ratbagd availability / La détection distingue une entrée du catalogue Linux d'un périphérique réellement pilotable via ratbagd.

## 0.1.0-alpha

- Local Logitech USB HID discovery / Détection locale des périphériques USB HID Logitech.
- Local profile editor with eight interface languages / Éditeur de profils local en huit langues.
- Linux DPI and report interval support through ratbagd for compatible mice / Réglage du DPI et de l'intervalle sous Linux via ratbagd pour les souris compatibles.
- Initial cross-platform build and security checks / Premiers contrôles de compilation et de sécurité multiplateformes.
