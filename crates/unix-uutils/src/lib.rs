use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UutilsError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unknown utility: {0}")]
    UnknownUtility(String),
    #[error("Target directory is protected system directory: {0}")]
    ProtectedSystemDirectory(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum UutilsCategory {
    Coreutils,
    Findutils,
    Diffutils,
    Sed,
    Procps,
    UtilLinux,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilityInfo {
    pub name: &'static str,
    pub category: UutilsCategory,
    pub upstream_crate: &'static str,
    pub summary: &'static str,
}

pub struct UutilsRegistry {
    utilities: BTreeMap<&'static str, UtilityInfo>,
}

impl Default for UutilsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl UutilsRegistry {
    pub fn new() -> Self {
        let mut map = BTreeMap::new();

        let entries = [
            // coreutils
            ("cat", UutilsCategory::Coreutils, "uutils/coreutils", "Concatenate files and print on the standard output"),
            ("cp", UutilsCategory::Coreutils, "uutils/coreutils", "Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY"),
            ("ls", UutilsCategory::Coreutils, "uutils/coreutils", "List information about the FILEs"),
            ("mkdir", UutilsCategory::Coreutils, "uutils/coreutils", "Create the DIRECTORY(ies), if they do not already exist"),
            ("mv", UutilsCategory::Coreutils, "uutils/coreutils", "Rename SOURCE to DEST, or move SOURCE(s) to DIRECTORY"),
            ("rm", UutilsCategory::Coreutils, "uutils/coreutils", "Remove (unlink) the FILE(s)"),
            ("echo", UutilsCategory::Coreutils, "uutils/coreutils", "Display a line of text"),
            ("touch", UutilsCategory::Coreutils, "uutils/coreutils", "Update the access and modification times of each FILE"),
            ("head", UutilsCategory::Coreutils, "uutils/coreutils", "Output the first part of files"),
            ("tail", UutilsCategory::Coreutils, "uutils/coreutils", "Output the last part of files"),
            ("uname", UutilsCategory::Coreutils, "uutils/coreutils", "Print system information"),
            ("whoami", UutilsCategory::Coreutils, "uutils/coreutils", "Print the current effective user name"),
            ("chmod", UutilsCategory::Coreutils, "uutils/coreutils", "Change file mode bits"),
            ("chown", UutilsCategory::Coreutils, "uutils/coreutils", "Change file owner and group"),
            ("wc", UutilsCategory::Coreutils, "uutils/coreutils", "Print newline, word, and byte counts for each FILE"),
            ("sort", UutilsCategory::Coreutils, "uutils/coreutils", "Sort lines of text files"),
            ("uniq", UutilsCategory::Coreutils, "uutils/coreutils", "Report or omit repeated lines"),
            ("cut", UutilsCategory::Coreutils, "uutils/coreutils", "Remove sections from each line of files"),
            ("tr", UutilsCategory::Coreutils, "uutils/coreutils", "Translate or delete characters"),
            ("tee", UutilsCategory::Coreutils, "uutils/coreutils", "Read from standard input and write to standard output and files"),
            ("date", UutilsCategory::Coreutils, "uutils/coreutils", "Display or set the system date and time"),
            ("env", UutilsCategory::Coreutils, "uutils/coreutils", "Set environment and run command, or print environment"),
            ("pwd", UutilsCategory::Coreutils, "uutils/coreutils", "Print name of current/working directory"),
            ("basename", UutilsCategory::Coreutils, "uutils/coreutils", "Strip directory and suffix from filenames"),
            ("dirname", UutilsCategory::Coreutils, "uutils/coreutils", "Strip last component from file name"),
            ("du", UutilsCategory::Coreutils, "uutils/coreutils", "Estimate file space usage"),
            ("df", UutilsCategory::Coreutils, "uutils/coreutils", "Report file system disk space usage"),
            ("stat", UutilsCategory::Coreutils, "uutils/coreutils", "Display file or file system status"),
            ("sync", UutilsCategory::Coreutils, "uutils/coreutils", "Synchronize cached writes to persistent storage"),
            ("sleep", UutilsCategory::Coreutils, "uutils/coreutils", "Delay for a specified amount of time"),
            ("true", UutilsCategory::Coreutils, "uutils/coreutils", "Return a successful exit status (0)"),
            ("false", UutilsCategory::Coreutils, "uutils/coreutils", "Return an unsuccessful exit status (1)"),
            ("yes", UutilsCategory::Coreutils, "uutils/coreutils", "Output a string repeatedly until killed"),
            ("base64", UutilsCategory::Coreutils, "uutils/coreutils", "Base64 encode or decode FILE, or standard input"),
            ("cksum", UutilsCategory::Coreutils, "uutils/coreutils", "Print CRC checksum and byte counts"),
            ("md5sum", UutilsCategory::Coreutils, "uutils/coreutils", "Compute and check MD5 message digest"),
            ("sha256sum", UutilsCategory::Coreutils, "uutils/coreutils", "Compute and check SHA256 message digest"),
            ("split", UutilsCategory::Coreutils, "uutils/coreutils", "Split a file into pieces"),
            ("seq", UutilsCategory::Coreutils, "uutils/coreutils", "Print numbers from FIRST to LAST, in steps of INCREMENT"),
            ("mktemp", UutilsCategory::Coreutils, "uutils/coreutils", "Create a temporary file or directory"),
            ("readlink", UutilsCategory::Coreutils, "uutils/coreutils", "Print value of a symbolic link or canonical file name"),
            ("realpath", UutilsCategory::Coreutils, "uutils/coreutils", "Print the resolved path"),

            // findutils
            ("find", UutilsCategory::Findutils, "uutils/findutils", "Search for files in a directory hierarchy"),
            ("xargs", UutilsCategory::Findutils, "uutils/findutils", "Build and execute command lines from standard input"),
            ("locate", UutilsCategory::Findutils, "uutils/findutils", "Find files by name using indexed database"),

            // diffutils
            ("diff", UutilsCategory::Diffutils, "uutils/diffutils", "Compare files line by line"),
            ("cmp", UutilsCategory::Diffutils, "uutils/diffutils", "Compare two files byte by byte"),
            ("sdiff", UutilsCategory::Diffutils, "uutils/diffutils", "Side-by-side merge of file differences"),

            // sed
            ("sed", UutilsCategory::Sed, "uutils/sed", "Stream editor for filtering and transforming text"),

            // procps
            ("ps", UutilsCategory::Procps, "uutils/procps", "Report a snapshot of current processes"),
            ("top", UutilsCategory::Procps, "uutils/procps", "Display dynamic real-time view of running system"),
            ("kill", UutilsCategory::Procps, "uutils/procps", "Send signal to process or kill process"),
            ("free", UutilsCategory::Procps, "uutils/procps", "Display amount of free and used memory in the system"),
            ("uptime", UutilsCategory::Procps, "uutils/procps", "Tell how long the system has been running"),
            ("pgrep", UutilsCategory::Procps, "uutils/procps", "Look up processes based on name and other attributes"),
            ("pkill", UutilsCategory::Procps, "uutils/procps", "Signal processes based on name and other attributes"),
            ("vmstat", UutilsCategory::Procps, "uutils/procps", "Report virtual memory statistics"),
            ("sysctl", UutilsCategory::Procps, "uutils/procps", "Configure kernel parameters at runtime"),

            // util-linux
            ("dmesg", UutilsCategory::UtilLinux, "uutils/util-linux", "Print or control the kernel ring buffer"),
            ("blkid", UutilsCategory::UtilLinux, "uutils/util-linux", "Locate/print block device attributes"),
            ("lscpu", UutilsCategory::UtilLinux, "uutils/util-linux", "Display information about the CPU architecture"),
            ("lsblk", UutilsCategory::UtilLinux, "uutils/util-linux", "List information about all available block devices"),
            ("mount", UutilsCategory::UtilLinux, "uutils/util-linux", "Mount a filesystem"),
            ("umount", UutilsCategory::UtilLinux, "uutils/util-linux", "Unmount a filesystem"),
            ("fdisk", UutilsCategory::UtilLinux, "uutils/util-linux", "Manipulate disk partition table"),
            ("swapon", UutilsCategory::UtilLinux, "uutils/util-linux", "Enable devices and files for paging and swapping"),
            ("swapoff", UutilsCategory::UtilLinux, "uutils/util-linux", "Disable devices and files for paging and swapping"),
            ("ionice", UutilsCategory::UtilLinux, "uutils/util-linux", "Set or get process I/O scheduling class and priority"),
            ("taskset", UutilsCategory::UtilLinux, "uutils/util-linux", "Set or retrieve a process's CPU affinity"),
            ("chcpu", UutilsCategory::UtilLinux, "uutils/util-linux", "Configure CPU state (enable/disable/rescan)"),
            ("setarch", UutilsCategory::UtilLinux, "uutils/util-linux", "Change reported architecture in new program environment"),
            ("unshare", UutilsCategory::UtilLinux, "uutils/util-linux", "Run program with some namespaces unshared from parent"),
            ("nsenter", UutilsCategory::UtilLinux, "uutils/util-linux", "Run program with namespaces of other processes"),
        ];

        for (name, category, upstream_crate, summary) in entries {
            map.insert(
                name,
                UtilityInfo {
                    name,
                    category,
                    upstream_crate,
                    summary,
                },
            );
        }

        Self { utilities: map }
    }

    pub fn get(&self, name: &str) -> Option<&UtilityInfo> {
        self.utilities.get(name)
    }

    pub fn list(&self) -> Vec<&UtilityInfo> {
        self.utilities.values().collect()
    }

    pub fn count_by_category(&self) -> BTreeMap<UutilsCategory, usize> {
        let mut counts = BTreeMap::new();
        for info in self.utilities.values() {
            *counts.entry(info.category).or_insert(0) += 1;
        }
        counts
    }

    pub fn total_count(&self) -> usize {
        self.utilities.len()
    }

    /// Install isolated shims into the target directory.
    /// NEVER touches Windows System directories!
    pub fn install_shims(&self, bin_dir: &Path) -> Result<usize, UutilsError> {
        let canonical_str = bin_dir.to_string_lossy().to_lowercase();
        if canonical_str.contains("windows\\system32")
            || canonical_str.contains("windows/system32")
            || canonical_str.contains("windows\\syswow64")
        {
            return Err(UutilsError::ProtectedSystemDirectory(
                bin_dir.display().to_string(),
            ));
        }

        fs::create_dir_all(bin_dir)?;

        let mut count = 0;
        for (name, _) in &self.utilities {
            // Windows CMD wrapper shim
            let cmd_path = bin_dir.join(format!("{}.cmd", name));
            let cmd_content = format!(
                "@echo off\r\nREM Sovereign uutils dispatcher shim for {}\r\nunix-distro uutils-dispatch {} %*\r\n",
                name, name
            );
            fs::write(&cmd_path, cmd_content)?;

            // Unix shell shim (for WSL / Git Bash / MSYS)
            let sh_path = bin_dir.join(name);
            let sh_content = format!(
                "#!/bin/sh\n# Sovereign uutils dispatcher shim for {}\nexec unix-distro uutils-dispatch {} \"$@\"\n",
                name, name
            );
            fs::write(&sh_path, sh_content)?;

            count += 1;
        }

        Ok(count)
    }

    /// Generate shell environment activation scripts
    pub fn generate_profile_scripts(&self, bin_dir: &Path) -> (String, String) {
        let win_path = bin_dir.to_string_lossy();
        let ps1_script = format!(
            "# Sovereign GNU-in-Rust uutils Environment Profile\r\n$env:PATH = \"{};$env:PATH\"\r\nWrite-Host \"[uutils] All-GNU-in-Rust initialized: {} utilities active.\" -ForegroundColor Green\r\n",
            win_path,
            self.total_count()
        );

        let unix_style_path = win_path.replace('\\', "/").replace("C:", "/c");
        let sh_script = format!(
            "# Sovereign GNU-in-Rust uutils Environment Profile\nexport PATH=\"{}:$PATH\"\necho \"[uutils] All-GNU-in-Rust initialized: {} utilities active.\"\n",
            unix_style_path,
            self.total_count()
        );

        (ps1_script, sh_script)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_contains_all_categories() {
        let reg = UutilsRegistry::new();
        let counts = reg.count_by_category();
        assert!(counts.contains_key(&UutilsCategory::Coreutils));
        assert!(counts.contains_key(&UutilsCategory::Findutils));
        assert!(counts.contains_key(&UutilsCategory::Diffutils));
        assert!(counts.contains_key(&UutilsCategory::Sed));
        assert!(counts.contains_key(&UutilsCategory::Procps));
        assert!(counts.contains_key(&UutilsCategory::UtilLinux));
        assert!(reg.total_count() >= 50);
    }

    #[test]
    fn test_safety_check_against_system32() {
        let reg = UutilsRegistry::new();
        let res = reg.install_shims(Path::new("C:\\Windows\\System32"));
        assert!(res.is_err());
    }

    #[test]
    fn test_shim_installation_in_isolated_dir() {
        let reg = UutilsRegistry::new();
        let temp_dir = std::env::temp_dir().join("sovereign_uutils_test_bin");
        let count = reg.install_shims(&temp_dir).unwrap();
        assert_eq!(count, reg.total_count());

        let ls_cmd = temp_dir.join("ls.cmd");
        assert!(ls_cmd.exists());
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
