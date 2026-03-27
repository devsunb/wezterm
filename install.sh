#!/usr/bin/env bash

set -Eeuo pipefail

cargo build --release -p wezterm -p wezterm-gui -p wezterm-mux-server -p strip-ansi-escapes

APP=/tmp/wezterm-build/WezTerm.app
rm -rf /tmp/wezterm-build && mkdir -p /tmp/wezterm-build
cp -r assets/macos/WezTerm.app "$APP"
rm -f "$APP"/*.dylib
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp -r assets/shell-integration/* "$APP/Contents/Resources"
cp -r assets/shell-completion "$APP/Contents/Resources"
tic -xe wezterm -o "$APP/Contents/Resources/terminfo" termwiz/data/wezterm.terminfo
for bin in wezterm wezterm-mux-server wezterm-gui strip-ansi-escapes; do
  cp target/release/$bin "$APP/Contents/MacOS/$bin"
done

codesign --force --deep --sign - --identifier com.github.wez.wezterm "$APP"
codesign --verify --deep --strict --verbose=2 "$APP"

rm -rf /Applications/WezTerm.app
cp -R "$APP" /Applications/
ln -sf /Applications/WezTerm.app/Contents/MacOS/wezterm "$HOME/.local/bin/wezterm"
