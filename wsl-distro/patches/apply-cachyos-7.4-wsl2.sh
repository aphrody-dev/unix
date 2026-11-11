#!/usr/bin/env bash
set -euo pipefail
KERNEL_SRC="${1:-/usr/src/linux-7.4}"
PATCH_ROOT="${2:-/usr/src/cachyos-kernel-patches/7.4}"
echo "[aphrody-omarchy] Applying CachyOS + WSL2 patch series onto ${KERNEL_SRC}"
cd "${KERNEL_SRC}"
patch -Np1 -i "${PATCH_ROOT}/sched/0001-bore-cachy.patch"
patch -Np1 -i "${PATCH_ROOT}/sched/0001-prjc-cachy.patch"
patch -Np1 -i "${PATCH_ROOT}/misc/dkms-clang.patch"
patch -Np1 -i "${PATCH_ROOT}/misc/nvidia/0005-Fix-build-on-Linux-7.4.patch"
patch -Np1 -i "${PATCH_ROOT}/misc/0001-aufs-7.4-merge-v20260914.patch"
patch -Np1 -i "${PATCH_ROOT}/misc/0001-acpi-call.patch"
scripts/kconfig/merge_config.sh -m .config "$(dirname "$0")/wsl2-cachyos-omarchy-7.4.config"
make LLVM=1 -j"$(nproc)" bzImage modules
