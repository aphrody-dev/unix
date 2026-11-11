use crate::packages::SovereignPackageEngine;
use crate::themes::{AetherThemeRegistry, ThemeError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;
use unix_cachyos::{CachyOsConfigGenerator, SystemHardwareProfile};

#[derive(Error, Debug)]
pub enum WslDistroError {
    #[error("WSL bundle I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Theme error: {0}")]
    Theme(#[from] ThemeError),
    #[error("Protected host OS directory rejected: {0}")]
    ProtectedDirectory(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WslBundleManifest {
    pub distro_name: String,
    pub default_user: String,
    pub default_theme: String,
    pub kernel_image_path: String,
    pub out_dir: String,
    pub generated_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WslInstallPlan {
    pub distro_name: String,
    pub image_path: String,
    pub install_dir: String,
    pub image_exists_on_disk: bool,
    pub dry_run: bool,
    pub wsl_exe_command: String,
    pub fallback_import_command: String,
    pub existing_wsl_distros: Vec<String>,
}

pub struct SovereignWslBuilder {
    pub distro_name: String,
    pub default_user: String,
    pub default_theme: String,
    pub kernel_win_path: String,
    pub memory_gb: u32,
    pub processors: u32,
}

impl Default for SovereignWslBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SovereignWslBuilder {
    pub fn new() -> Self {
        Self {
            distro_name: "aphrody-omarchy".to_string(),
            default_user: "aphrody".to_string(),
            default_theme: "tokyo-night".to_string(),
            kernel_win_path: r"C:\\src\\crates\\unix\\wsl\\bzImage-7.4-cachyos-omarchy"
                .to_string(),
            memory_gb: 24,
            processors: 24,
        }
    }

    pub fn generate_wslconfig(&self) -> String {
        format!(
            "# %USERPROFILE%\\.wslconfig - Sovereign Native WSL2 Zero-Overhead Profile\n\
             # Replaces QEMU/WHPX TryOmarchy.exe with direct Hyper-V VMBus + /dev/dxg D3D12 GPU-PV\n\
             [wsl2]\n\
             kernel={kernel}\n\
             memory={mem}GB\n\
             processors={procs}\n\
             networkingMode=mirrored\n\
             dnsTunneling=true\n\
             autoProxy=true\n\
             firewall=true\n\
             guiApplications=true\n\
             nestedVirtualization=true\n\
             pageReporting=true\n\
             kernelCommandLine=cgroup_no_v1=all systemd.unified_cgroup_hierarchy=1 mitigations=off nowatchdog\n\
             \n\
             [experimental]\n\
             autoMemoryReclaim=gradual\n\
             sparseVhd=true\n\
             hostAddressLoopback=true\n",
            kernel = self.kernel_win_path,
            mem = self.memory_gb,
            procs = self.processors,
        )
    }

    pub fn generate_rootfs_wsl_conf(&self) -> String {
        format!(
            "# /etc/wsl.conf - Sovereign Omarchy + CachyOS + Arch WSL2 Guest Configuration\n\
             [boot]\n\
             systemd=true\n\
             command=\"/usr/bin/mkdir -p /run/wslg /tmp/.X11-unix\"\n\
             \n\
             [automount]\n\
             enabled=true\n\
             root=/mnt/\n\
             options=\"metadata,umask=22,fmask=11\"\n\
             mountFsTab=true\n\
             \n\
             [interop]\n\
             enabled=true\n\
             appendWindowsPath=true\n\
             \n\
             [network]\n\
             generateHosts=true\n\
             generateResolvConf=true\n\
             hostname={distro}\n\
             \n\
             [user]\n\
             default={user}\n",
            distro = self.distro_name,
            user = self.default_user,
        )
    }

    pub fn generate_wsl_distribution_conf(&self) -> String {
        format!(
            "[oobe]\n\
             command = /usr/lib/wsl/first-setup-aphrody-omarchy.sh\n\
             defaultUid = 1000\n\
             defaultName = {distro}\n\
             \n\
             [shortcut]\n\
             enabled = true\n\
             icon = /usr/lib/wsl/aphrody-omarchy.ico\n\
             \n\
             [windowsterminal]\n\
             enabled = true\n",
            distro = self.distro_name,
        )
    }

    pub fn generate_unified_pacman_conf(&self, profile: &SystemHardwareProfile) -> String {
        format!(
            "# /etc/pacman.conf - Unified CachyOS v3 + Omarchy + Arch Linux GitLab Repositories\n\
             # Target Hardware: {cpu} ({tier}) + {gpu}\n\
             \n\
             [options]\n\
             HoldPkg           = pacman glibc\n\
             Architecture      = auto x86_64_v3 x86_64\n\
             ParallelDownloads = 16\n\
             Color\n\
             ILoveCandy\n\
             CheckSpace\n\
             VerbosePkgLists\n\
             SigLevel          = Required DatabaseOptional\n\
             LocalFileSigLevel = Optional\n\
             \n\
             # 1. CachyOS x86-64-v3 AVX2/FMA3/BMI2 Tier\n\
             [cachyos-v3]\n\
             Include = /etc/pacman.d/cachyos-v3-mirrorlist\n\
             \n\
             [cachyos-core-v3]\n\
             Include = /etc/pacman.d/cachyos-v3-mirrorlist\n\
             \n\
             [cachyos-extra-v3]\n\
             Include = /etc/pacman.d/cachyos-v3-mirrorlist\n\
             \n\
             [cachyos]\n\
             Include = /etc/pacman.d/cachyos-mirrorlist\n\
             \n\
             # 2. Omarchy Quattro Desktop & Shell Repository\n\
             [omarchy]\n\
             SigLevel = Optional TrustAll\n\
             Server = https://pkgs.omarchy.org/stable/$arch\n\
             \n\
             # 3. Arch Linux Canonical Repositories (gitlab.archlinux.org)\n\
             [core]\n\
             Include = /etc/pacman.d/mirrorlist\n\
             \n\
             [extra]\n\
             Include = /etc/pacman.d/mirrorlist\n\
             \n\
             [multilib]\n\
             Include = /etc/pacman.d/mirrorlist\n",
            cpu = profile.cpu.brand,
            tier = profile.cpu.architecture_level,
            gpu = profile.gpu.name,
        )
    }

    pub fn generate_zram_generator_conf() -> &'static str {
        "# /etc/systemd/zram-generator.conf\n\
         # Overrides cachyos-settings zram rules inside WSL2: Hyper-V / WSL2 manages host-backed swap directly.\n"
    }

    pub fn generate_systemd_mask_manifest() -> &'static str {
        "# Systemd unit masks required for clean Hyper-V / WSL2 boot without TTY contention\n\
         # Ref: CachyOS/cachyos-wsl & microsoft/WSL#13595\n\
         console-getty.service\n\
         getty@.service\n\
         serial-getty@.service\n\
         systemd-firstboot.service\n\
         systemd-networkd.service\n\
         systemd-networkd-wait-online.service\n\
         systemd-resolved.service\n\
         systemd-tmpfiles-clean.service\n\
         systemd-tmpfiles-setup.service\n\
         systemd-tmpfiles-setup-dev.service\n\
         systemd-tmpfiles-setup-dev-early.service\n\
         tmp.mount\n"
    }

    pub fn generate_first_setup_script(&self) -> String {
        format!(
            "#!/usr/bin/env bash\n\
             # /usr/lib/wsl/first-setup-aphrody-omarchy.sh\n\
             set -euo pipefail\n\
             \n\
             declare -r DEFAULT_UID=1000\n\
             declare -r DEFAULT_USER=\"{user}\"\n\
             declare -r DEFAULT_GROUPS=\"wheel,video,render\"\n\
             \n\
             echo \"Initializing Sovereign Omarchy + CachyOS + Arch WSL2 Distribution...\"\n\
             pacman-key --init >/dev/null 2>&1 || true\n\
             pacman-key --populate archlinux cachyos >/dev/null 2>&1 || true\n\
             \n\
             if [ -x /usr/lib/wsl/optimize-repos ]; then\n\
             \x20   /usr/lib/wsl/optimize-repos || true\n\
             fi\n\
             \n\
             if ! id -u \"${{DEFAULT_USER}}\" >/dev/null 2>&1; then\n\
             \x20   useradd --create-home --uid \"${{DEFAULT_UID}}\" --groups \"${{DEFAULT_GROUPS}}\" --shell /usr/bin/fish \"${{DEFAULT_USER}}\"\n\
             \x20   passwd -d \"${{DEFAULT_USER}}\" >/dev/null 2>&1 || true\n\
             fi\n\
             \n\
             install -d -m 0700 -o \"${{DEFAULT_UID}}\" -g \"${{DEFAULT_UID}}\" \"/home/${{DEFAULT_USER}}/.config/hypr\"\n\
             install -d -m 0700 -o \"${{DEFAULT_UID}}\" -g \"${{DEFAULT_UID}}\" \"/home/${{DEFAULT_USER}}/.config/cosmic\"\n\
             install -d -m 0700 -o \"${{DEFAULT_UID}}\" -g \"${{DEFAULT_UID}}\" \"/home/${{DEFAULT_USER}}/.config/omarchy/current\"\n\
             \n\
             echo \"D3D12 /dev/dxg + Rust WSLg (Hyprland + COSMIC + Omarchy {theme}) ready.\"\n",
            user = self.default_user,
            theme = self.default_theme,
        )
    }

    pub fn generate_build_image_script(&self) -> String {
        format!(
            "#!/usr/bin/env bash\n\
             # build-aphrody-omarchy-wsl.sh - Reproducible RootFS & .wsl Builder\n\
             # Fuses CachyOS/cachyos-wsl + omacom/omarchy + gitlab.archlinux.org\n\
             set -euo pipefail\n\
             \n\
             export TZ=UTC\n\
             declare -r WORKDIR=\"${{1:-./work}}\"\n\
             declare -r IMAGE_VERSION=\"${{2:-2026.11.11}}\"\n\
             declare -r BUILDDIR=\"${{WORKDIR}}/build\"\n\
             declare -r OUTPUTDIR=\"${{WORKDIR}}/output\"\n\
             SOURCE_DATE_EPOCH=$(date -u -d \"$(awk -F. '{{print $1\"-\"$2\"-\"$3}}' <<< \"${{IMAGE_VERSION}}\")\" +\"%s\")\n\
             export SOURCE_DATE_EPOCH\n\
             \n\
             declare -ra PACKAGES=(\n\
             \x20   base\n\
             \x20   sudo\n\
             \x20   fish\n\
             \x20   cachyos-keyring\n\
             \x20   cachyos-mirrorlist\n\
             \x20   cachyos-v3-mirrorlist\n\
             \x20   cachyos-settings\n\
             \x20   cachyos-fish-config\n\
             \x20   ananicy-cpp\n\
             \x20   cachyos-ananicy-rules\n\
             \x20   mesa\n\
             \x20   vulkan-dzn\n\
             \x20   wayland\n\
             \x20   wayland-protocols\n\
             \x20   hyprland\n\
             \x20   cosmic-session\n\
             \x20   quickshell\n\
             \x20   ghostty\n\
             \x20   alacritty\n\
             \x20   neovim\n\
             )\n\
             \n\
             mkdir -vp \"${{BUILDDIR}}/var/lib/pacman\" \"${{OUTPUTDIR}}\"\n\
             cp -a rootfs/* \"${{BUILDDIR}}/\"\n\
             ln -sf /usr/lib/os-release \"${{BUILDDIR}}/etc/os-release\"\n\
             \n\
             fakechroot -- fakeroot -- pacman -Sy --disable-sandbox-filesystem \\\n\
             \x20   -r \"${{BUILDDIR}}\" --noconfirm --dbpath \"${{BUILDDIR}}/var/lib/pacman\" \\\n\
             \x20   --config rootfs/etc/pacman.conf --noscriptlet \"${{PACKAGES[@]}}\"\n\
             \n\
             # Mask Hyper-V conflicting TTY & network units (microsoft/WSL#13595)\n\
             ln -sf /dev/null \"${{BUILDDIR}}/etc/systemd/system/getty@.service\"\n\
             ln -sf /dev/null \"${{BUILDDIR}}/etc/systemd/system/serial-getty@.service\"\n\
             for unit in console-getty systemd-firstboot systemd-networkd systemd-resolved tmp.mount; do\n\
             \x20   ln -sf /dev/null \"${{BUILDDIR}}/etc/systemd/system/${{unit}}.service\" || true\n\
             done\n\
             \n\
             chmod 0755 \"${{BUILDDIR}}/usr/lib/wsl/first-setup-aphrody-omarchy.sh\"\n\
             chmod 0440 \"${{BUILDDIR}}/etc/sudoers.d/10-wheel\"\n\
             rm -rf \"${{BUILDDIR}}/etc/pacman.d/gnupg/\"*\n\
             \n\
             fakeroot -- tar --numeric-owner --xattrs --acls \\\n\
             \x20   --mtime=\"@${{SOURCE_DATE_EPOCH}}\" --clamp-mtime --sort=name \\\n\
             \x20   -C \"${{BUILDDIR}}\" -cf \"${{OUTPUTDIR}}/{distro}-${{IMAGE_VERSION}}.tar\" .\n\
             \n\
             xz -T0 -9 \"${{OUTPUTDIR}}/{distro}-${{IMAGE_VERSION}}.tar\"\n\
             mv -v \"${{OUTPUTDIR}}/{distro}-${{IMAGE_VERSION}}.tar.xz\" \"${{OUTPUTDIR}}/{distro}-${{IMAGE_VERSION}}.wsl\"\n\
             sha256sum \"${{OUTPUTDIR}}/{distro}-${{IMAGE_VERSION}}.wsl\" > \"${{OUTPUTDIR}}/{distro}-${{IMAGE_VERSION}}.wsl.SHA256\"\n",
            distro = self.distro_name,
        )
    }

    pub fn generate_hyprland_wslg_conf(&self) -> String {
        format!(
            "# ~/.config/hypr/hyprland.conf - Omarchy + Rust WSLg Direct3D 12 (/dev/dxg) Compositor Config\n\
             source = ~/.config/hypr/hyprland-colors.conf\n\
             \n\
             env = GALLIUM_DRIVER,d3d12\n\
             env = MESA_D3D12_DEFAULT_ADAPTER_NAME,NVIDIA GeForce RTX 4070\n\
             env = WGPU_BACKEND,vulkan\n\
             env = XDG_CURRENT_DESKTOP,Hyprland:COSMIC\n\
             env = XDG_SESSION_TYPE,wayland\n\
             env = GDK_BACKEND,wayland\n\
             env = QT_QPA_PLATFORM,wayland\n\
             \n\
             monitor = DP-1, 2560x1440@120, 0x0, 1\n\
             \n\
             general {{\n\
             \x20   gaps_in = 6\n\
             \x20   gaps_out = 12\n\
             \x20   border_size = 2\n\
             \x20   layout = dwindle\n\
             }}\n\
             \n\
             # Omarchy Keyboard-First Workflow Bindings\n\
             bind = SUPER, SPACE, exec, omarchy-menu\n\
             bind = SUPER, K, exec, omarchy-menu-keybindings\n\
             bind = SUPER, RETURN, exec, ghostty\n\
             bind = SUPER, W, killactive,\n\
             bind = SUPER SHIFT, T, exec, omarchy-wsl themes export {theme}\n\
             bind = SUPER, 1, workspace, 1\n\
             bind = SUPER, 2, workspace, 2\n\
             bind = SUPER, 3, workspace, 3\n\
             bind = SUPER, 4, workspace, 4\n\
             bind = SUPER, 5, workspace, 5\n\
             bind = SUPER, 6, workspace, 6\n\
             bind = SUPER, 7, workspace, 7\n\
             bind = SUPER, 8, workspace, 8\n\
             bind = SUPER, 9, workspace, 9\n",
            theme = self.default_theme,
        )
    }

    pub fn emit_bundle(&self, out_dir: &Path) -> Result<WslBundleManifest, WslDistroError> {
        let lower = out_dir.to_string_lossy().to_lowercase();
        if lower.contains("windows\\system32")
            || lower.contains("windows/system32")
            || lower.contains("windows\\syswow64")
        {
            return Err(WslDistroError::ProtectedDirectory(
                out_dir.display().to_string(),
            ));
        }

        let rootfs_etc = out_dir.join("rootfs").join("etc");
        let rootfs_systemd = rootfs_etc.join("systemd");
        let rootfs_sudoers = rootfs_etc.join("sudoers.d");
        let rootfs_wsl_lib = out_dir.join("rootfs").join("usr").join("lib").join("wsl");
        let hypr_dir = out_dir
            .join("rootfs")
            .join("etc")
            .join("skel")
            .join(".config")
            .join("hypr");
        let cosmic_dir = out_dir
            .join("rootfs")
            .join("etc")
            .join("skel")
            .join(".config")
            .join("cosmic");
        let scripts_dir = out_dir.join("scripts");
        let kernel_dir = out_dir.join("kernel");

        for d in [
            out_dir,
            &rootfs_etc,
            &rootfs_systemd,
            &rootfs_sudoers,
            &rootfs_wsl_lib,
            &hypr_dir,
            &cosmic_dir,
            &scripts_dir,
            &kernel_dir,
        ] {
            fs::create_dir_all(d)?;
        }

        let profile = SystemHardwareProfile::probe();
        let pkg_engine = SovereignPackageEngine::new();
        let theme_reg = AetherThemeRegistry::new();
        let theme = theme_reg
            .get(&self.default_theme)
            .or_else(|_| theme_reg.get("tokyo-night"))?;
        let compiled_theme = theme.compile_all();

        let files_to_write: Vec<(PathBuf, String)> = vec![
            (out_dir.join(".wslconfig"), self.generate_wslconfig()),
            (rootfs_etc.join("wsl.conf"), self.generate_rootfs_wsl_conf()),
            (
                rootfs_etc.join("wsl-distribution.conf"),
                self.generate_wsl_distribution_conf(),
            ),
            (
                rootfs_etc.join("pacman.conf"),
                self.generate_unified_pacman_conf(&profile),
            ),
            (
                rootfs_etc.join("makepkg.conf"),
                CachyOsConfigGenerator::generate_makepkg_conf(&profile),
            ),
            (
                rootfs_systemd.join("zram-generator.conf"),
                Self::generate_zram_generator_conf().to_string(),
            ),
            (
                rootfs_systemd.join("masked-wsl-units.list"),
                Self::generate_systemd_mask_manifest().to_string(),
            ),
            (
                rootfs_sudoers.join("10-wheel"),
                "%wheel ALL=(ALL:ALL) NOPASSWD: ALL\n".to_string(),
            ),
            (
                rootfs_wsl_lib.join("first-setup-aphrody-omarchy.sh"),
                self.generate_first_setup_script(),
            ),
            (
                scripts_dir.join("build-aphrody-omarchy-wsl.sh"),
                self.generate_build_image_script(),
            ),
            (
                scripts_dir.join("build-image.sh"),
                self.generate_build_image_script(),
            ),
            (
                hypr_dir.join("hyprland.conf"),
                self.generate_hyprland_wslg_conf(),
            ),
            (
                hypr_dir.join("hyprland-colors.conf"),
                compiled_theme.hyprland_colors_conf,
            ),
            (
                cosmic_dir.join("cosmic-theme.ron"),
                compiled_theme.cosmic_theme_ron,
            ),
            (
                kernel_dir.join("wsl2-cachyos-omarchy-7.4.config"),
                pkg_engine.generate_wsl2_kernel_config_fragment(),
            ),
        ];

        let mut generated_files = Vec::with_capacity(files_to_write.len());
        for (path, content) in files_to_write {
            fs::write(&path, content)?;
            generated_files.push(path.display().to_string());
        }

        Ok(WslBundleManifest {
            distro_name: self.distro_name.clone(),
            default_user: self.default_user.clone(),
            default_theme: self.default_theme.clone(),
            kernel_image_path: self.kernel_win_path.clone(),
            out_dir: out_dir.display().to_string(),
            generated_files,
        })
    }

    pub fn inspect_live_wsl_distros() -> Vec<String> {
        let output = Command::new("wsl.exe")
            .args(["--list", "--quiet"])
            .output();

        let Ok(out) = output else {
            return Vec::new();
        };
        if !out.status.success() {
            return Vec::new();
        }

        // wsl.exe --list emits UTF-16LE on Windows
        let raw = &out.stdout;
        let decoded = if raw.chunks_exact(2).any(|c| c[1] == 0) && raw.len() >= 2 {
            let u16_words: Vec<u16> = raw
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            String::from_utf16_lossy(&u16_words)
        } else {
            String::from_utf8_lossy(raw).into_owned()
        };

        decoded
            .lines()
            .map(|l| l.trim_matches('\0').trim().to_string())
            .filter(|l| !l.is_empty())
            .collect()
    }

    pub fn plan_or_launch_wsl(
        &self,
        wsl_image_file: &Path,
        install_dir: &Path,
        execute: bool,
    ) -> Result<WslInstallPlan, WslDistroError> {
        let lower = install_dir.to_string_lossy().to_lowercase();
        if lower.contains("windows\\system32") || lower.contains("windows/system32") {
            return Err(WslDistroError::ProtectedDirectory(
                install_dir.display().to_string(),
            ));
        }

        let exists = wsl_image_file.is_file();
        let dry_run = !execute || !exists;
        let wsl_cmd = format!(
            "wsl.exe --install --from-file \"{}\" --name {}",
            wsl_image_file.display(),
            self.distro_name
        );
        let import_cmd = format!(
            "wsl.exe --import {} \"{}\" \"{}\" --version 2",
            self.distro_name,
            install_dir.display(),
            wsl_image_file.display()
        );

        if execute && exists {
            fs::create_dir_all(install_dir)?;
            let _ = Command::new("wsl.exe")
                .arg("--install")
                .arg("--from-file")
                .arg(wsl_image_file)
                .arg("--name")
                .arg(&self.distro_name)
                .status();
        }

        Ok(WslInstallPlan {
            distro_name: self.distro_name.clone(),
            image_path: wsl_image_file.display().to_string(),
            install_dir: install_dir.display().to_string(),
            image_exists_on_disk: exists,
            dry_run,
            wsl_exe_command: wsl_cmd,
            fallback_import_command: import_cmd,
            existing_wsl_distros: Self::inspect_live_wsl_distros(),
        })
    }
}
