#!/usr/bin/env bash
# /usr/lib/wsl/first-setup-aphrody-omarchy.sh
set -euo pipefail

declare -r DEFAULT_UID=1000
declare -r DEFAULT_USER="aphrody"
declare -r DEFAULT_GROUPS="wheel,video,render"

echo "Initializing Sovereign Omarchy + CachyOS + Arch WSL2 Distribution..."
pacman-key --init >/dev/null 2>&1 || true
pacman-key --populate archlinux cachyos >/dev/null 2>&1 || true

if [ -x /usr/lib/wsl/optimize-repos ]; then
    /usr/lib/wsl/optimize-repos || true
fi

if ! id -u "${DEFAULT_USER}" >/dev/null 2>&1; then
    useradd --create-home --uid "${DEFAULT_UID}" --groups "${DEFAULT_GROUPS}" --shell /usr/bin/fish "${DEFAULT_USER}"
    passwd -d "${DEFAULT_USER}" >/dev/null 2>&1 || true
fi

install -d -m 0700 -o "${DEFAULT_UID}" -g "${DEFAULT_UID}" "/home/${DEFAULT_USER}/.config/hypr"
install -d -m 0700 -o "${DEFAULT_UID}" -g "${DEFAULT_UID}" "/home/${DEFAULT_USER}/.config/cosmic"
install -d -m 0700 -o "${DEFAULT_UID}" -g "${DEFAULT_UID}" "/home/${DEFAULT_USER}/.config/omarchy/current"

echo "D3D12 /dev/dxg + Rust WSLg (Hyprland + COSMIC + Omarchy tokyo-night) ready."
