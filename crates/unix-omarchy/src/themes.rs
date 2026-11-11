use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ThemeError {
    #[error("Theme I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid colors.toml syntax at line {line}: {message}")]
    Parse { line: usize, message: String },
    #[error("Theme not found: {0}")]
    NotFound(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorsToml {
    pub name: String,
    pub source_kind: String,
    pub mode: String,
    pub accent: String,
    pub selection: String,
    pub muted: String,
    pub background: String,
    pub dark_background: String,
    pub darker_background: String,
    pub lighter_background: String,
    pub foreground: String,
    pub dark_foreground: String,
    pub light_foreground: String,
    pub bright_foreground: String,
    pub red: String,
    pub yellow: String,
    pub orange: String,
    pub green: String,
    pub cyan: String,
    pub blue: String,
    pub magenta: String,
    pub brown: String,
    pub bright_red: String,
    pub bright_yellow: String,
    pub bright_green: String,
    pub bright_cyan: String,
    pub bright_blue: String,
    pub bright_magenta: String,
    pub hyprland_active_border: Option<String>,
    pub hyprland_inactive_border: Option<String>,
    pub active_border_color: Option<String>,
    pub active_tab_background: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledAetherBundle {
    pub theme_name: String,
    pub hyprland_colors_conf: String,
    pub cosmic_theme_ron: String,
    pub bar_css: String,
    pub terminal_toml: String,
    pub aether_theme_lua: String,
    pub theme_css: String,
}

impl ColorsToml {
    pub fn parse_toml_str(name: &str, source_kind: &str, raw: &str) -> Result<Self, ThemeError> {
        let mut kv = BTreeMap::new();
        for (idx, line) in raw.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let Some((key_part, val_part)) = trimmed.split_once('=') else {
                return Err(ThemeError::Parse {
                    line: idx + 1,
                    message: format!("expected key = \"value\", got '{}'", trimmed),
                });
            };
            let key = key_part.trim().to_string();
            let val_no_comment = strip_inline_comment(val_part.trim());
            let unquoted = val_no_comment
                .trim()
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .or_else(|| {
                    val_no_comment
                        .trim()
                        .strip_prefix('\'')
                        .and_then(|s| s.strip_suffix('\''))
                })
                .ok_or_else(|| ThemeError::Parse {
                    line: idx + 1,
                    message: format!("expected quoted string for key '{}'", key),
                })?;
            kv.insert(key, unquoted.to_string());
        }

        let get_or = |key: &str, fallback: &str| -> String {
            kv.get(key)
                .cloned()
                .unwrap_or_else(|| fallback.to_string())
        };

        let mode = get_or("mode", "dark");
        let is_dark = mode != "light";
        let default_bg = if is_dark { "#1a1b26" } else { "#ffffff" };
        let default_fg = if is_dark { "#c0caf5" } else { "#101010" };

        let background = get_or("background", default_bg);
        let dark_background = get_or("dark_background", &background);
        let darker_background = get_or("darker_background", &dark_background);
        let lighter_background = get_or("lighter_background", &background);

        let foreground = get_or("foreground", default_fg);
        let dark_foreground = get_or("dark_foreground", &foreground);
        let light_foreground = get_or("light_foreground", &foreground);
        let bright_foreground = get_or("bright_foreground", &foreground);

        let accent = get_or("accent", "#7aa2f7");
        let selection = get_or("selection", &lighter_background);
        let muted = get_or("muted", &dark_foreground);

        let red = get_or("red", "#f7768e");
        let yellow = get_or("yellow", "#e0af68");
        let orange = get_or("orange", &yellow);
        let green = get_or("green", "#9ece6a");
        let cyan = get_or("cyan", "#7dcfff");
        let blue = get_or("blue", &accent);
        let magenta = get_or("magenta", "#bb9af7");
        let brown = get_or("brown", &muted);

        let bright_red = get_or("bright_red", &red);
        let bright_yellow = get_or("bright_yellow", &yellow);
        let bright_green = get_or("bright_green", &green);
        let bright_cyan = get_or("bright_cyan", &cyan);
        let bright_blue = get_or("bright_blue", &blue);
        let bright_magenta = get_or("bright_magenta", &magenta);

        Ok(Self {
            name: name.to_string(),
            source_kind: source_kind.to_string(),
            mode,
            accent,
            selection,
            muted,
            background,
            dark_background,
            darker_background,
            lighter_background,
            foreground,
            dark_foreground,
            light_foreground,
            bright_foreground,
            red,
            yellow,
            orange,
            green,
            cyan,
            blue,
            magenta,
            brown,
            bright_red,
            bright_yellow,
            bright_green,
            bright_cyan,
            bright_blue,
            bright_magenta,
            hyprland_active_border: kv.get("hyprland_active_border").cloned(),
            hyprland_inactive_border: kv.get("hyprland_inactive_border").cloned(),
            active_border_color: kv.get("active_border_color").cloned(),
            active_tab_background: kv.get("active_tab_background").cloned(),
        })
    }

    pub fn from_file(name: &str, path: &Path) -> Result<Self, ThemeError> {
        let raw = fs::read_to_string(path)?;
        Self::parse_toml_str(name, "upstream-disk", &raw)
    }

    pub fn is_dark(&self) -> bool {
        self.mode != "light"
    }

    pub fn strip_hex(color: &str) -> String {
        color.trim().trim_start_matches('#').to_lowercase()
    }

    pub fn hex_to_rgba_f32(color: &str) -> (f32, f32, f32, f32) {
        let hex = Self::strip_hex(color);
        if hex.len() >= 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
            let a = if hex.len() >= 8 {
                u8::from_str_radix(&hex[6..8], 16).unwrap_or(255) as f32 / 255.0
            } else {
                1.0
            };
            (r, g, b, a)
        } else {
            (0.0, 0.0, 0.0, 1.0)
        }
    }

    pub fn compile_hyprland(&self) -> String {
        let active_border = self.hyprland_active_border.clone().unwrap_or_else(|| {
            format!(
                "rgba({}ee) rgba({}ee) 45deg",
                Self::strip_hex(&self.accent),
                Self::strip_hex(&self.blue)
            )
        });
        let inactive_border = self
            .hyprland_inactive_border
            .clone()
            .unwrap_or_else(|| format!("rgba({}aa)", Self::strip_hex(&self.muted)));
        let shadow_color = format!("rgba({}ee)", Self::strip_hex(&self.darker_background));
        let group_active = format!("rgba({}ee)", Self::strip_hex(&self.accent));
        let group_inactive = format!("rgba({}aa)", Self::strip_hex(&self.lighter_background));

        format!(
            "# Aether-in-Rust Hyprland Theme: {name} ({mode})\n\
             $activeBorderColor = {active_border}\n\
             $inactiveBorderColor = {inactive_border}\n\
             $shadowColor = {shadow_color}\n\
             \n\
             general {{\n\
             \x20   col.active_border = $activeBorderColor\n\
             \x20   col.inactive_border = $inactiveBorderColor\n\
             }}\n\
             \n\
             decoration {{\n\
             \x20   shadow {{\n\
             \x20       enabled = true\n\
             \x20       color = $shadowColor\n\
             \x20   }}\n\
             }}\n\
             \n\
             group {{\n\
             \x20   col.border_active = {group_active}\n\
             \x20   col.border_inactive = {group_inactive}\n\
             \x20   groupbar {{\n\
             \x20       col.active = {group_active}\n\
             \x20       col.inactive = {group_inactive}\n\
             \x20       text_color = rgb({fg})\n\
             \x20   }}\n\
             }}\n",
            name = self.name,
            mode = self.mode,
            active_border = active_border,
            inactive_border = inactive_border,
            shadow_color = shadow_color,
            group_active = group_active,
            group_inactive = group_inactive,
            fg = Self::strip_hex(&self.foreground),
        )
    }

    pub fn compile_cosmic_ron(&self) -> String {
        let (pr, pg, pb, pa) = Self::hex_to_rgba_f32(&self.accent);
        let (opr, opg, opb, opa) = Self::hex_to_rgba_f32(&self.darker_background);
        let (sr, sg, sb, sa) = Self::hex_to_rgba_f32(&self.background);
        let (osr, osg, osb, osa) = Self::hex_to_rgba_f32(&self.foreground);
        let (scr, scg, scb, sca) = Self::hex_to_rgba_f32(&self.lighter_background);
        let (olr, olg, olb, ola) = Self::hex_to_rgba_f32(&self.muted);

        format!(
            "(\n\
             \x20   name: \"Omarchy Aether - {name}\",\n\
             \x20   is_dark: {is_dark},\n\
             \x20   primary: (red: {pr:.4}, green: {pg:.4}, blue: {pb:.4}, alpha: {pa:.4}),\n\
             \x20   on_primary: (red: {opr:.4}, green: {opg:.4}, blue: {opb:.4}, alpha: {opa:.4}),\n\
             \x20   surface: (red: {sr:.4}, green: {sg:.4}, blue: {sb:.4}, alpha: {sa:.4}),\n\
             \x20   on_surface: (red: {osr:.4}, green: {osg:.4}, blue: {osb:.4}, alpha: {osa:.4}),\n\
             \x20   surface_container: (red: {scr:.4}, green: {scg:.4}, blue: {scb:.4}, alpha: {sca:.4}),\n\
             \x20   outline: (red: {olr:.4}, green: {olg:.4}, blue: {olb:.4}, alpha: {ola:.4}),\n\
             \x20   corner_radius: 12.0,\n\
             \x20   spacing: 8.0,\n\
             )\n",
            name = self.name,
            is_dark = self.is_dark(),
        )
    }

    pub fn compile_bar_css(&self) -> String {
        format!(
            "/* Aether-in-Rust Waybar & Quickshell Stylesheet: {name} */\n\
             @define-color background {bg};\n\
             @define-color dark_background {dark_bg};\n\
             @define-color darker_background {darker_bg};\n\
             @define-color lighter_background {lighter_bg};\n\
             @define-color foreground {fg};\n\
             @define-color accent {accent};\n\
             @define-color selection {selection};\n\
             @define-color muted {muted};\n\
             @define-color urgent {red};\n\
             @define-color warning {yellow};\n\
             @define-color success {green};\n\
             \n\
             window#waybar, .omarchy-bar {{\n\
             \x20   background-color: @background;\n\
             \x20   color: @foreground;\n\
             \x20   border-bottom: 1px solid @lighter_background;\n\
             }}\n\
             \n\
             #workspaces button.active {{\n\
             \x20   color: @accent;\n\
             \x20   background-color: @selection;\n\
             }}\n",
            name = self.name,
            bg = self.background,
            dark_bg = self.dark_background,
            darker_bg = self.darker_background,
            lighter_bg = self.lighter_background,
            fg = self.foreground,
            accent = self.accent,
            selection = self.selection,
            muted = self.muted,
            red = self.red,
            yellow = self.yellow,
            green = self.green,
        )
    }

    pub fn compile_terminal_toml(&self) -> String {
        format!(
            "# Aether-in-Rust Unified Terminal Palette (Alacritty / Rio / Ghostty): {name}\n\
             [colors.primary]\n\
             background = \"{bg}\"\n\
             foreground = \"{fg}\"\n\
             \n\
             [colors.cursor]\n\
             text = \"{bg}\"\n\
             cursor = \"{fg}\"\n\
             \n\
             [colors.selection]\n\
             text = \"{fg}\"\n\
             background = \"{selection}\"\n\
             \n\
             [colors.normal]\n\
             black   = \"{dark_bg}\"\n\
             red     = \"{red}\"\n\
             green   = \"{green}\"\n\
             yellow  = \"{yellow}\"\n\
             blue    = \"{blue}\"\n\
             magenta = \"{magenta}\"\n\
             cyan    = \"{cyan}\"\n\
             white   = \"{light_fg}\"\n\
             \n\
             [colors.bright]\n\
             black   = \"{muted}\"\n\
             red     = \"{bright_red}\"\n\
             green   = \"{bright_green}\"\n\
             yellow  = \"{bright_yellow}\"\n\
             blue    = \"{bright_blue}\"\n\
             magenta = \"{bright_magenta}\"\n\
             cyan    = \"{bright_cyan}\"\n\
             white   = \"{bright_fg}\"\n",
            name = self.name,
            bg = self.background,
            fg = self.foreground,
            selection = self.selection,
            dark_bg = self.dark_background,
            red = self.red,
            green = self.green,
            yellow = self.yellow,
            blue = self.blue,
            magenta = self.magenta,
            cyan = self.cyan,
            light_fg = self.light_foreground,
            muted = self.muted,
            bright_red = self.bright_red,
            bright_green = self.bright_green,
            bright_yellow = self.bright_yellow,
            bright_blue = self.bright_blue,
            bright_magenta = self.bright_magenta,
            bright_cyan = self.bright_cyan,
            bright_fg = self.bright_foreground,
        )
    }

    pub fn compile_neovim_lua(&self) -> String {
        format!(
            "-- Aether-in-Rust Neovim Theme Spec: {name}\n\
             return {{\n\
             \x20 {{\n\
             \x20   \"omacom/aether.nvim\",\n\
             \x20   branch = \"v3\",\n\
             \x20   name = \"aether\",\n\
             \x20   priority = 1000,\n\
             \x20   opts = {{\n\
             \x20     colors = {{\n\
             \x20       bg         = \"{bg}\",\n\
             \x20       dark_bg    = \"{dark_bg}\",\n\
             \x20       darker_bg  = \"{darker_bg}\",\n\
             \x20       lighter_bg = \"{lighter_bg}\",\n\
             \x20       fg         = \"{fg}\",\n\
             \x20       dark_fg    = \"{dark_fg}\",\n\
             \x20       light_fg   = \"{light_fg}\",\n\
             \x20       bright_fg  = \"{bright_fg}\",\n\
             \x20       muted      = \"{muted}\",\n\
             \x20       red        = \"{red}\",\n\
             \x20       yellow     = \"{yellow}\",\n\
             \x20       orange     = \"{orange}\",\n\
             \x20       green      = \"{green}\",\n\
             \x20       cyan       = \"{cyan}\",\n\
             \x20       blue       = \"{blue}\",\n\
             \x20       purple     = \"{magenta}\",\n\
             \x20       brown      = \"{brown}\",\n\
             \x20       bright_red    = \"{bright_red}\",\n\
             \x20       bright_yellow = \"{bright_yellow}\",\n\
             \x20       bright_green  = \"{bright_green}\",\n\
             \x20       bright_cyan   = \"{bright_cyan}\",\n\
             \x20       bright_blue   = \"{bright_blue}\",\n\
             \x20       bright_purple = \"{bright_magenta}\",\n\
             \x20       accent               = \"{accent}\",\n\
             \x20       cursor               = \"{fg}\",\n\
             \x20       foreground           = \"{fg}\",\n\
             \x20       background           = \"{bg}\",\n\
             \x20       selection            = \"{selection}\",\n\
             \x20       selection_foreground = \"{fg}\",\n\
             \x20       selection_background = \"{selection}\",\n\
             \x20     }},\n\
             \x20   }},\n\
             \x20 }},\n\
             \x20 {{\n\
             \x20   \"LazyVim/LazyVim\",\n\
             \x20   opts = {{\n\
             \x20     colorscheme = \"aether\",\n\
             \x20   }},\n\
             \x20 }},\n\
             }}\n",
            name = self.name,
            bg = self.background,
            dark_bg = self.dark_background,
            darker_bg = self.darker_background,
            lighter_bg = self.lighter_background,
            fg = self.foreground,
            dark_fg = self.dark_foreground,
            light_fg = self.light_foreground,
            bright_fg = self.bright_foreground,
            muted = self.muted,
            red = self.red,
            yellow = self.yellow,
            orange = self.orange,
            green = self.green,
            cyan = self.cyan,
            blue = self.blue,
            magenta = self.magenta,
            brown = self.brown,
            bright_red = self.bright_red,
            bright_yellow = self.bright_yellow,
            bright_green = self.bright_green,
            bright_cyan = self.bright_cyan,
            bright_blue = self.bright_blue,
            bright_magenta = self.bright_magenta,
            accent = self.accent,
            selection = self.selection,
        )
    }

    pub fn compile_web_css(&self) -> String {
        format!(
            "/* Aether-in-Rust CSS Custom Properties: {name} ({mode}) */\n\
             :root,\n\
             [data-omarchy-theme=\"{name}\"] {{\n\
             \x20 --omarchy-mode: \"{mode}\";\n\
             \x20 --omarchy-accent: {accent};\n\
             \x20 --omarchy-selection: {selection};\n\
             \x20 --omarchy-muted: {muted};\n\
             \x20 --omarchy-bg: {bg};\n\
             \x20 --omarchy-bg-dark: {dark_bg};\n\
             \x20 --omarchy-bg-darker: {darker_bg};\n\
             \x20 --omarchy-bg-lighter: {lighter_bg};\n\
             \x20 --omarchy-fg: {fg};\n\
             \x20 --omarchy-fg-dark: {dark_fg};\n\
             \x20 --omarchy-fg-light: {light_fg};\n\
             \x20 --omarchy-fg-bright: {bright_fg};\n\
             \x20 --omarchy-red: {red};\n\
             \x20 --omarchy-yellow: {yellow};\n\
             \x20 --omarchy-orange: {orange};\n\
             \x20 --omarchy-green: {green};\n\
             \x20 --omarchy-cyan: {cyan};\n\
             \x20 --omarchy-blue: {blue};\n\
             \x20 --omarchy-magenta: {magenta};\n\
             \x20 --omarchy-brown: {brown};\n\
             }}\n",
            name = self.name,
            mode = self.mode,
            accent = self.accent,
            selection = self.selection,
            muted = self.muted,
            bg = self.background,
            dark_bg = self.dark_background,
            darker_bg = self.darker_background,
            lighter_bg = self.lighter_background,
            fg = self.foreground,
            dark_fg = self.dark_foreground,
            light_fg = self.light_foreground,
            bright_fg = self.bright_foreground,
            red = self.red,
            yellow = self.yellow,
            orange = self.orange,
            green = self.green,
            cyan = self.cyan,
            blue = self.blue,
            magenta = self.magenta,
            brown = self.brown,
        )
    }

    pub fn compile_all(&self) -> CompiledAetherBundle {
        CompiledAetherBundle {
            theme_name: self.name.clone(),
            hyprland_colors_conf: self.compile_hyprland(),
            cosmic_theme_ron: self.compile_cosmic_ron(),
            bar_css: self.compile_bar_css(),
            terminal_toml: self.compile_terminal_toml(),
            aether_theme_lua: self.compile_neovim_lua(),
            theme_css: self.compile_web_css(),
        }
    }

    pub fn export_to_dir(&self, out_dir: &Path) -> Result<Vec<PathBuf>, ThemeError> {
        fs::create_dir_all(out_dir)?;
        let bundle = self.compile_all();
        let files = [
            ("hyprland-colors.conf", bundle.hyprland_colors_conf),
            ("cosmic-theme.ron", bundle.cosmic_theme_ron),
            ("bar.css", bundle.bar_css),
            ("terminal.toml", bundle.terminal_toml),
            ("aether-theme.lua", bundle.aether_theme_lua),
            ("theme.css", bundle.theme_css),
        ];
        let mut written = Vec::with_capacity(files.len());
        for (filename, content) in files {
            let path = out_dir.join(filename);
            fs::write(&path, content)?;
            written.push(path);
        }
        Ok(written)
    }
}

fn strip_inline_comment(input: &str) -> &str {
    let mut in_double = false;
    let mut in_single = false;
    for (idx, ch) in input.char_indices() {
        match ch {
            '"' if !in_single => in_double = !in_double,
            '\'' if !in_double => in_single = !in_single,
            '#' if !in_double && !in_single => return input[..idx].trim(),
            _ => {}
        }
    }
    input
}

pub struct AetherThemeRegistry {
    themes: BTreeMap<String, ColorsToml>,
}

impl Default for AetherThemeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AetherThemeRegistry {
    pub const DEFAULT_UPSTREAM_THEMES_DIR: &'static str =
        r"C:\src\third_party\omacom\omarchy\themes";

    pub fn new() -> Self {
        Self::with_upstream_dir(Path::new(Self::DEFAULT_UPSTREAM_THEMES_DIR))
    }

    pub fn with_upstream_dir(upstream_dir: &Path) -> Self {
        let mut themes = BTreeMap::new();

        for (name, source_kind, raw_toml) in builtin_presets() {
            if let Ok(parsed) = ColorsToml::parse_toml_str(name, source_kind, raw_toml) {
                themes.insert(name.to_string(), parsed);
            }
        }

        if upstream_dir.is_dir()
            && let Ok(entries) = fs::read_dir(upstream_dir)
        {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let colors_path = path.join("colors.toml");
                    if colors_path.is_file()
                        && let Some(dir_name) = path.file_name().and_then(|s| s.to_str())
                        && let Ok(parsed) = ColorsToml::from_file(dir_name, &colors_path)
                    {
                        themes.insert(dir_name.to_string(), parsed);
                    }
                }
            }
        }

        Self { themes }
    }

    pub fn standalone_only() -> Self {
        let mut themes = BTreeMap::new();
        for (name, source_kind, raw_toml) in builtin_presets() {
            let parsed = ColorsToml::parse_toml_str(name, source_kind, raw_toml)
                .expect("built-in theme preset must parse cleanly");
            themes.insert(name.to_string(), parsed);
        }
        Self { themes }
    }

    pub fn list(&self) -> Vec<&ColorsToml> {
        self.themes.values().collect()
    }

    pub fn get(&self, name: &str) -> Result<&ColorsToml, ThemeError> {
        self.themes
            .get(name)
            .ok_or_else(|| ThemeError::NotFound(name.to_string()))
    }

    pub fn len(&self) -> usize {
        self.themes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.themes.is_empty()
    }
}

