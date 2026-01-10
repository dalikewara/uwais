use serde::Deserialize;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use crate::git::{git_download, is_git_common_url, is_git_ssh_url};
use crate::http::{download_file, fetch_json};
use crate::os::{Kind as OSKind, OS};
use crate::sys::{extract_archive, filename, is_dir_empty, ls, remove_file_or_dir};
use crate::time::unix_timestamp;

const MAX_RETRIES: u8 = 3;
const RETRY_DELAY_MS: u64 = 1000;

const TEMP_DIR_PREFIX: &str = ".uwais-tmp";

#[derive(Debug, Deserialize)]
struct LatestAppReleaseDTO {
    assets: Vec<LatestAppReleaseDTOAsset>,
}

#[derive(Debug, Deserialize)]
struct LatestAppReleaseDTOAsset {
    browser_download_url: String,
    name: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Kind {
    LocalPath,
    GitUrl,
    GitSSH,
    LatestAppRelease,
    #[default]
    Unknown,
}

impl Kind {
    #[inline]
    pub const fn is_valid(&self) -> bool {
        !matches!(self, Kind::Unknown)
    }

    #[inline]
    pub const fn requires_network(&self) -> bool {
        matches!(self, Kind::GitUrl | Kind::GitSSH | Kind::LatestAppRelease)
    }

    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Kind::LocalPath => "local-path",
            Kind::GitUrl => "git-url",
            Kind::GitSSH => "git-ssh",
            Kind::LatestAppRelease => "latest-release",
            Kind::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Source {
    pub kind: Kind,
    pub url: String,
    os: OS,
    source_dir: PathBuf,
    cleaned_up: bool,
}

impl Source {
    pub fn new(os: OS, url: &str) -> Self {
        let url = url.trim();
        let kind = if url.is_empty() {
            Kind::Unknown
        } else {
            Self::detect_kind(url)
        };

        Self {
            os,
            kind,
            url: url.to_string(),
            source_dir: PathBuf::new(),
            cleaned_up: false,
        }
    }

    pub fn new_latest_app_release(os: OS) -> Self {
        Self {
            os,
            kind: Kind::LatestAppRelease,
            url: "https://api.github.com/repos/dalikewara/uwais/releases/latest".to_string(),
            source_dir: PathBuf::new(),
            cleaned_up: false,
        }
    }

    fn detect_kind(url: &str) -> Kind {
        if is_git_common_url(url) {
            Kind::GitUrl
        } else if is_git_ssh_url(url) {
            Kind::GitSSH
        } else if url.starts_with('/') || url.starts_with('.') || url.starts_with("~") {
            Kind::LocalPath
        } else {
            Kind::Unknown
        }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.kind.is_valid() && !self.url.trim().is_empty()
    }

    #[inline]
    fn is_from_external(&self) -> bool {
        self.kind.requires_network()
    }

    pub fn provide_dir(&mut self) -> Result<PathBuf, String> {
        match self.kind {
            Kind::LocalPath => self.provide_local(),
            Kind::GitUrl | Kind::GitSSH => self.provide_git(),
            Kind::LatestAppRelease => self.provide_latest_release_dir(),
            _ => Err(format!("Invalid source type: {}", self.kind.name())),
        }
    }

    fn provide_local(&mut self) -> Result<PathBuf, String> {
        let local_path = PathBuf::from(&self.url);
        if !local_path.exists() {
            return Err(format!(
                "Local source does not exist: {}",
                local_path.display()
            ));
        }
        if !local_path.is_dir() {
            return Err(format!(
                "Local source must be a directory: {}",
                local_path.display()
            ));
        }

        self.source_dir = local_path.clone();

        Ok(local_path)
    }

    fn provide_git(&mut self) -> Result<PathBuf, String> {
        let tmp_dir = self.create_temp_dir("source-git");

        match self.git_download_with_retry(&tmp_dir) {
            Ok(_) => {
                if !tmp_dir.is_dir() {
                    self.cleanup_temp_dir(&tmp_dir);
                    return Err("Git source was not downloaded successfully".to_string());
                }

                self.source_dir = tmp_dir.clone();

                Ok(tmp_dir)
            }
            Err(err) => {
                self.cleanup_temp_dir(&tmp_dir);
                Err(format!("Failed to download Git source: {}", err))
            }
        }
    }

