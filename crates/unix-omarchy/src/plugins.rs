use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Plugin I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BuiltinOmarchyPlugin {
    pub slug: &'static str,
    pub id: &'static str,
    pub name: &'static str,
    pub version: &'static str,
    pub kind: &'static str,
    pub entry_point: &'static str,
    pub description: &'static str,
    pub sub_plugins: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplacePluginEntry {
    pub id: String,
    pub repo: String,
    pub source_type: String,
    pub category: String,
    pub branch: String,
    pub commit: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginSearchReport {
    pub builtin_count: usize,
    pub marketplace_source_count: usize,
    pub builtin_matches: Vec<BuiltinOmarchyPlugin>,
    pub marketplace_matches: Vec<MarketplacePluginEntry>,
}

pub struct OmarchyPluginEngine {
    pub builtin_dir: PathBuf,
    pub registry_json_path: PathBuf,
}

impl Default for OmarchyPluginEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl OmarchyPluginEngine {
    pub const DEFAULT_BUILTIN_DIR: &'static str =
        r"C:\src\third_party\omacom\omarchy\shell\plugins";
    pub const DEFAULT_REGISTRY_JSON: &'static str =
        r"C:\src\third_party\omacom\omarchy-plugin-marketplace\registry.json";

    pub fn new() -> Self {
        Self {
            builtin_dir: PathBuf::from(Self::DEFAULT_BUILTIN_DIR),
            registry_json_path: PathBuf::from(Self::DEFAULT_REGISTRY_JSON),
        }
    }

    pub fn with_paths(builtin_dir: PathBuf, registry_json_path: PathBuf) -> Self {
        Self {
            builtin_dir,
            registry_json_path,
        }
    }

    pub fn builtin_plugins() -> [BuiltinOmarchyPlugin; 16] {
        [
            BuiltinOmarchyPlugin {
                slug: "agents",
                id: "omarchy.agents",
                name: "Agents",
                version: "1.0.0",
                kind: "bar-widget",
                entry_point: "Panel.qml",
                description: "Usage, limits, pace, and prepaid balances for AI agents and providers in a native Omarchy bar panel",
                sub_plugins: &[],
            },
            BuiltinOmarchyPlugin {
                slug: "background",
                id: "omarchy.background",
                name: "Background",
                version: "1.0.0",
                kind: "service",
                entry_point: "Background.qml",
                description: "Desktop background renderer with click handling and transitions",
                sub_plugins: &[],
            },
            BuiltinOmarchyPlugin {
                slug: "bar",
                id: "omarchy.bar",
                name: "Bar",
                version: "1.0.0",
                kind: "bar",
                entry_point: "Bar.qml",
                description: "Status bar with workspace, media, network, and AI widgets",
                sub_plugins: &[],
            },
            BuiltinOmarchyPlugin {
                slug: "clipboard",
                id: "omarchy.clipboard",
                name: "Clipboard",
                version: "1.0.0",
                kind: "overlay",
                entry_point: "Clipboard.qml",
                description: "Clipboard manager to view and paste history",
                sub_plugins: &[],
            },
            BuiltinOmarchyPlugin {
                slug: "dev-gallery",
                id: "omarchy.dev-gallery",
                name: "Dev gallery",
                version: "1.0.0",
                kind: "panel",
                entry_point: "GalleryPanel.qml",
                description: "Visual reference for omarchy-shell common UI components",
                sub_plugins: &[],
            },
            BuiltinOmarchyPlugin {
                slug: "dock",
                id: "omarchy.dock",
                name: "Dock",
                version: "1.0.0",
                kind: "service",
                entry_point: "Dock.qml",
                description: "Application dock shown on floating workspaces",
                sub_plugins: &[],
            },
            BuiltinOmarchyPlugin {
                slug: "emojis",
                id: "omarchy.emojis",
                name: "Emojis",
                version: "1.0.0",
                kind: "overlay",
                entry_point: "Emojis.qml",
                description: "Character and symbol search picker overlay",
                sub_plugins: &[],
            },
            BuiltinOmarchyPlugin {
                slug: "image-picker",
                id: "omarchy.image-picker",
                name: "Image picker",
                version: "1.0.0",
                kind: "overlay",
                entry_point: "ImagePicker.qml",
                description: "Image-grid selector overlay used for wallpapers and themes",
                sub_plugins: &[],
            },
            BuiltinOmarchyPlugin {
                slug: "lock",
                id: "omarchy.lock",
                name: "Lock Screen",
                version: "1.0.0",
                kind: "service",
                entry_point: "Service.qml",
                description: "Quickshell session lock with password and PAM authentication flows",
                sub_plugins: &[],
            },
            BuiltinOmarchyPlugin {
                slug: "menu",
                id: "omarchy.menu",
                name: "Omarchy menu",
                version: "1.0.0",
                kind: "menu",
                entry_point: "Menu.qml",
                description: "Quickshell-powered Omarchy root command menu (Super+Space)",
                sub_plugins: &[],
            },
            BuiltinOmarchyPlugin {
                slug: "notifications",
                id: "omarchy.notifications",
                name: "Notifications",
                version: "2.0.0",
                kind: "service",
                entry_point: "Service.qml",
                description: "Notification daemon with grouped stacks, inline actions, Do Not Disturb, and history",
                sub_plugins: &[],
            },
            BuiltinOmarchyPlugin {
                slug: "osd",
                id: "omarchy.osd",
                name: "On-screen display",
                version: "1.0.0",
                kind: "panel",
                entry_point: "Osd.qml",
                description: "Quickshell volume, brightness, and status overlays",
                sub_plugins: &[],
            },
            BuiltinOmarchyPlugin {
                slug: "panels",
                id: "omarchy.panels",
                name: "Panels Suite",
                version: "1.0.0",
                kind: "panel-group",
                entry_point: "panels/*",
                description: "Built-in bar panels: audio, bluetooth, clock, disk-speedtest, power, speedtest, tailscale, weather, wifiqr",
                sub_plugins: &[
                    "omarchy.audio",
                    "omarchy.bluetooth",
                    "omarchy.clock",
                    "omarchy.disk-speedtest",
                    "omarchy.power",
                    "omarchy.speedtest",
                    "omarchy.tailscale",
                    "omarchy.weather",
                    "omarchy.wifiqr",
                ],
            },
            BuiltinOmarchyPlugin {
                slug: "polkit",
                id: "omarchy.polkit",
                name: "Polkit Agent",
                version: "1.0.0",
                kind: "service",
                entry_point: "PolkitAgent.qml",
                description: "Theme-aware authentication dialog for privileged actions",
                sub_plugins: &[],
            },
            BuiltinOmarchyPlugin {
                slug: "reminders",
                id: "omarchy.reminders",
                name: "Reminders",
                version: "1.0.0",
                kind: "overlay",
                entry_point: "ReminderFlow.qml",
                description: "Interactive reminder setup flow",
                sub_plugins: &[],
            },
            BuiltinOmarchyPlugin {
                slug: "services",
                id: "omarchy.services",
                name: "Services Suite",
                version: "1.0.0",
                kind: "service-group",
                entry_point: "services/*",
                description: "Built-in background services: battery, idle, media, nightlight, remote-session",
                sub_plugins: &[
                    "omarchy.battery",
                    "omarchy.idle",
                    "omarchy.media",
                    "omarchy.nightlight",
                    "omarchy.remote-session",
                ],
            },
        ]
    }

    pub fn verify_upstream_builtin_dirs(&self) -> usize {
        if !self.builtin_dir.is_dir() {
            return 0;
        }
        Self::builtin_plugins()
            .iter()
            .filter(|p| self.builtin_dir.join(p.slug).is_dir())
            .count()
    }

    /// Fast streaming scan over Omarchy's 420k-line `registry.json` without materializing a massive DOM AST
    /// unless needed, extracting source count and matching entries up to `limit`.
    pub fn search(
        &self,
        query: Option<&str>,
        limit: usize,
    ) -> Result<PluginSearchReport, PluginError> {
        let q_lower = query.map(|q| q.trim().to_lowercase()).unwrap_or_default();
        let builtins = Self::builtin_plugins();

        let builtin_matches: Vec<BuiltinOmarchyPlugin> = builtins
            .iter()
            .filter(|p| {
                if q_lower.is_empty() {
                    return true;
                }
                p.slug.to_lowercase().contains(&q_lower)
                    || p.id.to_lowercase().contains(&q_lower)
                    || p.name.to_lowercase().contains(&q_lower)
                    || p.description.to_lowercase().contains(&q_lower)
                    || p.kind.to_lowercase().contains(&q_lower)
                    || p.sub_plugins
                        .iter()
                        .any(|sp| sp.to_lowercase().contains(&q_lower))
            })
            .cloned()
            .collect();

        let mut marketplace_source_count = 0usize;
        let mut marketplace_matches = Vec::new();

        if self.registry_json_path.is_file() {
            let file = File::open(&self.registry_json_path)?;
            let reader = BufReader::with_capacity(64 * 1024, file);
            let mut in_sources = false;
            let mut brace_depth: i32 = 0;
            let mut current_obj = String::with_capacity(2048);

            for line_res in reader.lines() {
                let line = line_res?;
                if !in_sources {
                    if line.trim_start().starts_with("\"sources\": [") {
                        in_sources = true;
                    }
                    continue;
                }

                if brace_depth == 0 && line.trim_start().starts_with(']') {
                    break;
                }

                let open_braces = count_json_structural_char(&line, '{');
                let close_braces = count_json_structural_char(&line, '}');

                if brace_depth > 0 || open_braces > 0 {
                    current_obj.push_str(&line);
                    current_obj.push('\n');
                }

                brace_depth += open_braces as i32;
                brace_depth -= close_braces as i32;

                if brace_depth == 0 && !current_obj.is_empty() {
                    marketplace_source_count += 1;

                    let obj_trimmed = current_obj.trim().trim_end_matches(',').trim();
                    let should_parse = if marketplace_matches.len() >= limit {
                        false
                    } else if q_lower.is_empty() {
                        true
                    } else {
                        obj_trimmed.to_lowercase().contains(&q_lower)
                    };

                    if should_parse
                        && let Ok(parsed_entries) = parse_source_object(obj_trimmed)
                    {
                        for entry in parsed_entries {
                            if marketplace_matches.len() >= limit {
                                break;
                            }
                            if q_lower.is_empty() || entry_matches(&entry, &q_lower) {
                                marketplace_matches.push(entry);
                            }
                        }
                    }

                    current_obj.clear();
                }
            }
        }

        Ok(PluginSearchReport {
            builtin_count: builtins.len(),
            marketplace_source_count,
            builtin_matches,
            marketplace_matches,
        })
    }

    pub fn scaffold_plugin(
        target_root: &Path,
        plugin_id: &str,
        display_name: &str,
        kind: &str,
    ) -> Result<Vec<PathBuf>, PluginError> {
        let slug = plugin_id.split('.').next_back().unwrap_or(plugin_id);
        let plugin_dir = target_root.join(slug);
        fs::create_dir_all(&plugin_dir)?;

        let qml_file = match kind {
            "service" => "Service.qml",
            "overlay" => "Overlay.qml",
            "bar-widget" => "BarWidget.qml",
            _ => "Panel.qml",
        };
        let entry_key = match kind {
            "service" => "service",
            "overlay" => "overlay",
            "bar-widget" => "barWidget",
            _ => "panel",
        };

        let mut entry_points = BTreeMap::new();
        entry_points.insert(entry_key, qml_file);

        let manifest = serde_json::json!({
            "schemaVersion": 1,
            "id": plugin_id,
            "name": display_name,
            "version": "0.1.0",
            "author": "Aphrody Sovereign OS",
            "license": "MIT",
            "description": format!("Sovereign Omarchy shell plugin: {}", display_name),
            "kinds": [kind],
            "entryPoints": entry_points,
        });

        let manifest_path = plugin_dir.join("manifest.json");
        fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;

        let qml_path = plugin_dir.join(qml_file);
        let qml_content = format!(
            "import QtQuick\n\
             // Sovereign Omarchy Plugin: {id}\n\
             Item {{\n\
             \x20   id: root\n\
             \x20   property string pluginId: \"{id}\"\n\
             \x20   property string pluginTitle: \"{title}\"\n\
             }}\n",
            id = plugin_id,
            title = display_name
        );
        fs::write(&qml_path, qml_content)?;

        Ok(vec![manifest_path, qml_path])
    }
}

fn count_json_structural_char(line: &str, target: char) -> usize {
    let mut in_str = false;
    let mut escaped = false;
    let mut count = 0;
    for ch in line.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' && in_str {
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_str = !in_str;
            continue;
        }
        if !in_str && ch == target {
            count += 1;
        }
    }
    count
}

