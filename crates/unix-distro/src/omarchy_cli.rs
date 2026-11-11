use anyhow::Result;
use aphrody_omarchy::{
    AetherThemeRegistry, OmarchyDoctorReport, OmarchyPluginEngine, SovereignPackageEngine,
    SovereignWslBuilder,
};
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum ThemesCommand {
    /// List all 25 Omarchy + Sovereign Aether themes
    List {
        #[arg(long)]
        json: bool,
    },
    /// Compile and export an Aether theme into Hyprland, COSMIC RON, Waybar, Terminal, Neovim, and CSS
    Export {
        theme_name: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Subcommand, Debug)]
pub enum PluginsCommand {
    /// List built-in Omarchy shell plugins and search the community marketplace
    List {
        #[arg(long)]
        query: Option<String>,
        #[arg(long, default_value_t = 25)]
        limit: usize,
        #[arg(long)]
        json: bool,
    },
    /// Scaffold a new Omarchy Quickshell plugin directory and manifest.json
    Scaffold {
        plugin_id: String,
        #[arg(long, default_value = "Sovereign Plugin")]
        name: String,
        #[arg(long, default_value = "bar-widget")]
        kind: String,
        #[arg(long)]
        out: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
pub enum PatchesCommand {
    /// List CachyOS + WSL2 kernel patches for the target Linux kernel series
    List {
        #[arg(long, default_value = "7.4")]
        kernel: String,
        #[arg(long)]
        json: bool,
    },
    /// Generate quilt/git patch series, applicator script, WSL2 .config fragment, and PKGBUILD
    GenerateSeries {
        #[arg(long, default_value = "7.4")]
        kernel: String,
        #[arg(long, default_value = "C:\\src\\crates\\unix\\wsl-distro\\patches")]
        out: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
pub enum WslCommand {
    /// Generate complete native WSL2 rootfs configuration bundle replacing QEMU TryOmarchy
    GenerateBundle {
        #[arg(long, default_value = "C:\\src\\crates\\unix\\wsl-distro")]
        out: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// Inspect or launch native WSL2 distribution installation (safe dry-run when .wsl image is absent)
    Install {
        #[arg(
            long,
            default_value = "C:\\src\\crates\\unix\\wsl-distro\\aphrody-omarchy.wsl"
        )]
        image: PathBuf,
        #[arg(long, default_value = "C:\\Users\\aphro\\AppData\\Local\\aphrody-omarchy")]
        install_dir: PathBuf,
        #[arg(long)]
        execute: bool,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum ArchCommand {
    /// Generate unified Arch GitLab devtools/pkgctl + CachyOS v3 + Omarchy repository manifest
    GitlabSyncManifest {
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

pub fn handle_themes(cmd: ThemesCommand) -> Result<()> {
    let registry = AetherThemeRegistry::new();
    match cmd {
        ThemesCommand::List { json } => {
            let list = registry.list();
            if json {
                println!("{}", serde_json::to_string_pretty(&list)?);
            } else {
                println!(
                    "=== Omarchy + Sovereign Aether Theme Registry (Total: {}) ===",
                    list.len()
                );
                for t in list {
                    println!(
                        "{:<20} [{:<5}] bg={} fg={} accent={} ({})",
                        t.name, t.mode, t.background, t.foreground, t.accent, t.source_kind
                    );
                }
            }
        }
        ThemesCommand::Export { theme_name, out } => {
            let theme = registry.get(&theme_name)?;
            if let Some(out_dir) = out {
                let written = theme.export_to_dir(&out_dir)?;
                println!(
                    "Exported Aether theme '{}' ({} files) into {}",
                    theme.name,
                    written.len(),
                    out_dir.display()
                );
                for p in written {
                    println!("  - {}", p.display());
                }
            } else {
                let bundle = theme.compile_all();
                println!("{}", serde_json::to_string_pretty(&bundle)?);
            }
        }
    }
    Ok(())
}

pub fn handle_plugins(cmd: PluginsCommand) -> Result<()> {
    let engine = OmarchyPluginEngine::new();
    match cmd {
        PluginsCommand::List { query, limit, json } => {
            let report = engine.search(query.as_deref(), limit)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!(
                    "=== Omarchy Shell Plugins (Built-in: {}, Marketplace Sources: {}) ===",
                    report.builtin_count, report.marketplace_source_count
                );
                println!("-- Built-in Matches ({}) --", report.builtin_matches.len());
                for b in &report.builtin_matches {
                    println!(
                        "  {:<22} [{:<13}] {} - {}",
                        b.id, b.kind, b.entry_point, b.description
                    );
                }
                println!(
                    "-- Marketplace Matches (showing {} of {}) --",
                    report.marketplace_matches.len(),
                    report.marketplace_source_count
                );
                for m in &report.marketplace_matches {
                    println!(
                        "  {:<32} [{:<14}] {}",
                        m.id, m.category, m.repo
                    );
                }
            }
        }
        PluginsCommand::Scaffold {
            plugin_id,
            name,
            kind,
            out,
        } => {
            let files = OmarchyPluginEngine::scaffold_plugin(&out, &plugin_id, &name, &kind)?;
            println!("Scaffolded Omarchy plugin '{}' into {}:", plugin_id, out.display());
            for f in files {
                println!("  - {}", f.display());
            }
        }
    }
    Ok(())
}

pub fn handle_patches(cmd: PatchesCommand) -> Result<()> {
    let engine = SovereignPackageEngine::new();
    match cmd {
        PatchesCommand::List { kernel, json } => {
            let patches = engine.list_kernel_patches(&kernel);
            if json {
                println!("{}", serde_json::to_string_pretty(&patches)?);
            } else {
                println!(
                    "=== Linux {} CachyOS + WSL2 Kernel Patch Series ({} patches) ===",
                    kernel,
                    patches.len()
                );
                for p in &patches {
                    println!(
                        "  {:02}. [{:<10}] {:<44} disk={} ({} bytes)",
                        p.order, p.subsystem, p.relative_path, p.present_on_disk, p.byte_size
                    );
                }
            }
        }
        PatchesCommand::GenerateSeries { kernel, out } => {
            let files = engine.generate_patch_series_bundle(&kernel, &out)?;
            println!(
                "Generated Linux {} patch series & config bundle into {}:",
                kernel,
                out.display()
            );
            for f in files {
                println!("  - {}", f.display());
            }
        }
    }
    Ok(())
}

pub fn handle_wsl(cmd: WslCommand) -> Result<()> {
    let builder = SovereignWslBuilder::new();
    match cmd {
        WslCommand::GenerateBundle { out, json } => {
            let manifest = builder.emit_bundle(&out)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&manifest)?);
            } else {
                println!(
                    "Generated Sovereign Native WSL2 Distribution Bundle '{}' into {}",
                    manifest.distro_name, manifest.out_dir
                );
                for f in &manifest.generated_files {
                    println!("  - {}", f);
                }
            }
        }
        WslCommand::Install {
            image,
            install_dir,
            execute,
            json,
        } => {
            let plan = builder.plan_or_launch_wsl(&image, &install_dir, execute)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&plan)?);
            } else {
                println!("=== Native WSL2 Distribution Orchestrator ===");
                println!("Distro Name:        {}", plan.distro_name);
                println!("Image Path:         {} (exists: {})", plan.image_path, plan.image_exists_on_disk);
                println!("Mode:               {}", if plan.dry_run { "DRY-RUN (safe)" } else { "LIVE INSTALL" });
                println!("Primary Command:    {}", plan.wsl_exe_command);
                println!("Import Command:     {}", plan.fallback_import_command);
                println!("Registered WSL VMs: {:?}", plan.existing_wsl_distros);
            }
        }
    }
    Ok(())
}

