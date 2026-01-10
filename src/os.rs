use std::path::PathBuf;

use crate::sys::{app_exe_path, filename, parent_dir};

const UPDATER_TASK_PREFIX: &str = "latest-";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Windows,
    Linux,
    MacOS,
    #[default]
    Unknown,
}

impl Kind {
    pub const fn detect() -> Self {
        if cfg!(target_os = "windows") {
            Kind::Windows
        } else if cfg!(target_os = "linux") {
            Kind::Linux
        } else if cfg!(target_os = "macos") {
            Kind::MacOS
        } else {
            Kind::Unknown
        }
    }

    #[inline]
    pub const fn is_supported(self) -> bool {
        !matches!(self, Kind::Unknown)
    }
}

#[derive(Debug, Default, Clone)]
pub struct OS {
    pub kind: Kind,
    pub app_name: String,
    pub app_path: PathBuf,
    pub app_dir: PathBuf,
    pub app_updater_task_filename_prefix: String,
}

impl OS {
    pub fn new() -> Self {
        let kind = Kind::detect();
        let app_path = app_exe_path();
        let app_name = filename(&app_path);
        let app_dir = parent_dir(&app_path);

        Self {
            kind,
            app_path: if app_path.as_os_str().is_empty() {
                PathBuf::new()
            } else {
                app_path
            },
            app_name,
            app_dir,
            app_updater_task_filename_prefix: UPDATER_TASK_PREFIX.to_string(),
        }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.kind.is_supported()
            && !self.app_name.is_empty()
            && self.app_path.exists()
            && self.app_dir.exists()
    }

    pub fn get_app_updater_task_filepath_from_main_process(&self) -> PathBuf {
        if self.app_name.is_empty() || self.app_dir.as_os_str().is_empty() {
            return PathBuf::new();
        }

        self.app_dir.join(format!(
            "{}{}",
            self.app_updater_task_filename_prefix, self.app_name
        ))
    }

    pub fn get_app_filepath_from_updater_task_child_process(&self) -> PathBuf {
        if self.app_name.is_empty() || self.app_dir.as_os_str().is_empty() {
            return PathBuf::new();
        }

        let stripped_name = self
            .app_name
            .strip_prefix(&self.app_updater_task_filename_prefix)
            .unwrap_or(&self.app_name);

        self.app_dir.join(stripped_name)
    }
}