#[derive(Deserialize)]
struct RawRegistrySource {
    repo: String,
    #[serde(rename = "type", default)]
    source_type: String,
    #[serde(rename = "listingValidatedBranch", default)]
    branch: String,
    #[serde(rename = "listingValidatedCommit", default)]
    commit: String,
    #[serde(default)]
    catalog: Option<RawCatalogBlock>,
    #[serde(default)]
    plugins: Option<BTreeMap<String, RawPluginBlock>>,
}

#[derive(Deserialize)]
struct RawCatalogBlock {
    #[serde(default)]
    id: String,
    #[serde(default)]
    category: String,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Deserialize)]
struct RawPluginBlock {
    #[serde(default)]
    category: String,
    #[serde(default)]
    tags: Vec<String>,
}

fn parse_source_object(raw_json: &str) -> Result<Vec<MarketplacePluginEntry>, serde_json::Error> {
    let src: RawRegistrySource = serde_json::from_str(raw_json)?;
    let mut results = Vec::new();

    if let Some(cat) = src.catalog {
        let id = if cat.id.is_empty() {
            src.repo
                .trim_end_matches('/')
                .rsplit('/')
                .next()
                .unwrap_or("unknown")
                .to_string()
        } else {
            cat.id
        };
        results.push(MarketplacePluginEntry {
            id,
            repo: src.repo.clone(),
            source_type: src.source_type.clone(),
            category: cat.category,
            branch: src.branch.clone(),
            commit: src.commit.clone(),
            tags: cat.tags,
        });
    }

    if let Some(map) = src.plugins {
        for (plugin_id, meta) in map {
            results.push(MarketplacePluginEntry {
                id: plugin_id,
                repo: src.repo.clone(),
                source_type: src.source_type.clone(),
                category: meta.category,
                branch: src.branch.clone(),
                commit: src.commit.clone(),
                tags: meta.tags,
            });
        }
    }

    if results.is_empty() {
        let fallback_id = src
            .repo
            .trim_end_matches('/')
            .rsplit('/')
            .next()
            .unwrap_or("unknown")
            .to_string();
        results.push(MarketplacePluginEntry {
            id: fallback_id,
            repo: src.repo,
            source_type: src.source_type,
            category: "General".to_string(),
            branch: src.branch,
            commit: src.commit,
            tags: Vec::new(),
        });
    }

    Ok(results)
}

fn entry_matches(entry: &MarketplacePluginEntry, q_lower: &str) -> bool {
    entry.id.to_lowercase().contains(q_lower)
        || entry.repo.to_lowercase().contains(q_lower)
        || entry.category.to_lowercase().contains(q_lower)
        || entry.tags.iter().any(|t| t.to_lowercase().contains(q_lower))
}
