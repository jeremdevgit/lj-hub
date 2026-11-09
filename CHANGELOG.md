# Changelog / Journal des modifications

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