    fn git_download_with_retry(&self, output_dir: &PathBuf) -> Result<(), String> {
        let mut last_error = String::new();

        for attempt in 1..=MAX_RETRIES {
            match git_download(&self.url, output_dir) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    last_error = e;
                    if attempt < MAX_RETRIES {
                        let delay = RETRY_DELAY_MS * 2u64.pow((attempt - 1) as u32);
                        thread::sleep(Duration::from_millis(delay));
                    }
                }
            }
        }

        Err(format!(
            "Failed after {} attempts: {}",
            MAX_RETRIES, last_error
        ))
    }

    fn provide_latest_release_dir(&mut self) -> Result<PathBuf, String> {
        let release: LatestAppReleaseDTO = fetch_json(&self.url)
            .map_err(|err| format!("Failed to fetch release info: {}", err))?;
        if release.assets.is_empty() {
            return Err("No release assets were found".to_string());
        }

        let tmp_dir = self.create_temp_dir("source-latest-app-release");

        self.download_release_asset(&release.assets, &tmp_dir)?;

        if is_dir_empty(&tmp_dir) {
            self.cleanup_temp_dir(&tmp_dir);
            return Err("No matching release asset found for the current OS".to_string());
        }

        self.source_dir = tmp_dir.clone();

        Ok(tmp_dir)
    }

    fn download_release_asset(
        &mut self,
        assets: &[LatestAppReleaseDTOAsset],
        output_dir: &PathBuf,
    ) -> Result<(), String> {
        for asset in assets {
            if !self.matches_current_os(&asset.name) {
                continue;
            }

            let output_path = output_dir.join(&asset.name);

            download_file(&asset.browser_download_url, &output_path).map_err(|err| {
                self.cleanup_temp_dir(output_dir);
                format!("Failed to download release asset: {}", err)
            })?;

            if !output_path.is_file() {
                self.cleanup_temp_dir(output_dir);
                return Err(format!(
                    "Downloaded asset not found: {}",
                    output_path.display()
                ));
            }

            return Ok(());
        }

        Err("No matching asset found for the current OS".to_string())
    }

    pub fn provide_latest_app_release(&mut self) -> Result<PathBuf, String> {
        if self.kind != Kind::LatestAppRelease {
            return Err("Invalid source type for latest app release".to_string());
        }

        let tmp_dir = self.provide_dir()?;

        let archive_path = self.find_matching_archive(&tmp_dir)?;
        let binary_path = self.extract_and_locate_binary(&tmp_dir, &archive_path)?;

        Ok(binary_path)
    }

    #[inline]
    fn find_matching_archive(&self, dir: &PathBuf) -> Result<PathBuf, String> {
        ls(dir)
            .into_iter()
            .find(|entry| entry.is_file() && self.matches_current_os(&filename(entry)))
            .ok_or_else(|| "No matching archive found for the current OS".to_string())
    }

    fn extract_and_locate_binary(
        &self,
        tmp_dir: &PathBuf,
        archive_path: &PathBuf,
    ) -> Result<PathBuf, String> {
        extract_archive(archive_path, tmp_dir)
            .map_err(|e| format!("Failed to extract archive: {}", e))?;

        let binary_path = tmp_dir.join(&self.os.app_name);

        if !binary_path.is_file() {
            return Err(format!(
                "Binary not found after extraction: {}",
                binary_path.display()
            ));
        }

        Ok(binary_path)
    }

    #[inline]
    fn matches_current_os(&self, filename: &str) -> bool {
        match self.os.kind {
            OSKind::Windows => filename.ends_with("windows.zip"),
            OSKind::Linux => filename.ends_with("linux.zip"),
            OSKind::MacOS => filename.ends_with("macos.zip"),
            _ => false,
        }
    }

    #[inline]
    fn create_temp_dir(&self, prefix: &str) -> PathBuf {
        PathBuf::from(format!(
            "{}-{}-{}",
            TEMP_DIR_PREFIX,
            prefix,
            unix_timestamp()
        ))
    }

    fn cleanup_temp_dir(&mut self, dir: &PathBuf) {
        if self.source_dir == *dir {
            self.cleaned_up = true;
        }

        let _ = remove_file_or_dir(dir);
    }

    pub fn clear(&mut self) {
        if self.cleaned_up || !self.is_from_external() || !self.source_dir.exists() {
            return;
        }

        if remove_file_or_dir(&self.source_dir).is_ok() {
            self.cleaned_up = true;
        }
    }
}

impl Drop for Source {
    fn drop(&mut self) {
        if !self.cleaned_up && self.is_from_external() && self.source_dir.exists() {
            let _ = remove_file_or_dir(&self.source_dir);
        }
    }
}
