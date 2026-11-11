#!/usr/bin/env bash
# build-aphrody-omarchy-wsl.sh - Reproducible RootFS & .wsl Builder
# Fuses CachyOS/cachyos-wsl + omacom/omarchy + gitlab.archlinux.org
set -euo pipefail

export TZ=UTC
declare -r WORKDIR="${1:-./work}"
declare -r IMAGE_VERSION="${2:-2026.11.11}"
declare -r BUILDDIR="${WORKDIR}/build"
declare -r OUTPUTDIR="${WORKDIR}/output"
SOURCE_DATE_EPOCH=$(date -u -d "$(awk -F. '{print $1"-"$2"-"$3}' <<< "${IMAGE_VERSION}")" +"%s")
export SOURCE_DATE_EPOCH

declare -ra PACKAGES=(
    base
    sudo
    fish
    cachyos-keyring
    cachyos-mirrorlist
    cachyos-v3-mirrorlist
    cachyos-settings
    cachyos-fish-config
    ananicy-cpp
    cachyos-ananicy-rules
    mesa
    vulkan-dzn
    wayland
    wayland-protocols
    hyprland
    cosmic-session
    quickshell
    ghostty
    alacritty
    neovim
)

mkdir -vp "${BUILDDIR}/var/lib/pacman" "${OUTPUTDIR}"
cp -a rootfs/* "${BUILDDIR}/"
ln -sf /usr/lib/os-release "${BUILDDIR}/etc/os-release"

fakechroot -- fakeroot -- pacman -Sy --disable-sandbox-filesystem \
    -r "${BUILDDIR}" --noconfirm --dbpath "${BUILDDIR}/var/lib/pacman" \
    --config rootfs/etc/pacman.conf --noscriptlet "${PACKAGES[@]}"

# Mask Hyper-V conflicting TTY & network units (microsoft/WSL#13595)
ln -sf /dev/null "${BUILDDIR}/etc/systemd/system/getty@.service"
ln -sf /dev/null "${BUILDDIR}/etc/systemd/system/serial-getty@.service"
for unit in console-getty systemd-firstboot systemd-networkd systemd-resolved tmp.mount; do
    ln -sf /dev/null "${BUILDDIR}/etc/systemd/system/${unit}.service" || true
done

chmod 0755 "${BUILDDIR}/usr/lib/wsl/first-setup-aphrody-omarchy.sh"
chmod 0440 "${BUILDDIR}/etc/sudoers.d/10-wheel"
rm -rf "${BUILDDIR}/etc/pacman.d/gnupg/"*

fakeroot -- tar --numeric-owner --xattrs --acls \
    --mtime="@${SOURCE_DATE_EPOCH}" --clamp-mtime --sort=name \
    -C "${BUILDDIR}" -cf "${OUTPUTDIR}/aphrody-omarchy-${IMAGE_VERSION}.tar" .

xz -T0 -9 "${OUTPUTDIR}/aphrody-omarchy-${IMAGE_VERSION}.tar"
mv -v "${OUTPUTDIR}/aphrody-omarchy-${IMAGE_VERSION}.tar.xz" "${OUTPUTDIR}/aphrody-omarchy-${IMAGE_VERSION}.wsl"
sha256sum "${OUTPUTDIR}/aphrody-omarchy-${IMAGE_VERSION}.wsl" > "${OUTPUTDIR}/aphrody-omarchy-${IMAGE_VERSION}.wsl.SHA256"
