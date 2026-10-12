#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

app_id="io.github.meelee35.screencap"
name="$(awk -F'"' '/^name = / { print $2; exit }' Cargo.toml)"
version="$(awk -F'"' '/^version = / { print $2; exit }' Cargo.toml)"
arch="$(uname -m)"

rm -rf dist
mkdir -p dist

cargo build --release --locked
strip -s "target/release/$name"

cargo generate-rpm
cargo deb

cp target/generate-rpm/*.rpm dist/
cp target/debian/*.deb dist/

stage="$(mktemp -d)"
root="$stage/$name-$version"
mkdir -p "$root/bin"
mkdir -p "$root/share/applications"
mkdir -p "$root/share/icons/hicolor/scalable/apps"
mkdir -p "$root/share/licenses/$name"
install -m 755 "target/release/$name" "$root/bin/$name"
install -m 644 "packaging/portable.desktop" "$root/share/applications/$app_id.desktop"
install -m 644 "assets/$app_id.svg" "$root/share/icons/hicolor/scalable/apps/$app_id.svg"
install -m 644 LICENSE "$root/share/licenses/$name/LICENSE"
tar -C "$stage" -czf "dist/$name-$version-$arch-linux.tar.gz" "$name-$version"
rm -rf "$stage"
