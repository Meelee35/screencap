# Screencap

[![Build](https://github.com/Meelee35/screencap/actions/workflows/rust.yml/badge.svg)](https://github.com/Meelee35/screencap/actions/workflows/rust.yml)
[![Release](https://img.shields.io/github/v/release/Meelee35/screencap)](https://github.com/Meelee35/screencap/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/Meelee35/screencap/total)](https://github.com/Meelee35/screencap/releases)
[![License](https://img.shields.io/github/license/Meelee35/screencap)](https://github.com/Meelee35/screencap/blob/main/LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Wayland-informational)](https://github.com/Meelee35/screencap)
[![UI: Slint](https://img.shields.io/badge/UI-Slint-blueviolet)](https://slint.dev)
[![Desktop: KDE Plasma](https://img.shields.io/badge/desktop-KDE%20Plasma-1d99f3)](https://kde.org)

A lightweight and fast screen capturing utility for Wayland.

<img width="1920" height="1009" alt="Screenshot_20261012_153641" src="https://github.com/user-attachments/assets/0104a111-f7ae-4d82-a1c6-e5a057aff4cc" />

## Features
- Fullscreen and region capture modes
- Copy the capture to the clipboard (Requires wl-copy to be installed)
- Save the capture to the disk (Defaults to ~/Pictures)
- Direct screen capture on KWin, but supports other desktop environments via the portal
- Configurable using the CLI or by editing a TOML

## Requirements
- Wayland
- KDE Plasma or a desktop environment with xdg-desktop-portal
- wl-copy for the clipboard features

## Installation
### Fedora / RPM
Download the `.rpm` from the [latest release](https://github.com/Meelee35/screencap/releases/latest)
and install it:

```sh
sudo dnf install ./screencap-<version>-1.x86_64.rpm
```
### Debian / Ubuntu
Download the `.deb` from the [latest release](https://github.com/Meelee35/screencap/releases/latest) and install it:

```sh
sudo apt install ./screencap_<version>-1_amd64.deb
```

### Other / Portable
Download the `.tar.gz` from the [latest release](https://github.com/Meelee35/screencap/releases/latest).
If you want to install it permanently, run the following:
```
tar -xzf screencap-<version>-x86_64-linux.tar.gz
install -Dm755 screencap-<version>/bin/screencap ~/.local/bin/screencap
install -Dm644 screencap-<version>/share/applications/io.github.meelee35.screencap.desktop \
  ~/.local/share/applications/io.github.meelee35.screencap.desktop
install -Dm644 screencap-<version>/share/icons/hicolor/scalable/apps/io.github.meelee35.screencap.svg \
  ~/.local/share/icons/hicolor/scalable/apps/io.github.meelee35.screencap.svg
```
Make sure `~/.local/bin` is on your `PATH`.
### From Source
```sh
git clone https://github.com/Meelee35/screencap
cd screencap
cargo build --release
./target/release/screencap
```
## Usage
Launch **Screencap** from your application menu, or add a keybind for it using your distribution's tools, drag to select a region, and the capture is copied
and/or saved according to your configuration.
Command-line options:

```text
Usage: screencap [OPTIONS] [COMMAND]

Options:
  -s, --savedirectory <SAVEDIRECTORY>  where to save captures
  -c, --clipboard <CLIPBOARD>          whether to save captures to the clipboard [possible values: true, false]
  -h, --help                           Print help

Commands (Optional):
  config-write     write a fresh configuration file
  config-override  update individual configuration values
```

## Configuration

On first run Screencap writes a default configuration to:

```
~/.config/io.github.meelee35.screencap/config.toml
```

```toml
save_directory = "/home/you/Pictures"
clipboard = true
```

Set `save_directory` to change where captures are saved, and `clipboard` to control whether captures
are copied to the clipboard. Command-line flags override these values for a single run, unless a command to do otherwise is specified.

## Building packages

The packaging script needs [`cargo-generate-rpm`](https://crates.io/crates/cargo-generate-rpm) and
[`cargo-deb`](https://crates.io/crates/cargo-deb):

```sh
cargo install cargo-generate-rpm cargo-deb
./packaging/build.sh
```

Artifacts are written to `dist/`.

## Contributing

Issues and pull requests are welcome. Please run `cargo test` and `cargo clippy` before opening a PR.
