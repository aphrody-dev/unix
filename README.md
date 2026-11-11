# Sovereign Unix Distribution: Omarchy + CachyOS + Arch Linux GitLab WSL2 Engine

Production-grade Rust 2024 Unix distribution and native WSL2 runtime fusing:
- **Omarchy (`omacom`)**: Aether theme compiler (23 canonical themes + 2 sovereign themes), 16 built-in Quickshell plugins, and streaming search over the 6,443-source community plugin marketplace.
- **CachyOS (`CachyOS` + `cachyos-wsl`)**: Raptor Lake (`x86-64-v3` AVX2/FMA3/BMI2) `[cachyos-v3]` repositories, Linux `7.4` BORE / `sched-ext` / `dkms-clang` / NVIDIA `sm_89` kernel patch series, `ananicy-cpp`, and Proton-CachyOS `/dev/ntsync` gaming orchestration.
- **Arch Linux GitLab (`gitlab.archlinux.org/archlinux`)**: Unified `devtools` / `pkgctl` clean-chroot PKGBUILD synthesis and reproducible `.wsl` image builder.
- **Native Zero-Overhead WSL2 + D3D12 `/dev/dxg`**: Replaces QEMU/WHPX `TryOmarchy.exe` with native WSL2 Hyper-V VMBus + Direct3D 12 Ultimate `/dev/dxg` GPU-PV + Rust WSLg (`C:\src\crates\wayland`).
- **All-GNU-in-Rust (`uutils`) + Windows NT / GPU SDK**: 73 isolated POSIX utilities plus MSVC 14.51 C++23 and .NET 11/10 WinUI host bridges.

---

## Workspace Layout (`C:\src\crates\unix`)

```
C:\src\crates\unix\
├── Cargo.toml                  # Rust 2024 workspace
├── crates\
│   ├── unix-uutils\            # 73 All-GNU-in-Rust utilities & isolated shim installer
│   ├── unix-cachyos\           # chwd CPU/GPU hardware probe, pacman/makepkg, Proton/Wine ntsync
│   ├── unix-omarchy\           # crate `aphrody-omarchy`: Aether themes, plugins, patches, WSL2 builder
│   └── unix-distro\            # CLIs: `unix-distro.exe` and `omarchy-wsl.exe`
├── wsl-distro\                 # Generated native WSL2 bundle (.wslconfig, rootfs, Hyprland/COSMIC, patches)
├── bin\                        # Isolated userland utility shims (never touches C:\Windows\System32)
└── winnt-sdk\                  # MSVC C++23 `native_nt_gpu.dll` + .NET 11/10 WinUI Host
```

---

## Crate `aphrody-omarchy` (`crates/unix-omarchy`)

1. **Aether-in-Rust Theme Compiler (`themes.rs`)**:
   - Parses upstream Omarchy `colors.toml` palettes from `C:\src\third_party\omacom\omarchy\themes\*\colors.toml` and embeds all 23 canonical Omarchy themes plus 2 sovereign presets (`aphrody-m3-dark`, `cachyos-emerald`) for standalone zero-dependency execution.
   - Compiles any `ColorsToml` simultaneously into 6 target formats:
     1. `hyprland-colors.conf` (`col.active_border`, `col.inactive_border`, shadow, `groupbar`)
     2. `cosmic-theme.ron` (COSMIC Desktop RON theme)
     3. `bar.css` (Waybar / Quickshell CSS)
     4. `terminal.toml` (Alacritty / Rio / Ghostty TOML)
     5. `aether-theme.lua` (Neovim `omacom/aether.nvim` Lua spec)
     6. `theme.css` (CSS Custom Properties for Web/TUI preview)
2. **Plugin & Marketplace Engine (`plugins.rs`)**:
   - Catalogs all 16 built-in Omarchy shell plugins (`agents`, `background`, `bar`, `clipboard`, `dev-gallery`, `dock`, `emojis`, `image-picker`, `lock`, `menu`, `notifications`, `osd`, `panels`, `polkit`, `reminders`, `services`).
   - Streams `C:\src\third_party\omacom\omarchy-plugin-marketplace\registry.json` (6,443 community sources) with sub-second filtering and scaffolds new Quickshell plugins (`manifest.json` + QML entry point).
3. **Arch GitLab + CachyOS + Omarchy Package & Patch Engine (`packages.rs`)**:
   - Synthesizes unified repository manifests across `[cachyos-v3]`, `[cachyos-core-v3]`, `[cachyos-extra-v3]`, `[omarchy]`, and Arch `[core]`/`[extra]`/`[multilib]`.
   - Verifies and emits the Linux `7.4` kernel patch series (`0001-bore-cachy.patch`, `0001-prjc-cachy.patch`, `dkms-clang.patch`, `nvidia/0005-Fix-build-on-Linux-7.4.patch`, `0001-aufs-7.4-merge-v20260914.patch`, `0001-acpi-call.patch`) alongside the WSL2 `/dev/dxg` + `hv_sock` + `virtio_fs` + `/dev/ntsync` kconfig fragment.
4. **Native WSL2 Distro Builder & Launcher (`wsl.rs`)**:
   - Emits `.wslconfig` (`memory=24GB`, `processors=24`, `networkingMode=mirrored`, `dnsTunneling=true`, `autoProxy=true`, `guiApplications=true`, custom `bzImage-7.4-cachyos-omarchy`), `rootfs/etc/wsl.conf` (`systemd=true`), `rootfs/etc/wsl-distribution.conf` (`oobe` command `/usr/lib/wsl/first-setup-aphrody-omarchy.sh`, default user `aphrody`), `zram-generator.conf`, Hyper-V `getty@.service` unit masks, and reproducible `build-aphrody-omarchy-wsl.sh`.
   - Orchestrates `wsl.exe --install --from-file` / `wsl.exe --import` with automatic dry-run safety and live `wsl --list` inspection.

---

## CLI Usage (`omarchy-wsl` & `unix-distro`)

Installed in `C:\Users\aphro\.local\bin\omarchy-wsl.exe` and `C:\Users\aphro\.local\bin\unix-distro.exe`:

```powershell
# List all 25 Omarchy + Sovereign Aether themes and export Tokyo Night
omarchy-wsl themes list
omarchy-wsl themes export tokyo-night --out C:\src\crates\unix\wsl-distro\theme-tokyo-night

# Search built-in (16) and community marketplace (6,443) plugins
omarchy-wsl plugins list --query hypr --limit 10

# Inspect and generate Linux 7.4 CachyOS + WSL2 kernel patch series
omarchy-wsl patches list --kernel 7.4
omarchy-wsl patches generate-series --out C:\src\crates\unix\wsl-distro\patches

# Generate full native WSL2 distribution bundle
omarchy-wsl wsl generate-bundle --out C:\src\crates\unix\wsl-distro

# Generate Arch GitLab + CachyOS v3 + Omarchy sync manifest
omarchy-wsl arch gitlab-sync-manifest --out C:\src\crates\unix\wsl-distro\arch-gitlab-manifest.json

# Run full ecosystem diagnostics
omarchy-wsl doctor
unix-distro status
```

---

## Verification

```powershell
cargo test --workspace --manifest-path C:\src\crates\unix\Cargo.toml
```
