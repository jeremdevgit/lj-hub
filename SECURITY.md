# Security / Sécurité

Report security issues privately using GitHub's **Report a vulnerability** feature. Please do not publish an exploit before a fix is available. Include the affected version, platform, device model and steps to reproduce. No service credentials are required by LJ Hub.

Signalez une faille en privé via **Report a vulnerability** sur GitHub. Merci d'indiquer la version, la plateforme, le modèle du périphérique et les étapes de reproduction. LJ Hub ne demande aucun identifiant de service.

## Boundaries / Limites

Profiles are local JSON files, not encrypted secrets. Do not store passwords in profile notes. Linux hardware writes are limited to validated DPI, report interval and optional LED 0 color arguments passed directly to `ratbagctl`, without a shell. RGB writes are disabled by default so rJb can own lighting. Hardware compatibility is model-specific. The app does not download drivers or firmware.

Les profils sont des fichiers JSON locaux, pas des coffres chiffrés. Ne stockez pas de mots de passe dans les notes. Sous Linux, seuls le DPI, l'intervalle et la couleur facultative de la LED 0 sont transmis directement à `ratbagctl`, sans shell. Le RVB est désactivé par défaut pour laisser le contrôle à rJb. La compatibilité dépend du modèle. L'application ne télécharge ni pilote ni micrologiciel.

Dependency audit on 2026-11-08: zero known vulnerabilities in `Cargo.lock`; informational maintenance notices for transitive `paste` and `ttf-parser`. / Audit du 08/11/2026 : aucune vulnérabilité connue ; avis de maintenance pour les dépendances transitives `paste` et `ttf-parser`.
