use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostEntry {
    pub ip: String,
    pub hostnames: Vec<String>,
    pub comment: Option<String>,
}

pub struct HostsFile {
    path: String,
    entries: Vec<HostEntry>,
}

impl HostsFile {
    /// Create a new HostsFile instance by loading the system's hosts file
    pub fn new() -> io::Result<Self> {
        let path = Self::get_hosts_file_path();
        let entries = Self::parse_hosts_file(&path)?;

        Ok(Self { path, entries })
    }

    /// Get the path to the hosts file based on the operating system
    fn get_hosts_file_path() -> String {
        if cfg!(windows) {
            r"C:\Windows\System32\drivers\etc\hosts".to_string()
        } else {
            // Fallback to Unix-like systems
            "/etc/hosts".to_string()
        }
    }

    /// Parse the hosts file into a vector of HostEntry
    fn parse_hosts_file(path: &str) -> io::Result<Vec<HostEntry>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut entries = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // Split the line by whitespace
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            // The IP address is the first part
            let ip = parts[0].to_string();

            // The rest are hostnames and possibly a comment
            let mut hostnames = Vec::new();
            let mut comment = None;

            for part in &parts[1..] {
                if part.starts_with('#') {
                    // Join the rest as a comment
                    let comment_idx = trimmed.find('#').unwrap();
                    comment = Some(trimmed[comment_idx..].trim().to_string());
                    break;
                }
                hostnames.push(part.to_string());
            }

            if !hostnames.is_empty() {
                entries.push(HostEntry {
                    ip,
                    hostnames,
                    comment,
                });
            }
        }

        Ok(entries)
    }

    /// Get all entries from the hosts file
    pub fn get_entries(&self) -> &[HostEntry] {
        &self.entries
    }

    /// Add a new entry to the hosts file
    pub fn add_entry(
        &mut self,
        ip: &str,
        hostnames: &[&str],
        comment: Option<&str>,
    ) -> io::Result<()> {
        let entry = HostEntry {
            ip: ip.to_string(),
            hostnames: hostnames.iter().map(|&h| h.to_string()).collect(),
            comment: comment.map(|c| c.to_string()),
        };

        // Check if the entry already exists (by hostname)
        for hostname in &entry.hostnames {
            if self.contains_hostname(hostname) {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!("Hostname {} already exists in hosts file", hostname),
                ));
            }
        }

        self.entries.push(entry);
        self.save()
    }

    /// Check if the hosts file contains a specific hostname
    pub fn contains_hostname(&self, hostname: &str) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.hostnames.iter().any(|h| h == hostname))
    }

    /// Save the current entries back to the hosts file
    pub fn save(&self) -> io::Result<()> {
        // Check if we have permission to write to the hosts file
        let can_write = Path::new(&self.path)
            .metadata()
            .map(|m| !m.permissions().readonly())
            .unwrap_or(false);

        if !can_write {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Insufficient permissions to write to hosts file. Try running with admin/sudo privileges.",
            ));
        }

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;

        // Write the standard localhost entries that most hosts files have
        writeln!(file, "127.0.0.1       localhost")?;
        if cfg!(windows) {
            writeln!(file, "::1             localhost")?;
        } else {
            writeln!(file, "::1             localhost ip6-localhost ip6-loopback")?;
        }
        writeln!(file)?;

        // Write all custom entries
        for entry in &self.entries {
            let mut line = format!("{}    {}", entry.ip, entry.hostnames.join(" "));
            if let Some(comment) = &entry.comment {
                line = format!("{}  # {}", line, comment);
            }
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_list_host() {
        assert!(HostsFile::new().is_ok())
    }
}