fn builtin_presets() -> [(&'static str, &'static str, &'static str); 25] {
    [
        (
            "catppuccin",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#89b4fa"
selection = "#45475a"
muted = "#585b70"
background = "#1e1e2e"
dark_background = "#161622"
darker_background = "#101019"
lighter_background = "#313244"
foreground = "#cdd6f4"
dark_foreground = "#6c7086"
light_foreground = "#bac2de"
bright_foreground = "#cdd6f4"
red = "#f38ba8"
yellow = "#f9e2af"
orange = "#f6b6ab"
green = "#a6e3a1"
cyan = "#94e2d5"
blue = "#89b4fa"
magenta = "#f5c2e7"
brown = "#7b5b55"
bright_red = "#f38ba8"
bright_yellow = "#f9e2af"
bright_green = "#a6e3a1"
bright_cyan = "#94e2d5"
bright_blue = "#89b4fa"
bright_magenta = "#f5c2e7"
"##,
        ),
        (
            "catppuccin-latte",
            "omarchy-builtin",
            r##"
mode = "light"
accent = "#1e66f5"
selection = "#ccd0da"
muted = "#acb0be"
background = "#eff1f5"
dark_background = "#e3e4e8"
darker_background = "#d7d8dc"
lighter_background = "#dce0e8"
foreground = "#4c4f69"
dark_foreground = "#9ca0b0"
light_foreground = "#5c5f77"
bright_foreground = "#4c4f69"
red = "#d20f39"
yellow = "#df8e1d"
orange = "#d84e2b"
green = "#40a02b"
cyan = "#179299"
blue = "#1e66f5"
magenta = "#ea76cb"
brown = "#6c2715"
bright_red = "#d20f39"
bright_yellow = "#df8e1d"
bright_green = "#40a02b"
bright_cyan = "#179299"
bright_blue = "#1e66f5"
bright_magenta = "#ea76cb"
"##,
        ),
        (
            "ethereal",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#7d82d9"
selection = "#252e56"
muted = "#6d7db6"
background = "#060B1E"
dark_background = "#040816"
darker_background = "#030610"
lighter_background = "#131a3a"
foreground = "#ffcead"
dark_foreground = "#6d7db6"
light_foreground = "#c9b8a6"
bright_foreground = "#ffcead"
red = "#ED5B5A"
yellow = "#E9BB4F"
orange = "#eb8b54"
green = "#92a593"
cyan = "#a3bfd1"
blue = "#7d82d9"
magenta = "#c89dc1"
brown = "#75452a"
bright_red = "#faaaa9"
bright_yellow = "#f7dc9c"
bright_green = "#c4cfc4"
bright_cyan = "#dfeaf0"
bright_blue = "#c2c4f0"
bright_magenta = "#ead7e7"
"##,
        ),
        (
            "everforest",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#7fbbb3"
selection = "#3d484d"
muted = "#475258"
background = "#2d353b"
dark_background = "#21272c"
darker_background = "#181d20"
lighter_background = "#343f44"
foreground = "#d3c6aa"
dark_foreground = "#4f585e"
light_foreground = "#9da9a0"
bright_foreground = "#d3c6aa"
red = "#e67e80"
yellow = "#dbbc7f"
orange = "#e09d7f"
green = "#a7c080"
cyan = "#83c092"
blue = "#7fbbb3"
magenta = "#d699b6"
brown = "#704e3f"
bright_red = "#e67e80"
bright_yellow = "#dbbc7f"
bright_green = "#a7c080"
bright_cyan = "#83c092"
bright_blue = "#7fbbb3"
bright_magenta = "#d699b6"
"##,
        ),
        (
            "flexoki-light",
            "omarchy-builtin",
            r##"
mode = "light"
accent = "#205EA6"
selection = "#CECDC3"
muted = "#B7B5AC"
background = "#FFFCF0"
dark_background = "#f2efe4"
darker_background = "#e5e2d8"
lighter_background = "#E6E4D9"
foreground = "#100F0F"
dark_foreground = "#878580"
light_foreground = "#403E3C"
bright_foreground = "#100F0F"
red = "#D14D41"
yellow = "#D0A215"
orange = "#d0772b"
green = "#879A39"
cyan = "#3AA99F"
blue = "#205EA6"
magenta = "#CE5D97"
brown = "#683b15"
bright_red = "#D14D41"
bright_yellow = "#D0A215"
bright_green = "#879A39"
bright_cyan = "#3AA99F"
bright_blue = "#4385BE"
bright_magenta = "#CE5D97"
"##,
        ),
        (
            "gruvbox",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#7daea3"
selection = "#504945"
muted = "#665c54"
background = "#282828"
dark_background = "#1e1e1e"
darker_background = "#161616"
lighter_background = "#3c3836"
foreground = "#d4be98"
dark_foreground = "#7c6f64"
light_foreground = "#bdae93"
bright_foreground = "#d4be98"
red = "#ea6962"
yellow = "#d8a657"
orange = "#e1875c"
green = "#a9b665"
cyan = "#89b482"
blue = "#7daea3"
magenta = "#d3869b"
brown = "#70432e"
bright_red = "#ea6962"
bright_yellow = "#d8a657"
bright_green = "#a9b665"
bright_cyan = "#89b482"
bright_blue = "#7daea3"
bright_magenta = "#d3869b"
"##,
        ),
        (
            "hackerman",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#82FB9C"
selection = "#1f253a"
muted = "#2d3450"
background = "#0B0C16"
dark_background = "#080910"
darker_background = "#06060c"
lighter_background = "#151828"
foreground = "#ddf7ff"
dark_foreground = "#6a6e95"
light_foreground = "#b5c5db"
bright_foreground = "#ddf7ff"
hyprland_active_border = "rgba(26a269ee) rgba(2ec27eee) 45deg"
red = "#50f872"
yellow = "#50f7d4"
orange = "#50f7a3"
green = "#4fe88f"
cyan = "#7cf8f7"
blue = "#829dd4"
magenta = "#86a7df"
brown = "#287b51"
bright_red = "#85ff9d"
bright_yellow = "#a4ffec"
bright_green = "#9cf7c2"
bright_cyan = "#d1fffe"
bright_blue = "#c4d2ed"
bright_magenta = "#cddbf4"
"##,
        ),
        (
            "jade-horse",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#8bcbb1"
selection = "#2b5147"
muted = "#64887b"
background = "#142620"
dark_background = "#0f1e19"
darker_background = "#0a1713"
lighter_background = "#203b31"
foreground = "#dfede5"
dark_foreground = "#91ad9f"
light_foreground = "#bed6c9"
bright_foreground = "#f0faf4"
red = "#d88d91"
yellow = "#c5cc91"
orange = "#d2ab8d"
green = "#8bcbb1"
cyan = "#86c7c5"
blue = "#96b7d0"
magenta = "#b7a1c8"
brown = "#7e8970"
bright_red = "#e6a6aa"
bright_yellow = "#dce1ac"
bright_green = "#a7dfc5"
bright_cyan = "#a2dcda"
bright_blue = "#b0cde3"
bright_magenta = "#cfbce0"
"##,
        ),
        (
            "kanagawa",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#dcd7ba"
selection = "#363646"
muted = "#54546D"
background = "#1f1f28"
dark_background = "#17171e"
darker_background = "#111116"
lighter_background = "#223249"
foreground = "#dcd7ba"
dark_foreground = "#727169"
light_foreground = "#c8c093"
bright_foreground = "#dcd7ba"
red = "#c34043"
yellow = "#c0a36e"
orange = "#c17158"
green = "#76946a"
cyan = "#6a9589"
blue = "#7e9cd8"
magenta = "#957fb8"
brown = "#60382c"
bright_red = "#e82424"
bright_yellow = "#e6c384"
bright_green = "#98bb6c"
bright_cyan = "#7aa89f"
bright_blue = "#7fb4ca"
bright_magenta = "#938aa9"
"##,
        ),
        (
            "last-horizon",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#b59790"
selection = "#584e51"
muted = "#584e51"
background = "#0c0b0c"
dark_background = "#090809"
darker_background = "#060606"
lighter_background = "#0c0b0c"
foreground = "#FAFCFB"
dark_foreground = "#584e51"
light_foreground = "#cfd3cd"
bright_foreground = "#e2dddc"
hyprland_active_border = "rgba(8a8588ee) rgba(e2dddcee)"
hyprland_inactive_border = "rgba(584e51aa)"
active_border_color = "#d6d3de"
active_tab_background = "#a5a0b6"
red = "#c38b7b"
yellow = "#6B5E73"
green = "#87a9b0"
cyan = "#a5a0b6"
blue = "#b59790"
magenta = "#c4d8e2"
bright_red = "#c38b7b"
bright_yellow = "#6B5E73"
bright_green = "#87a9b0"
bright_cyan = "#a5a0b6"
bright_blue = "#b59790"
bright_magenta = "#c4d8e2"
"##,
        ),
        (
            "lumon",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#8bc9eb"
selection = "#243d56"
muted = "#304860"
background = "#16242d"
dark_background = "#101b21"
darker_background = "#0b1216"
lighter_background = "#1b2d40"
foreground = "#d6e2ee"
dark_foreground = "#4d86b0"
light_foreground = "#d6e2ee"
bright_foreground = "#f2fcff"
active_border_color = "#f2fcff"
active_tab_background = "#6fb8e3"
red = "#4d86b0"
yellow = "#6fa4c9"
orange = "#8bc9eb"
green = "#5e95bc"
cyan = "#b4e4f6"
blue = "#6fb8e3"
magenta = "#8bc9eb"
brown = "#456475"
bright_red = "#73a6cb"
bright_yellow = "#9dcae5"
bright_green = "#86b7d8"
bright_cyan = "#d1eef8"
bright_blue = "#f2fcff"
bright_magenta = "#b1d8ee"
"##,
        ),
        (
            "lupine",
            "omarchy-builtin",
            r##"
mode = "light"
accent = "#3264eb"
selection = "#d0d0d0"
muted = "#9e9e9e"
background = "#fafafa"
dark_background = "#ececec"
darker_background = "#dedede"
lighter_background = "#f5f5f5"
foreground = "#212121"
dark_foreground = "#757575"
light_foreground = "#424242"
bright_foreground = "#000000"
red = "#c900c4"
yellow = "#026fde"
orange = "#026fde"
green = "#4a2fd0"
cyan = "#0c67de"
blue = "#3264eb"
magenta = "#8a4ad7"
brown = "#013a6f"
bright_red = "#f930fb"
bright_yellow = "#358fff"
bright_green = "#9f85e0"
bright_cyan = "#3986ff"
bright_blue = "#5482ff"
bright_magenta = "#b363ff"
"##,
        ),
        (
            "matte-black",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#e68e0d"
selection = "#2a2a2a"
muted = "#333333"
background = "#121212"
dark_background = "#0d0d0d"
darker_background = "#090909"
lighter_background = "#1e1e1e"
foreground = "#bebebe"
dark_foreground = "#555555"
light_foreground = "#8a8a8d"
bright_foreground = "#bebebe"
red = "#D35F5F"
yellow = "#b91c1c"
orange = "#c63d3d"
green = "#FFC107"
cyan = "#bebebe"
blue = "#e68e0d"
magenta = "#D35F5F"
brown = "#631e1e"
bright_red = "#B91C1C"
bright_yellow = "#b90a0a"
bright_green = "#FFC107"
bright_cyan = "#eaeaea"
bright_blue = "#f59e0b"
bright_magenta = "#B91C1C"
"##,
        ),
        (
            "miasma",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#78824b"
selection = "#383838"
muted = "#666666"
background = "#222222"
dark_background = "#191919"
darker_background = "#121212"
lighter_background = "#2c2c2c"
foreground = "#c2c2b0"
dark_foreground = "#555555"
light_foreground = "#8a8a7e"
bright_foreground = "#c2c2b0"
red = "#685742"
yellow = "#b36d43"
orange = "#8d6242"
green = "#5f875f"
cyan = "#c9a554"
blue = "#78824b"
magenta = "#bb7744"
brown = "#463121"
bright_red = "#685742"
bright_yellow = "#b36d43"
bright_green = "#5f875f"
bright_cyan = "#c9a554"
bright_blue = "#78824b"
bright_magenta = "#bb7744"
"##,
        ),
        (
            "nord",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#81a1c1"
selection = "#434c5e"
muted = "#4c566a"
background = "#2e3440"
dark_background = "#222730"
darker_background = "#191c23"
lighter_background = "#3b4252"
foreground = "#d8dee9"
dark_foreground = "#667080"
light_foreground = "#adb5c4"
bright_foreground = "#d8dee9"
red = "#bf616a"
yellow = "#ebcb8b"
orange = "#d5967a"
green = "#a3be8c"
cyan = "#88c0d0"
blue = "#81a1c1"
magenta = "#b48ead"
brown = "#6a4b3d"
bright_red = "#bf616a"
bright_yellow = "#ebcb8b"
bright_green = "#a3be8c"
bright_cyan = "#8fbcbb"
bright_blue = "#81a1c1"
bright_magenta = "#b48ead"
"##,
        ),
        (
            "retro-82",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#faa968"
selection = "#134e5a"
muted = "#2a6b78"
background = "#05182e"
dark_background = "#031222"
darker_background = "#020c17"
lighter_background = "#0a2540"
foreground = "#f6dcac"
dark_foreground = "#3f8f8a"
light_foreground = "#a7c9c6"
bright_foreground = "#f6dcac"
red = "#f85525"
yellow = "#e97b3c"
orange = "#faa968"
green = "#028391"
cyan = "#8cbfb8"
blue = "#3f8f8a"
magenta = "#3f8f8a"
brown = "#743d1e"
bright_red = "#f85525"
bright_yellow = "#e97b3c"
bright_green = "#028391"
bright_cyan = "#8cbfb8"
bright_blue = "#faa968"
bright_magenta = "#3f8f8a"
"##,
        ),
        (
            "ristretto",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#f38d70"
selection = "#403e41"
muted = "#72696a"
background = "#2c2525"
dark_background = "#211b1b"
darker_background = "#181414"
lighter_background = "#3d2f2a"
foreground = "#e6d9db"
dark_foreground = "#72696a"
light_foreground = "#c3b7b8"
bright_foreground = "#e6d9db"
red = "#fd6883"
yellow = "#f9cc6c"
orange = "#fb9a77"
green = "#adda78"
cyan = "#85dacc"
blue = "#f38d70"
magenta = "#a8a9eb"
brown = "#7d4d3b"
bright_red = "#ff8297"
bright_yellow = "#fcd675"
bright_green = "#c8e292"
bright_cyan = "#9bf1e1"
bright_blue = "#f8a788"
bright_magenta = "#bebffd"
"##,
        ),
        (
            "rose-pine",
            "omarchy-builtin",
            r##"
mode = "light"
accent = "#56949f"
selection = "#dfdad9"
muted = "#cecacd"
background = "#faf4ed"
dark_background = "#ede7e1"
darker_background = "#e1dbd5"
lighter_background = "#f2e9e1"
foreground = "#575279"
dark_foreground = "#9893a5"
light_foreground = "#6e6a86"
bright_foreground = "#575279"
red = "#b4637a"
yellow = "#ea9d34"
orange = "#cf8057"
green = "#286983"
cyan = "#d7827e"
blue = "#56949f"
magenta = "#907aa9"
brown = "#67402b"
bright_red = "#b4637a"
bright_yellow = "#ea9d34"
bright_green = "#286983"
bright_cyan = "#d7827e"
bright_blue = "#56949f"
bright_magenta = "#907aa9"
"##,
        ),
        (
            "solitude",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#798186"
selection = "#343d41"
muted = "#4b4e55"
background = "#101315"
dark_background = "#0c0e10"
darker_background = "#080a0b"
lighter_background = "#101315"
foreground = "#cacccc"
dark_foreground = "#4b4e55"
light_foreground = "#cbc2be"
bright_foreground = "#a5aeb4"
hyprland_active_border = "rgba(798186ee) rgba(caccccee)"
hyprland_inactive_border = "rgb(1e1e1e)"
active_border_color = "#a8adb0"
active_tab_background = "#798186"
red = "#565d60"
yellow = "#d9dbdc"
green = "#9fa5a9"
cyan = "#707070"
blue = "#798186"
magenta = "#aeaeae"
bright_red = "#de6145"
bright_yellow = "#c9c2b4"
bright_green = "#343d41"
bright_cyan = "#707070"
bright_blue = "#5d6367"
bright_magenta = "#9a9a9a"
"##,
        ),
        (
            "starship",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#ff9f0a"
selection = "#3a3a3c"
muted = "#8e8e93"
background = "#1c1c1e"
dark_background = "#161618"
darker_background = "#0e0e10"
lighter_background = "#2c2c2e"
foreground = "#f5f5f7"
dark_foreground = "#98989d"
light_foreground = "#fbfbfd"
bright_foreground = "#ffffff"
red = "#ff5f4a"
yellow = "#ffc27a"
orange = "#ff6a3d"
green = "#ff9f0a"
cyan = "#a6dcef"
blue = "#8fb8de"
magenta = "#ff8fa3"
brown = "#b8a99a"
bright_red = "#ff8373"
bright_yellow = "#ffd8a8"
bright_green = "#ffb340"
bright_cyan = "#cdeefa"
bright_blue = "#b5d0ea"
bright_magenta = "#ffb3c1"
"##,
        ),
        (
            "tokyo-night",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#7aa2f7"
selection = "#292e42"
muted = "#414868"
background = "#1a1b26"
dark_background = "#13141c"
darker_background = "#0e0e14"
lighter_background = "#24283b"
foreground = "#a9b1d6"
dark_foreground = "#565f89"
light_foreground = "#b4bee6"
bright_foreground = "#c0caf5"
red = "#f7768e"
yellow = "#e0af68"
orange = "#eb927b"
green = "#9ece6a"
cyan = "#449dab"
blue = "#7aa2f7"
magenta = "#ad8ee6"
brown = "#75493d"
bright_red = "#ff7a93"
bright_yellow = "#ff9e64"
bright_green = "#b9f27c"
bright_cyan = "#0db9d7"
bright_blue = "#7da6ff"
bright_magenta = "#bb9af7"
"##,
        ),
        (
            "vantablack",
            "omarchy-builtin",
            r##"
mode = "dark"
accent = "#8d8d8d"
selection = "#1a1a1a"
muted = "#7a7a7a"
background = "#000000"
dark_background = "#090909"
darker_background = "#070707"
lighter_background = "#1a1a1a"
foreground = "#ffffff"
dark_foreground = "#505050"
light_foreground = "#ececec"
bright_foreground = "#ffffff"
red = "#a4a4a4"
yellow = "#cecece"
orange = "#b9b9b9"
green = "#b6b6b6"
cyan = "#b0b0b0"
blue = "#8d8d8d"
magenta = "#9b9b9b"
brown = "#5c5c5c"
bright_red = "#a4a4a4"
bright_yellow = "#cecece"
bright_green = "#b6b6b6"
bright_cyan = "#b0b0b0"
bright_blue = "#8d8d8d"
bright_magenta = "#9b9b9b"
"##,
        ),
        (
            "white",
            "omarchy-builtin",
            r##"
mode = "light"
accent = "#6e6e6e"
selection = "#c0c0c0"
muted = "#808080"
background = "#ffffff"
dark_background = "#f5f5f5"
darker_background = "#e8e8e8"
lighter_background = "#c0c0c0"
foreground = "#000000"
dark_foreground = "#c0c0c0"
light_foreground = "#000000"
bright_foreground = "#000000"
red = "#2a2a2a"
yellow = "#4a4a4a"
green = "#3a3a3a"
cyan = "#3e3e3e"
blue = "#1a1a1a"
magenta = "#2e2e2e"
bright_red = "#2a2a2a"
bright_yellow = "#4a4a4a"
bright_green = "#3a3a3a"
bright_cyan = "#3e3e3e"
bright_blue = "#1a1a1a"
bright_magenta = "#2e2e2e"
"##,
        ),
        (
            "aphrody-m3-dark",
            "sovereign-builtin",
            r##"
mode = "dark"
accent = "#a8c7fa"
selection = "#004a77"
muted = "#8e918f"
background = "#131314"
dark_background = "#0e0e0f"
darker_background = "#09090a"
lighter_background = "#1f1f22"
foreground = "#e3e3e3"
dark_foreground = "#8e918f"
light_foreground = "#c4c7c5"
bright_foreground = "#ffffff"
hyprland_active_border = "rgba(a8c7faee) rgba(7cacf8ee) 45deg"
hyprland_inactive_border = "rgba(444746aa)"
red = "#f2b8b5"
yellow = "#fdd663"
orange = "#fcad70"
green = "#6dd58c"
cyan = "#78d9ec"
blue = "#a8c7fa"
magenta = "#d0bcff"
brown = "#9c7a63"
bright_red = "#f9dedc"
bright_yellow = "#feefc3"
bright_green = "#c4eed0"
bright_cyan = "#b4ecff"
bright_blue = "#d3e3fd"
bright_magenta = "#eaddff"
"##,
        ),
        (
            "cachyos-emerald",
            "sovereign-builtin",
            r##"
mode = "dark"
accent = "#00e8c6"
selection = "#163b38"
muted = "#486b68"
background = "#0b1416"
dark_background = "#080f11"
darker_background = "#050a0b"
lighter_background = "#132326"
foreground = "#e0f7f4"
dark_foreground = "#68928e"
light_foreground = "#b5e5df"
bright_foreground = "#f2fffe"
hyprland_active_border = "rgba(00e8c6ee) rgba(00b4d8ee) 45deg"
hyprland_inactive_border = "rgba(234043aa)"
red = "#ff6b81"
yellow = "#ffd166"
orange = "#ff9f43"
green = "#00e8c6"
cyan = "#00b4d8"
blue = "#38bdf8"
magenta = "#a78bfa"
brown = "#6f5e53"
bright_red = "#ff8fa3"
bright_yellow = "#ffe08a"
bright_green = "#5efce4"
bright_cyan = "#48cae4"
bright_blue = "#7dd3fc"
bright_magenta = "#c4b5fd"
"##,
        ),
    ]
}
