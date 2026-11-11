#!/bin/zsh
set -euo pipefail
mkdir -p 'dist/macos/LJ Hub.app/Contents/MacOS'
cp target/release/lj-hub 'dist/macos/LJ Hub.app/Contents/MacOS/lj-hub'
cp installer/macos/Info.plist 'dist/macos/LJ Hub.app/Contents/Info.plist'
chmod 755 'dist/macos/LJ Hub.app/Contents/MacOS/lj-hub'
codesign --force --deep --sign - 'dist/macos/LJ Hub.app'
codesign --verify --deep --strict 'dist/macos/LJ Hub.app'
ln -s /Applications dist/macos/Applications
hdiutil create -volname 'LJ Hub' -srcfolder dist/macos -ov -format UDZO 'dist/LJ-Hub-0.2.0-alpha-macOS-Apple-Silicon.dmg'
hdiutil verify 'dist/LJ-Hub-0.2.0-alpha-macOS-Apple-Silicon.dmg'
