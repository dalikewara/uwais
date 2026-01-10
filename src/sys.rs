use colored::{ColoredString, Colorize};
use std::env::{current_dir, current_exe};
use std::fs::{
    copy, create_dir_all, read_dir, read_to_string, remove_dir_all, remove_file as rm_file, write,
    File,
};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");

#[inline]
pub fn filename<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string()
}

#[inline]
pub fn file_stem<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string()
}

pub fn write_file<P: AsRef<Path>>(filepath: P, content: &str) -> Result<(), String> {
    let path = filepath.as_ref();

    write(path, content).map_err(|err| format!("Failed to write {}: {}", path.display(), err))
}

pub fn create_file<P: AsRef<Path>>(filepath: P, content: &str) -> Result<(), String> {
    let filepath = filepath.as_ref();
    if filepath.is_file() {
        return Err(format!("File already exists: {}", filepath.display()));
    }

    let parent = parent_dir(filepath);
    if !parent.is_dir() {
        create_dir(&parent)?;
    }

    write_file(filepath, content)
}

pub fn remove_file<P: AsRef<Path>>(filepath: P) -> Result<(), String> {
    let filepath = filepath.as_ref();
    if !filepath.is_file() {
        return Err(format!("File does not exist: {}", filepath.display()));
    }

    rm_file(filepath).map_err(|err| format!("Failed to remove {}: {}", filepath.display(), err))
}

pub fn remove_paths<P: AsRef<Path>>(paths: Vec<P>) {
    for p in paths {
        let pref = p.as_ref();
        if pref.is_file() {
            let _ = remove_file(pref);
        } else if pref.is_dir() {
            let _ = remove_dir(pref);
        }
    }
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, String> {
    let path = path.as_ref();
    if !path.is_file() {
        return Err(format!("File does not exist: {}", path.display()));
    }

    read_to_string(path).map_err(|err| format!("Failed to read {}: {}", path.display(), err))
}

pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(
    source_path: P,
    destination_path: Q,
) -> Result<(), String> {
    let src = source_path.as_ref();
    let dst = destination_path.as_ref();

    copy(src, dst).map(|_| ()).map_err(|err| {
        format!(
            "Failed to copy {} to {}: {}",
            src.display(),
            dst.display(),
            err
        )
    })
}

#[inline]
pub fn dirname<P: AsRef<Path>>(path: P) -> String {
    filename(path)
}

#[inline]
pub fn parent_dir<P: AsRef<Path>>(path: P) -> PathBuf {
    path.as_ref()
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_default()
}

#[inline]
pub fn cwd() -> PathBuf {
    current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

#[inline]
pub fn ls<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    read_dir(dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect()
}

pub fn create_dir<P: AsRef<Path>>(dir: P) -> Result<(), String> {
    let dir = dir.as_ref();
    if dir.is_dir() {
        return Ok(());
    }
    if dir.exists() {
        return Err(format!(
            "Path exists but is not a directory: {}",
            dir.display()
        ));
    }

    create_dir_all(dir)
        .map_err(|err| format!("Failed to create directory {}: {}", dir.display(), err))
}

pub fn remove_dir<P: AsRef<Path>>(dir: P) -> Result<(), String> {
    let dir = dir.as_ref();
    if !dir.is_dir() {
        return Err(format!("Directory does not exist: {}", dir.display()));
    }

    remove_dir_all(dir)
        .map_err(|err| format!("Failed to remove directory {}: {}", dir.display(), err))
}

#[inline]
pub fn is_current_dir<P: AsRef<Path>>(path: P) -> bool {
    cwd()
        == path
            .as_ref()
            .canonicalize()
            .unwrap_or_else(|_| path.as_ref().to_path_buf())
}

pub fn is_dir_empty<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    if !path.is_dir() {
        return true;
    }

    read_dir(path)
        .map(|mut entries| entries.next().is_none())
        .unwrap_or(true)
}

pub fn is_dir_has<P: AsRef<Path>>(dir: P, files: &[&str], extensions: &[&str]) -> bool {
    let dir = dir.as_ref();

    if !files.iter().all(|f| dir.join(f).exists()) {
        return false;
    }

    if extensions.is_empty() {
        return true;
    }

    if let Ok(entries) = read_dir(dir) {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
                if extensions.contains(&ext) {
                    return true;
                }
            }
        }
    }

    false
}

pub fn remove_file_or_dir<P: AsRef<Path>>(path: P) -> Result<(), String> {
    let path = path.as_ref();
    if path.is_dir() {
        remove_dir(path)
    } else if path.is_file() {
        remove_file(path)
    } else {
        Err("Invalid path".to_string())
    }
}

#[inline]
pub fn app_exe_path() -> PathBuf {
    current_exe().unwrap_or_default()
}

pub fn extract_archive<P: AsRef<Path>, Q: AsRef<Path>>(
    path: P,
    output_dir: Q,
) -> Result<(), String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let mut zip = ZipArchive::new(file).map_err(|e| e.to_string())?;

    zip.extract(output_dir).map_err(|e| e.to_string())
}

#[inline]
pub fn path_to_str<P: AsRef<Path>>(p: P) -> String {
    p.as_ref().to_str().unwrap_or_default().to_owned()
}

#[inline]
pub fn path_to_colored<P: AsRef<Path>>(p: P) -> ColoredString {
    path_to_str(p).bright_blue()
}

#[inline]
pub fn trim_extension<'a>(text: &'a str, extension: &str) -> &'a str {
    text.strip_suffix(extension).unwrap_or(text)
}
