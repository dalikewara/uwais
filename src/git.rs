use std::path::Path;

use crate::exec::{check_command, exec};
use crate::http::download_file;
use crate::sys::{cwd, dirname, extract_archive, path_to_colored};

const GITHUB_DOMAIN: &str = "github.com";
const DEFAULT_BRANCHES: &[&str] = &["master", "main"];

#[inline]
pub fn is_git_common_url(url: &str) -> bool {
    (url.starts_with("https://") || url.starts_with("http://")) && url.ends_with(".git")
}

#[inline]
pub fn is_git_ssh_url(url: &str) -> bool {
    url.starts_with("git@") && url.ends_with(".git")
}

#[inline]
pub fn is_valid_git_url(url: &str) -> bool {
    is_git_common_url(url) || is_git_ssh_url(url)
}

pub fn git_clone<P: AsRef<Path>>(url: &str, output_dir: P) -> Result<(), String> {
    if !is_valid_git_url(url) {
        return Err(format!("Invalid Git URL: {}", url));
    }

    let output_dir = output_dir.as_ref();
    let output_dir_name = dirname(output_dir);

    if output_dir_name.is_empty() {
        return Err("Git repository output path cannot be empty".to_string());
    }

    if !check_command(cwd(), "git") {
        return Err("Git is not installed or is not available in PATH".to_string());
    }

    exec(cwd(), &["git", "clone", url, &output_dir_name]).map_err(|err| {
        format!(
            "Failed to clone Git repository from {}: {}",
            path_to_colored(url),
            err
        )
    })?;

    if !output_dir.is_dir() {
        return Err(format!(
            "Git repository was not cloned successfully to {}",
            path_to_colored(output_dir)
        ));
    }

    Ok(())
}

pub fn git_download<P: AsRef<Path>>(url: &str, output_dir: P) -> Result<(), String> {
    let output_dir = output_dir.as_ref();

    if let Ok(()) = try_archive_download(url, output_dir) {
        return Ok(());
    }

    git_clone(url, output_dir)
}

fn try_archive_download<P: AsRef<Path>>(url: &str, output_dir: P) -> Result<(), String> {
    if !url.contains(GITHUB_DOMAIN) {
        return Err("Archive download is only supported for GitHub repositories".to_string());
    }

    let output_dir = output_dir.as_ref();
    let archive_path = output_dir.join(".uwais-tmp-downloaded-repo.zip");

    for branch in DEFAULT_BRANCHES {
        let archive_url = format_github_archive_url(url, branch);
        if archive_url.is_empty() {
            continue;
        }

        if download_file(&archive_url, &archive_path).is_err() {
            continue;
        }

        if !archive_path.is_file() {
            continue;
        }

        let extraction_result = extract_archive(&archive_path, output_dir);
        let _ = std::fs::remove_file(&archive_path);

        if extraction_result.is_ok() {
            return Ok(());
        }
    }

    Err("Failed to download and extract archive from all attempted branches".to_string())
}

fn format_github_archive_url(git_url: &str, branch: &str) -> String {
    if !git_url.contains(GITHUB_DOMAIN) {
        return String::new();
    }

    let base_url = git_url.trim_end_matches('/').trim_end_matches(".git");

    format!("{}/archive/refs/heads/{}.zip", base_url, branch)
}
