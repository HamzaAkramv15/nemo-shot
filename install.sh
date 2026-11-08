#!/usr/bin/env bash
# yami-shot: build and install into ~/.local, no root needed. Needs Rust 1.89 or newer.
#   ./install.sh               build and install
#   ./install.sh --deps        what it needs from the system
#   ./install.sh --uninstall   remove it
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"
PREFIX="${PREFIX:-$HOME/.local}"
DATA="${XDG_DATA_HOME:-$HOME/.local/share}"
case "${1:-}" in
  --deps) echo "grim, wl-clipboard; optional: tesseract, zbar, yami-files or zenity"; exit 0 ;;
  --uninstall)
    rm -f "$PREFIX/bin/yami-shot"
    echo "yami-shot removed"; exit 0 ;;
esac
cargo build --release --locked
install -Dm755 target/release/yami-shot "$PREFIX/bin/yami-shot"

echo "yami-shot installed to $PREFIX/bin"