pub fn handle_arch(cmd: ArchCommand) -> Result<()> {
    let engine = SovereignPackageEngine::new();
    match cmd {
        ArchCommand::GitlabSyncManifest { out } => {
            let manifest = engine.generate_arch_gitlab_manifest();
            let serialized = serde_json::to_string_pretty(&manifest)?;
            if let Some(path) = out {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(&path, &serialized)?;
                println!("Saved Arch GitLab + CachyOS v3 + Omarchy manifest to {}", path.display());
            } else {
                println!("{}", serialized);
            }
        }
    }
    Ok(())
}

pub fn handle_doctor(json: bool) -> Result<()> {
    let report = OmarchyDoctorReport::run();
    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("=== Sovereign Omarchy + CachyOS + Arch WSL2 Doctor ===");
        println!("Hardware Tier:          {}", report.hardware_tier);
        println!("GPU Adapter:            {}", report.gpu_adapter);
        println!("Aether Themes:          {}", report.themes_total);
        println!(
            "Omarchy Built-in Plugs: {} (upstream dirs verified: {})",
            report.builtin_plugins_total, report.upstream_builtin_dirs_verified
        );
        println!(
            "Marketplace Sources:    {}",
            report.marketplace_sources_indexed
        );
        println!(
            "Linux 7.4 Patches:      {}/{} on disk",
            report.kernel_7_4_patches_on_disk, report.kernel_7_4_patches_total
        );
        println!("CachyOS PKGBUILDs:      {}", report.cachyos_pkgbuilds_on_disk);
        println!("Omarchy PKGBUILDs:      {}", report.omarchy_pkgbuilds_on_disk);
        println!("Arch GitLab Checkout:   {}", report.arch_gitlab_tree_present);
        println!("Linux 7.4-rc2 Tree:     {}", report.linux_7_4_tree_present);
        println!("Live WSL Distributions: {:?}", report.existing_wsl_distros);
        println!(
            "Overall Health:         {}",
            if report.healthy { "HEALTHY" } else { "DEGRADED" }
        );
    }
    Ok(())
}
