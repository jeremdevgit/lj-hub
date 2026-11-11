#!/usr/bin/env bash
set -euo pipefail
package=dist/linux-package
mkdir -p "$package/DEBIAN" "$package/usr/bin" "$package/usr/share/applications" "$package/usr/share/icons/hicolor/scalable/apps" dist
install -m 755 target/release/lj-hub "$package/usr/bin/lj-hub"
install -m 644 installer/linux/lj-hub.desktop "$package/usr/share/applications/lj-hub.desktop"
install -m 644 installer/linux/lj-hub.svg "$package/usr/share/icons/hicolor/scalable/apps/lj-hub.svg"
cat > "$package/DEBIAN/control" <<'EOF'
Package: lj-hub
Version: 0.2.0~alpha1
Section: utils
Priority: optional
Architecture: amd64
Maintainer: jeremdevgit <noreply@github.com>
Depends: libc6 (>= 2.35), libudev1, libx11-6, libgl1
Description: Local Logitech device profiles and inventory
 Lightweight local application for Logitech device discovery and profiles.
EOF
dpkg-deb --build --root-owner-group "$package" dist/LJ-Hub-0.2.0-alpha-Linux-amd64.deb
dpkg-deb --info dist/LJ-Hub-0.2.0-alpha-Linux-amd64.deb
