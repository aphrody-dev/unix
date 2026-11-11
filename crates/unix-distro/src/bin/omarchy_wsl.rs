#[path = "../omarchy_cli.rs"]
mod omarchy_cli;

use anyhow::Result;
use clap::{Parser, Subcommand};
use omarchy_cli::{ArchCommand, PatchesCommand, PluginsCommand, ThemesCommand, WslCommand};

#[derive(Parser, Debug)]
#[command(name = "omarchy-wsl")]
#[command(
    about = "Native Rust 2024 Omarchy + CachyOS + Arch Linux GitLab WSL2 Engine (Replaces QEMU TryOmarchy)",
    version
)]
struct OmarchyWslCli {
    #[command(subcommand)]
    command: OmarchyWslCommands,
}

#[derive(Subcommand, Debug)]
enum OmarchyWslCommands {
    /// Aether-in-Rust theme compiler (23 Omarchy + 2 Sovereign themes -> 6 native desktop/web formats)
    Themes {
        #[command(subcommand)]
        command: ThemesCommand,
    },
    /// Omarchy shell plugins & 6,443-source community marketplace engine
    Plugins {
        #[command(subcommand)]
        command: PluginsCommand,
    },
    /// Linux 7.4 CachyOS BORE + NVIDIA sm_89 + WSL2 /dev/dxg patch series generator
    Patches {
        #[command(subcommand)]
        command: PatchesCommand,
    },
    /// Native WSL2 distribution bundle generator & launcher
    Wsl {
        #[command(subcommand)]
        command: WslCommand,
    },
    /// Arch Linux GitLab devtools/pkgctl + CachyOS v3 + Omarchy repository manifest generator
    Arch {
        #[command(subcommand)]
        command: ArchCommand,
    },
    /// Full diagnostic check of themes, plugins, patches, PKGBUILDs, and WSL2 host state
    Doctor {
        #[arg(long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = OmarchyWslCli::parse();
    match cli.command {
        OmarchyWslCommands::Themes { command } => omarchy_cli::handle_themes(command),
        OmarchyWslCommands::Plugins { command } => omarchy_cli::handle_plugins(command),
        OmarchyWslCommands::Patches { command } => omarchy_cli::handle_patches(command),
        OmarchyWslCommands::Wsl { command } => omarchy_cli::handle_wsl(command),
        OmarchyWslCommands::Arch { command } => omarchy_cli::handle_arch(command),
        OmarchyWslCommands::Doctor { json } => omarchy_cli::handle_doctor(json),
    }
}
