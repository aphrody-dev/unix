//! Sovereign POSIX, Linux 7.4+, Non-C Kernels & 2026 Distro Discovery Engine
//! Merged into `unix-cachyos` with live local `third_party/{linux,cachyos,omacom}` git tree probing.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelVariant2026 {
    pub name: String,
    pub upstream_basis: String,
    pub primary_patches: Vec<String>,
    pub target_usecase: String,
    pub scheduler_default: String,
    pub memory_allocator: String,
    pub io_uring_status: String,
    pub rust_readiness: String,
    pub libc_target: String,
    pub unique_features: Vec<String>,
    pub local_checkout_head: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonCKernel {
    pub name: String,
    pub language: String,
    pub architecture_type: String,
    pub posix_compliance_level: String,
    pub repo_url: String,
    pub status: String,
    pub key_innovations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwesomeListCategory {
    pub category: String,
    pub lists: Vec<AwesomeListEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwesomeListEntry {
    pub title: String,
    pub repo_url: String,
    pub topic: String,
    pub items_count_estimate: u32,
    pub highlighted_innovations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosixDiscoveryCatalog {
    pub engine: String,
    pub local_linux_head: Option<String>,
    pub local_cachyos_patches_count: usize,
    pub kernel_variants_2026: Vec<KernelVariant2026>,
    pub rewritten_kernels_non_c: Vec<NonCKernel>,
    pub awesome_ecosystem_indexes: Vec<AwesomeListCategory>,
    pub new_2026_concepts_and_workflows: Vec<String>,
}

/// Read the Git HEAD commit SHA of a local repository checkout or gitfile submodule without spawning subprocesses.
pub fn probe_git_head_sha(repo_dir: &Path) -> Option<String> {
    let dot_git = repo_dir.join(".git");
    let git_dir = if dot_git.is_dir() {
        dot_git
    } else if dot_git.is_file() {
        let content = fs::read_to_string(&dot_git).ok()?;
        let rel = content.strip_prefix("gitdir:")?.trim();
        let candidate = Path::new(rel);
        if candidate.is_absolute() {
            candidate.to_path_buf()
        } else {
            repo_dir.join(candidate)
        }
    } else {
        return None;
    };

    let head = fs::read_to_string(git_dir.join("HEAD")).ok()?;
    let trimmed = head.trim();
    if let Some(ref_path) = trimmed.strip_prefix("ref:") {
        let ref_rel = ref_path.trim();
        if let Ok(sha) = fs::read_to_string(git_dir.join(ref_rel)) {
            return Some(sha.trim().to_string());
        }
        if let Ok(packed) = fs::read_to_string(git_dir.join("packed-refs")) {
            for line in packed.lines() {
                if let Some((sha, name)) = line.split_once(' ')
                    && name.trim() == ref_rel
                {
                    return Some(sha.trim().to_string());
                }
            }
        }
        None
    } else if trimmed.len() >= 40 {
        Some(trimmed[..40].to_string())
    } else {
        None
    }
}

/// Count `.patch` files under a CachyOS kernel-patches checkout if present.
pub fn count_local_patch_files(dir: &Path) -> usize {
    let mut count = 0usize;
    let Ok(entries) = fs::read_dir(dir) else {
        return 0;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            count += count_local_patch_files(&path);
        } else if path.extension().and_then(|e| e.to_str()) == Some("patch") {
            count += 1;
        }
    }
    count
}

/// Build the sovereign 2026 POSIX, Linux 7.4+, and Non-C Kernel discovery catalog grounded in `monorepo_root`.
pub fn build_posix_discovery_catalog(monorepo_root: &Path) -> PosixDiscoveryCatalog {
    let linux_dir = monorepo_root.join("third_party").join("linux");
    let cachy_patches_dir = monorepo_root
        .join("third_party")
        .join("cachyos")
        .join("kernel-patches");
    let local_linux_head = probe_git_head_sha(&linux_dir);
    let local_cachyos_patches_count = count_local_patch_files(&cachy_patches_dir);

    let kernel_variants = vec![
        KernelVariant2026 {
            name: "Upstream Mainline / Stable (Linux 7.4-rc2 / 7.3.y)".to_string(),
            upstream_basis: "git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git".to_string(),
            primary_patches: vec![
                "PREEMPT_RT mainline merge".to_string(),
                "EEVDF scheduler".to_string(),
                "Rust for Linux DRM/NVMe/Binder drivers".to_string(),
                "io_uring fixed-buffer zero-copy".to_string(),
            ],
            target_usecase: "Canonical reference for all Linux systems".to_string(),
            scheduler_default: "EEVDF".to_string(),
            memory_allocator: "SLUB".to_string(),
            io_uring_status: "Wire-speed support + SQPOLL".to_string(),
            rust_readiness: "Production in DRM, Binder, NVMe, Network PHY".to_string(),
            libc_target: "glibc / musl".to_string(),
            unique_features: vec![
                "Sacred userspace ABI stability guarantee".to_string(),
                "Strict merge window and -rc release cadence".to_string(),
            ],
            local_checkout_head: local_linux_head.clone(),
        },
        KernelVariant2026 {
            name: "Arch Linux / CachyOS Kernel (BORE + sched-ext)".to_string(),
            upstream_basis: "github.com/CachyOS/linux-cachyos".to_string(),
            primary_patches: vec![
                "BORE (Burst-Oriented Response Enhancer)".to_string(),
                "x86-64-v3 / raptorlake ThinLTO optimization".to_string(),
                "ntsync Windows NT synchronization primitive".to_string(),
            ],
            target_usecase: "Low-latency workstation, Wayland desktop, and Proton gaming".to_string(),
            scheduler_default: "EEVDF-BORE / scx_lavd".to_string(),
            memory_allocator: "SLUB + MGLRU".to_string(),
            io_uring_status: "Unrestricted high-throughput enabled".to_string(),
            rust_readiness: "CONFIG_RUST=y enabled".to_string(),
            libc_target: "glibc (x86-64-v3)".to_string(),
            unique_features: vec![
                format!("Local CachyOS patch corpus indexed ({local_cachyos_patches_count} .patch files)"),
                "sched-ext eBPF user-space schedulers (scx_lavd, scx_rusty)".to_string(),
            ],
            local_checkout_head: probe_git_head_sha(&cachy_patches_dir),
        },
        KernelVariant2026 {
            name: "WSL2 Linux Kernel (Hyper-V + DirectX 12 dxgkrnl)".to_string(),
            upstream_basis: "github.com/microsoft/WSL2-Linux-Kernel".to_string(),
            primary_patches: vec![
                "Hyper-V VMBus & /dev/dxg GPU paravirtualization".to_string(),
                "Virtio-FS & 9P host filesystem drivers".to_string(),
                "Mirrored Mode networking".to_string(),
            ],
            target_usecase: "Hybrid Windows 11 NT + Linux 7.4 userspace with CUDA sm_89".to_string(),
            scheduler_default: "EEVDF Hyper-V tuned".to_string(),
            memory_allocator: "SLUB + Hyper-V dynamic ballooning".to_string(),
            io_uring_status: "Enabled for guest VFS and socket rings".to_string(),
            rust_readiness: "Clang/LLVM kernel build compatible".to_string(),
            libc_target: "glibc / musl".to_string(),
            unique_features: vec![
                "Direct CUDA 13.4 sm_89 and D3D12 compute passthrough".to_string(),
                "Native Wayland compositor bridging via crates/wayland".to_string(),
            ],
            local_checkout_head: None,
        },
        KernelVariant2026 {
            name: "Android Common Kernel (AOSP GKI 6.12+)".to_string(),
            upstream_basis: "android.googlesource.com/kernel/common".to_string(),
            primary_patches: vec![
                "Generic Kernel Image (GKI) KMI ABI boundary".to_string(),
                "Rust Binder IPC driver".to_string(),
                "Energy Aware Scheduling (EAS)".to_string(),
            ],
            target_usecase: "Android mobile and embedded AOSP targets".to_string(),
            scheduler_default: "EEVDF + EAS".to_string(),
            memory_allocator: "SLUB + zram".to_string(),
            io_uring_status: "Sandboxed via SELinux policy".to_string(),
            rust_readiness: "First-class Kleaf + Clang ThinLTO + Rust".to_string(),
            libc_target: "Bionic libc".to_string(),
            unique_features: vec![
                "Single GKI kernel image across SoC vendors".to_string(),
                "Strict kCFI control-flow integrity".to_string(),
            ],
            local_checkout_head: None,
        },
        KernelVariant2026 {
            name: "NixOS Reproducible Kernel (/nix/store derivations)".to_string(),
            upstream_basis: "github.com/NixOS/nixpkgs".to_string(),
            primary_patches: vec![
                "Hermetic /nix/store kernel + initrd derivation".to_string(),
                "OpenZFS / Bcachefs declarative module trees".to_string(),
            ],
            target_usecase: "Bit-for-bit reproducible servers and atomic rollbacks".to_string(),
            scheduler_default: "EEVDF".to_string(),
            memory_allocator: "SLUB".to_string(),
            io_uring_status: "Enabled".to_string(),
            rust_readiness: "Hermetic rustc + bindgen derivations".to_string(),
            libc_target: "glibc (/nix/store)".to_string(),
            unique_features: vec![
                "Atomic generation symlink switching with instant rollback".to_string(),
            ],
            local_checkout_head: None,
        },
    ];

    let rewritten_kernels = vec![
        NonCKernel {
            name: "Redox OS".to_string(),
            language: "Rust".to_string(),
            architecture_type: "Microkernel".to_string(),
            posix_compliance_level: "High via relibc".to_string(),
            repo_url: "https://gitlab.redox-os.org/redox-os/redox".to_string(),
            status: "Active (Orbital GUI + POSIX userspace)".to_string(),
            key_innovations: vec![
                "URL-addressed schemes (`file:`, `tcp:`, `orbital:`)".to_string(),
                "Ring-3 userspace drivers and filesystems".to_string(),
            ],
        },
        NonCKernel {
            name: "Asterinas".to_string(),
            language: "Rust".to_string(),
            architecture_type: "Framekernel".to_string(),
            posix_compliance_level: "Linux ABI compatible".to_string(),
            repo_url: "https://github.com/asterinas/asterinas".to_string(),
            status: "Active (runs unmodified Linux binaries)".to_string(),
            key_innovations: vec![
                "Safe OSDK boundary isolating unsafe hardware primitives".to_string(),
            ],
        },
        NonCKernel {
            name: "Theseus OS".to_string(),
            language: "Rust".to_string(),
            architecture_type: "Single Address Space OS (SASOS)".to_string(),
            posix_compliance_level: "Selective".to_string(),
            repo_url: "https://github.com/theseus-os/Theseus".to_string(),
            status: "Research / Active".to_string(),
            key_innovations: vec![
                "Compiler-enforced isolation with live cell hot-swapping".to_string(),
            ],
        },
        NonCKernel {
            name: "seL4".to_string(),
            language: "Formally Verified C + Isabelle/HOL + Rust microkit".to_string(),
            architecture_type: "Capability Microkernel".to_string(),
            posix_compliance_level: "Via userspace personality servers".to_string(),
            repo_url: "https://github.com/seL4/seL4".to_string(),
            status: "Production high-assurance".to_string(),
            key_innovations: vec![
                "Machine-checked proof of functional correctness and isolation".to_string(),
            ],
        },
    ];

    let awesome_categories = vec![
        AwesomeListCategory {
            category: "Linux Kernel & eBPF Internals".to_string(),
            lists: vec![
                AwesomeListEntry {
                    title: "Linux Insides".to_string(),
                    repo_url: "https://github.com/0xAX/linux-insides".to_string(),
                    topic: "Boot, paging, interrupts, syscalls, schedulers".to_string(),
                    items_count_estimate: 450,
                    highlighted_innovations: vec![
                        "x86_64 early boot and virtual memory walkthroughs".to_string(),
                    ],
                },
                AwesomeListEntry {
                    title: "Awesome eBPF & sched-ext".to_string(),
                    repo_url: "https://github.com/zoidbergwill/awesome-ebpf".to_string(),
                    topic: "CO-RE tracing, XDP packet processing, BPF schedulers".to_string(),
                    items_count_estimate: 320,
                    highlighted_innovations: vec![
                        "Zero-copy XDP and programmable kernel scheduling".to_string(),
                    ],
                },
            ],
        },
        AwesomeListCategory {
            category: "POSIX.1-2024 & Rust Coreutils".to_string(),
            lists: vec![
                AwesomeListEntry {
                    title: "uutils coreutils".to_string(),
                    repo_url: "https://github.com/uutils/coreutils".to_string(),
                    topic: "Cross-platform Rust implementation of GNU coreutils".to_string(),
                    items_count_estimate: 110,
                    highlighted_innovations: vec![
                        "Memory-safe POSIX utilities across Linux and Windows NT".to_string(),
                    ],
                },
            ],
        },
    ];

    PosixDiscoveryCatalog {
        engine: "unix-cachyos::discovery (Rust 2024 native gitfile & patch tree scanner)".to_string(),
        local_linux_head,
        local_cachyos_patches_count,
        kernel_variants_2026: kernel_variants,
        rewritten_kernels_non_c: rewritten_kernels,
        awesome_ecosystem_indexes: awesome_categories,
        new_2026_concepts_and_workflows: vec![
            "sched-ext (SCX) programmable eBPF CPU schedulers".to_string(),
            "io_uring registered ring & fixed-buffer zero-copy networking".to_string(),
            "Rust DRM Nova GPU drivers and Rust Binder IPC".to_string(),
            "IEEE Std 1003.1-2024 (POSIX Issue 8) atomic *at() operations".to_string(),
            "WSL2 D3D12/CUDA sm_89 paravirtualization with native Rust Wayland compositor".to_string(),
        ],
    }
}

/// Write the discovery catalog JSON to `output_path`.
pub fn write_posix_discovery_catalog(monorepo_root: &Path, output_path: &Path) -> std::io::Result<PosixDiscoveryCatalog> {
    let catalog = build_posix_discovery_catalog(monorepo_root);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(&catalog)
        .map_err(std::io::Error::other)?;
    fs::write(output_path, json)?;
    Ok(catalog)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_discovery_catalog_with_variants_and_non_c_kernels() {
        let catalog = build_posix_discovery_catalog(Path::new("C:/src"));
        assert!(catalog.kernel_variants_2026.len() >= 5);
        assert!(catalog.rewritten_kernels_non_c.len() >= 4);
        assert!(!catalog.awesome_ecosystem_indexes.is_empty());
    }
}
