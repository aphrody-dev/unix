use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PackageEngineError {
    #[error("Package engine I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PacmanRepoSpec {
    pub name: &'static str,
    pub tier: &'static str,
    pub upstream_url: &'static str,
    pub mirror_include: &'static str,
    pub sig_level: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelPatchDescriptor {
    pub order: usize,
    pub kernel_series: String,
    pub relative_path: String,
    pub subsystem: String,
    pub summary: String,
    pub present_on_disk: bool,
    pub byte_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchGitLabPackageMapping {
    pub pkgbase: String,
    pub arch_gitlab_url: String,
    pub cachyos_pkgbuild_override: Option<String>,
    pub omarchy_pkgbuild_override: Option<String>,
    pub chroot_build_cmd: String,
    pub target_repo: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnifiedRepoManifest {
    pub distro_id: String,
    pub kernel_target: String,
    pub arch_tier: String,
    pub repositories: Vec<PacmanRepoSpec>,
    pub cachyos_pkgbuilds_count: usize,
    pub omarchy_pkgbuilds_count: usize,
    pub core_packages: Vec<ArchGitLabPackageMapping>,
    pub devtools_chroot_workflow: Vec<String>,
}

pub struct SovereignPackageEngine {
    pub cachyos_patches_root: PathBuf,
    pub cachyos_pkgbuilds_root: PathBuf,
    pub omarchy_pkgbuilds_root: PathBuf,
    pub archlinux_root: PathBuf,
    pub linux_tree_root: PathBuf,
}

impl Default for SovereignPackageEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SovereignPackageEngine {
    pub fn new() -> Self {
        Self {
            cachyos_patches_root: PathBuf::from(r"C:\src\third_party\cachyos\kernel-patches"),
            cachyos_pkgbuilds_root: PathBuf::from(r"C:\src\third_party\cachyos\CachyOS-PKGBUILDS"),
            omarchy_pkgbuilds_root: PathBuf::from(r"C:\src\third_party\omacom\omarchy-pkgs\pkgbuilds"),
            archlinux_root: PathBuf::from(r"C:\src\third_party\archlinux"),
            linux_tree_root: PathBuf::from(r"C:\src\third_party\linux"),
        }
    }

    pub fn repositories() -> Vec<PacmanRepoSpec> {
        vec![
            PacmanRepoSpec {
                name: "cachyos-v3",
                tier: "x86-64-v3 (AVX2/FMA3/BMI2)",
                upstream_url: "https://github.com/CachyOS/CachyOS-PKGBUILDS",
                mirror_include: "Include = /etc/pacman.d/cachyos-v3-mirrorlist",
                sig_level: "Required DatabaseOptional",
            },
            PacmanRepoSpec {
                name: "cachyos-core-v3",
                tier: "x86-64-v3 (AVX2/FMA3/BMI2)",
                upstream_url: "https://github.com/CachyOS/CachyOS-PKGBUILDS",
                mirror_include: "Include = /etc/pacman.d/cachyos-v3-mirrorlist",
                sig_level: "Required DatabaseOptional",
            },
            PacmanRepoSpec {
                name: "cachyos-extra-v3",
                tier: "x86-64-v3 (AVX2/FMA3/BMI2)",
                upstream_url: "https://github.com/CachyOS/CachyOS-PKGBUILDS",
                mirror_include: "Include = /etc/pacman.d/cachyos-v3-mirrorlist",
                sig_level: "Required DatabaseOptional",
            },
            PacmanRepoSpec {
                name: "cachyos",
                tier: "x86-64-generic",
                upstream_url: "https://github.com/CachyOS/CachyOS-PKGBUILDS",
                mirror_include: "Include = /etc/pacman.d/cachyos-mirrorlist",
                sig_level: "Required DatabaseOptional",
            },
            PacmanRepoSpec {
                name: "omarchy",
                tier: "x86-64-omarchy-quattro",
                upstream_url: "https://github.com/omacom/omarchy-pkgs",
                mirror_include: "Server = https://pkgs.omarchy.org/stable/$arch",
                sig_level: "Optional TrustAll",
            },
            PacmanRepoSpec {
                name: "core",
                tier: "archlinux-gitlab",
                upstream_url: "https://gitlab.archlinux.org/archlinux/packaging/packages",
                mirror_include: "Include = /etc/pacman.d/mirrorlist",
                sig_level: "Required DatabaseOptional",
            },
            PacmanRepoSpec {
                name: "extra",
                tier: "archlinux-gitlab",
                upstream_url: "https://gitlab.archlinux.org/archlinux/packaging/packages",
                mirror_include: "Include = /etc/pacman.d/mirrorlist",
                sig_level: "Required DatabaseOptional",
            },
            PacmanRepoSpec {
                name: "multilib",
                tier: "archlinux-gitlab",
                upstream_url: "https://gitlab.archlinux.org/archlinux/packaging/packages",
                mirror_include: "Include = /etc/pacman.d/mirrorlist",
                sig_level: "Required DatabaseOptional",
            },
        ]
    }

    pub fn list_kernel_patches(&self, kernel_series: &str) -> Vec<KernelPatchDescriptor> {
        let series_dir = self.cachyos_patches_root.join(kernel_series);
        let canonical_patches = [
            (
                "sched/0001-bore-cachy.patch",
                "sched",
                "Burst-Oriented Response Enhancer (BORE) interactive CPU scheduler for sub-ms frame pacing",
            ),
            (
                "sched/0001-prjc-cachy.patch",
                "sched",
                "Project C /BMQ alternative low-latency scheduler queue integration for Linux 7.4",
            ),
            (
                "misc/dkms-clang.patch",
                "kbuild",
                "Out-of-tree DKMS module compilation compatibility under Clang/LLVM ThinLTO",
            ),
            (
                "misc/nvidia/0005-Fix-build-on-Linux-7.4.patch",
                "gpu-nvidia",
                "NVIDIA open/proprietary kernel module ABI compatibility fix for Linux 7.4",
            ),
            (
                "misc/0001-aufs-7.4-merge-v20260914.patch",
                "fs",
                "AUFS multi-layered union filesystem merge for Linux 7.4 container/live roots",
            ),
            (
                "misc/0001-acpi-call.patch",
                "acpi",
                "ACPI method call interface for hybrid GPU power control",
            ),
        ];

        canonical_patches
            .iter()
            .enumerate()
            .map(|(idx, (rel, subsys, summary))| {
                let full_path = series_dir.join(rel.replace('/', "\\"));
                let meta = fs::metadata(&full_path).ok();
                KernelPatchDescriptor {
                    order: idx + 1,
                    kernel_series: kernel_series.to_string(),
                    relative_path: (*rel).to_string(),
                    subsystem: (*subsys).to_string(),
                    summary: (*summary).to_string(),
                    present_on_disk: meta.is_some(),
                    byte_size: meta.map(|m| m.len()).unwrap_or(0),
                }
            })
            .collect()
    }

    pub fn generate_wsl2_kernel_config_fragment(&self) -> String {
        let base_cfg_path = self
            .linux_tree_root
            .join(r"arch\x86\configs\cachyos_sm89_wsl2_baremetal.config");
        let base_content = fs::read_to_string(&base_cfg_path).unwrap_or_else(|_| {
            "# Base CachyOS sm_89 WSL2 fragment\nCONFIG_RUST=y\nCONFIG_SCHED_CLASS_EXT=y\nCONFIG_SCHED_BORE=y\nCONFIG_NTSYNC=y\nCONFIG_DXGKRNL=y\n".to_string()
        });

        format!(
            "{base}\n\
             # --- Aphrody Omarchy + WSL2 Native Zero-Overhead Extensions (Linux 7.4) ---\n\
             CONFIG_HYPERV_VSOCKETS=y\n\
             CONFIG_VSOCKETS=y\n\
             CONFIG_VIRTIO=y\n\
             CONFIG_VIRTIO_PCI=y\n\
             CONFIG_VIRTIO_FS=y\n\
             CONFIG_FUSE_FS=y\n\
             CONFIG_FUSE_PASSTHROUGH=y\n\
             CONFIG_PLAN9CAPS=y\n\
             CONFIG_NET_9P=y\n\
             CONFIG_NET_9P_VIRTIO=y\n\
             CONFIG_UDMABUF=y\n\
             CONFIG_DMABUF_HEAPS=y\n\
             CONFIG_DMABUF_HEAPS_SYSTEM=y\n\
             CONFIG_SYNC_FILE=y\n\
             CONFIG_BPF_SYSCALL=y\n\
             CONFIG_BPF_JIT=y\n",
            base = base_content.trim_end()
        )
    }

    pub fn generate_patch_series_bundle(
        &self,
        kernel_series: &str,
        out_dir: &Path,
    ) -> Result<Vec<PathBuf>, PackageEngineError> {
        fs::create_dir_all(out_dir)?;
        let patches = self.list_kernel_patches(kernel_series);

        let mut series_txt = String::new();
        series_txt.push_str(&format!(
            "# Linux {kernel_series} CachyOS + Omarchy + WSL2 Quilt/Git Patch Series\n"
        ));
        for p in &patches {
            series_txt.push_str(&format!("{}\n", p.relative_path));
        }
        let series_path = out_dir.join("series");
        fs::write(&series_path, &series_txt)?;

        let mut apply_sh = String::from(
            "#!/usr/bin/env bash\n\
             set -euo pipefail\n\
             KERNEL_SRC=\"${1:-/usr/src/linux-7.4}\"\n\
             PATCH_ROOT=\"${2:-/usr/src/cachyos-kernel-patches/7.4}\"\n\
             echo \"[aphrody-omarchy] Applying CachyOS + WSL2 patch series onto ${KERNEL_SRC}\"\n\
             cd \"${KERNEL_SRC}\"\n",
        );
        for p in &patches {
            apply_sh.push_str(&format!(
                "patch -Np1 -i \"${{PATCH_ROOT}}/{}\"\n",
                p.relative_path
            ));
        }
        apply_sh.push_str(
            "scripts/kconfig/merge_config.sh -m .config \"$(dirname \"$0\")/wsl2-cachyos-omarchy-7.4.config\"\n\
             make LLVM=1 -j\"$(nproc)\" bzImage modules\n",
        );
        let apply_path = out_dir.join("apply-cachyos-7.4-wsl2.sh");
        fs::write(&apply_path, &apply_sh)?;

        let kconfig_path = out_dir.join("wsl2-cachyos-omarchy-7.4.config");
        fs::write(&kconfig_path, self.generate_wsl2_kernel_config_fragment())?;

        let pkgbuild_path = out_dir.join("PKGBUILD.linux-cachyos-omarchy-wsl2");
        fs::write(&pkgbuild_path, self.synthesize_kernel_pkgbuild(kernel_series))?;

        Ok(vec![series_path, apply_path, kconfig_path, pkgbuild_path])
    }

    pub fn synthesize_kernel_pkgbuild(&self, kernel_series: &str) -> String {
        format!(
            "# Maintainer: Aphrody Sovereign OS\n\
             pkgbase=linux-cachyos-omarchy-wsl2\n\
             pkgname=(\"${{pkgbase}}\" \"${{pkgbase}}-headers\")\n\
             pkgver={kernel_series}.0.rc2\n\
             pkgrel=1\n\
             pkgdesc='Linux {kernel_series} kernel with CachyOS BORE/sched-ext, NVIDIA sm_89, /dev/ntsync, and WSL2 /dev/dxg Direct3D 12 zero-copy bridge'\n\
             arch=(x86_64_v3 x86_64)\n\
             url='https://gitlab.archlinux.org/archlinux/packaging/packages/linux'\n\
             license=(GPL-2.0-only)\n\
             makedepends=(bc cpio gettext libelf pahole perl python rust rust-bindgen rust-src tar xz zstd clang llvm lld)\n\
             options=(!debug !strip)\n\
             source=(\n\
             \x20 '0001-bore-cachy.patch'\n\
             \x20 'dkms-clang.patch'\n\
             \x20 '0005-Fix-build-on-Linux-7.4.patch'\n\
             \x20 'wsl2-cachyos-omarchy-7.4.config'\n\
             )\n\
             sha256sums=('SKIP' 'SKIP' 'SKIP' 'SKIP')\n\
             \n\
             build() {{\n\
             \x20 make LLVM=1 -C \"${{srcdir}}/linux\" -j\"$(nproc)\" bzImage modules\n\
             }}\n",
            kernel_series = kernel_series
        )
    }

    pub fn count_dir_entries(dir: &Path) -> usize {
        if !dir.is_dir() {
            return 0;
        }
        fs::read_dir(dir)
            .map(|rd| {
                rd.flatten()
                    .filter(|e| {
                        e.path().is_dir()
                            && !e
                                .file_name()
                                .to_string_lossy()
                                .starts_with('.')
                    })
                    .count()
            })
            .unwrap_or(0)
    }

    pub fn generate_arch_gitlab_manifest(&self) -> UnifiedRepoManifest {
        let cachyos_count = Self::count_dir_entries(&self.cachyos_pkgbuilds_root);
        let omarchy_count = Self::count_dir_entries(&self.omarchy_pkgbuilds_root);

        let core_specs = [
            (
                "linux",
                Some("linux-cachyos"),
                None,
                "cachyos-core-v3",
            ),
            (
                "pacman",
                Some("pacman"),
                None,
                "cachyos-core-v3",
            ),
            (
                "devtools",
                Some("devtools"),
                None,
                "cachyos-extra-v3",
            ),
            (
                "mkinitcpio",
                None,
                None,
                "core",
            ),
            (
                "archinstall",
                None,
                None,
                "extra",
            ),
            (
                "hyprland",
                Some("cachyos-hyprland-settings"),
                Some("hyprland"),
                "omarchy",
            ),
            (
                "cosmic-session",
                None,
                None,
                "extra",
            ),
            (
                "mesa",
                Some("mesa"),
                None,
                "cachyos-v3",
            ),
            (
                "quickshell",
                Some("quickshell-git"),
                Some("omarchy-shell"),
                "omarchy",
            ),
            (
                "aether",
                None,
                Some("aether"),
                "omarchy",
            ),
            (
                "ghostty",
                None,
                Some("ghostty"),
                "omarchy",
            ),
            (
                "scx-scheds",
                Some("scx"),
                None,
                "cachyos-v3",
            ),
            (
                "ananicy-cpp",
                Some("cachyos-ananicy-rules"),
                None,
                "cachyos-v3",
            ),
            (
                "wine",
                Some("wine-cachyos"),
                None,
                "cachyos-v3",
            ),
        ];

        let core_packages = core_specs
            .into_iter()
            .map(|(pkg, cachy, oma, repo)| ArchGitLabPackageMapping {
                pkgbase: pkg.to_string(),
                arch_gitlab_url: format!(
                    "https://gitlab.archlinux.org/archlinux/packaging/packages/{}",
                    pkg
                ),
                cachyos_pkgbuild_override: cachy.map(|s| format!("CachyOS-PKGBUILDS/{}", s)),
                omarchy_pkgbuild_override: oma.map(|s| format!("omarchy-pkgs/pkgbuilds/{}", s)),
                chroot_build_cmd: format!("pkgctl build --repo {} --arch x86_64_v3 {}", repo, pkg),
                target_repo: repo.to_string(),
            })
            .collect();

        let devtools_chroot_workflow = vec![
            "pkgctl repo clone --protocol=https linux hyprland cosmic-session pacman devtools"
                .to_string(),
            "cachy-chroot /var/lib/archbuild/cachyos-v3-x86_64/root pacman -Syu --noconfirm"
                .to_string(),
            "extra-x86_64-build -r /var/lib/archbuild/cachyos-v3 -- -m -C /etc/makepkg-raptorlake.conf"
                .to_string(),
            "repo-add -s -v /srv/repo/aphrody-omarchy/x86_64_v3/aphrody-omarchy.db.tar.zst *.pkg.tar.zst"
                .to_string(),
        ];

        UnifiedRepoManifest {
            distro_id: "aphrody-omarchy-cachyos-wsl2".to_string(),
            kernel_target: "7.4.0-rc2-cachyos-omarchy-wsl2".to_string(),
            arch_tier: "x86-64-v3 (Raptor Lake AVX2/FMA3/BMI2)".to_string(),
            repositories: Self::repositories(),
            cachyos_pkgbuilds_count: cachyos_count,
            omarchy_pkgbuilds_count: omarchy_count,
            core_packages,
            devtools_chroot_workflow,
        }
    }
}
