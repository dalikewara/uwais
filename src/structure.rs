use include_dir::{include_dir, Dir, DirEntry};
use serde_json::Value as Json;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::lang::Lang;

pub static DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/structure");
pub const EXTENSION_TO_REMOVE: &str = ".txt";
pub const TEMPLATE_PREFIX_FILENAME: &str = "__template";
pub const STRUCTURE_VERSION_TO_REPLACE: &str = "{{STRUCTURE_VERSION}}";
pub const PROJECT_NAME_TO_REPLACE: &str = "{{PROJECT_NAME}}";
pub const MODULE_NAME_TO_REPLACE: &str = "{{MODULE_NAME}}";
pub const LANGUAGE_NAME_TO_REPLACE: &str = "{{LANGUAGE_NAME}}";
pub const LANGUAGE_EXTENSION_TO_REPLACE: &str = "{{LANGUAGE_EXTENSION}}";
pub const VENDORING_SCRIPT_TO_REPLACE: &str = "{{VENDORING_SCRIPT}}";
pub const TEMPLATE_NAME_TO_REPLACE: &str = "{{TEMPLATE_NAME}}";
pub const TEMPLATE_NAME_PASCAL_CASE_TO_REPLACE: &str = "{{TEMPLATE_NAME_PASCAL_CASE}}";

const DEPENDENCY_FILENAME: &str = "dependency.json";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Version {
    #[default]
    V4,
}

impl Version {
    #[inline]
    pub const fn is_valid(self) -> bool {
        matches!(self, Version::V4)
    }

    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Version::V4 => "v4",
        }
    }

    #[inline]
    pub const fn name(self) -> &'static str {
        self.as_str()
    }

    #[inline]
    pub const fn dir_name(self) -> &'static str {
        self.as_str()
    }
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "v4" => Version::V4,
            _ => Version::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Part {
    Common,
    Domain,
    Feature,
    #[default]
    Unknown,
}

impl Part {
    #[inline]
    pub const fn is_valid(self) -> bool {
        !matches!(self, Part::Unknown)
    }

    #[inline]
    pub const fn dir_name(self) -> &'static str {
        match self {
            Part::Common => "common",
            Part::Domain => "domain",
            Part::Feature => "features",
            Part::Unknown => "unknown",
        }
    }

    #[inline]
    pub const fn json_key(self) -> &'static str {
        match self {
            Part::Common => "commons",
            Part::Domain => "domains",
            Part::Feature => "features",
            Part::Unknown => "",
        }
    }
}

impl From<&str> for Part {
    fn from(s: &str) -> Self {
        match s {
            "common" => Part::Common,
            "domain" => Part::Domain,
            "feature" | "features" => Part::Feature,
            _ => Part::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IncludedDir {
    pub dir: &'static Dir<'static>,
}

impl Default for IncludedDir {
    fn default() -> Self {
        Self { dir: &DIR }
    }
}

impl IncludedDir {
    pub fn new(lang: &Lang, version: Version) -> Self {
        let path = Path::new(version.dir_name()).join(&lang.structure_dir_name);

        Self {
            dir: DIR.get_dir(&path).unwrap_or(&DIR),
        }
    }

    pub fn entries(&self) -> Result<Vec<&DirEntry<'_>>, String> {
        let mut entries = Vec::with_capacity(32);

        collect_dir_entries(self.dir, &mut entries);

        Ok(entries)
    }

    pub fn components(&self) -> Result<Vec<std::path::Component<'_>>, String> {
        Ok(self.dir.path().components().collect())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Structure {
    pub version: Version,
    pub lang: Lang,
    pub included_dir: IncludedDir,
    pub command_post_generation: Vec<Vec<String>>,
}

impl Structure {
    pub fn new(version: Version, lang: Lang) -> Self {
        let included_dir = IncludedDir::new(&lang, version);

        Self {
            version,
            lang,
            included_dir,
            command_post_generation: Vec::new(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if !self.version.is_valid() {
            return Err("The structure version is invalid".to_string());
        }
        if !self.lang.is_valid() {
            return Err("The language is invalid".to_string());
        }
        if self.lang.structure_dir_name.is_empty() {
            return Err("The language structure directory name is empty".to_string());
        }

        Ok(())
    }

    pub fn get_entries(&self) -> Result<Vec<DirEntry<'_>>, String> {
        let entries = self.included_dir.entries()?;

        Ok(entries
            .into_iter()
            .filter(|entry| !path_contains_template(entry.path()))
            .cloned()
            .collect())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Template {
    pub version: Version,
    pub part: Part,
    pub lang: Lang,
    pub included_dir: IncludedDir,
}

impl Template {
    pub fn new(version: Version, part: Part, lang: Lang) -> Self {
        let included_dir = IncludedDir::new(&lang, version);

        Self {
            version,
            part,
            lang,
            included_dir,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if !self.version.is_valid() {
            return Err("The template version is invalid".to_string());
        }
        if !self.part.is_valid() {
            return Err("The template part is invalid".to_string());
        }
        if !self.lang.is_valid() {
            return Err("The language is invalid".to_string());
        }
        if self.lang.structure_dir_name.is_empty() {
            return Err("The language structure directory name is empty".to_string());
        }

        Ok(())
    }

    pub fn get_entries(&self) -> Result<Vec<DirEntry<'_>>, String> {
        let part_dir_name = self.part.dir_name();
        let entries = self.included_dir.entries()?;

        Ok(entries
            .into_iter()
            .filter(|entry| {
                let path = entry.path();
                let path_str = path.to_string_lossy();
                if !path_str.contains(TEMPLATE_PREFIX_FILENAME) {
                    return false;
                }

                match self.part {
                    Part::Common | Part::Domain => path_str.contains(part_dir_name),
                    _ => true,
                }
            })
            .cloned()
            .collect())
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Partial {
    pub version: Version,
    pub part: Part,
}

impl Partial {
    #[inline]
    pub const fn new(version: Version, part: Part) -> Self {
        Self { version, part }
    }

    #[inline]
    pub const fn is_valid(self) -> bool {
        self.version.is_valid() && self.part.is_valid()
    }

    #[inline]
    pub fn get_dir(self) -> PathBuf {
        PathBuf::from(self.part.dir_name())
    }

    pub fn collect_feature_dependencies(
        self,
        base_dir: &Path,
        feature_name: &str,
    ) -> Result<(Vec<PathBuf>, Vec<String>), String> {
        if feature_name.trim().is_empty() {
            return Err("Feature name cannot be empty".to_string());
        }

        if !matches!(self.version, Version::V4) {
            return Ok((Vec::new(), Vec::new()));
        }

        let mut all_paths: Vec<PathBuf> = Vec::new();
        let mut all_externals = HashSet::with_capacity(8);
        let mut dependency_queue = Vec::with_capacity(8);

        let initial_dep = base_dir
            .join(self.part.dir_name())
            .join(feature_name)
            .join(DEPENDENCY_FILENAME);

        if initial_dep.exists() {
            dependency_queue.push(initial_dep);
        }

        while let Some(dep_path) = dependency_queue.pop() {
            if let Ok(json) = load_dependency_file(&dep_path) {
                self.process_dep_type(&json, base_dir, Part::Domain, &mut all_paths, None);
                self.process_dep_type(&json, base_dir, Part::Common, &mut all_paths, None);
                self.process_dep_type(
                    &json,
                    base_dir,
                    Part::Feature,
                    &mut all_paths,
                    Some(&mut dependency_queue),
                );

                if let Some(externals) = json.get("externals").and_then(|v| v.as_array()) {
                    all_externals.extend(
                        externals
                            .iter()
                            .filter_map(|e| e.as_str())
                            .map(String::from),
                    );
                }
            }
        }

        Ok((
            all_paths.into_iter().collect(),
            all_externals.into_iter().collect(),
        ))
    }

    fn process_dep_type(
        self,
        json: &Json,
        base_dir: &Path,
        part: Part,
        all_paths: &mut Vec<PathBuf>,
        mut dep_queue: Option<&mut Vec<PathBuf>>,
    ) {
        let Some(items) = json.get(part.json_key()).and_then(|v| v.as_array()) else {
            return;
        };

        if items.is_empty() {
            return;
        }

        let dir_name = part.dir_name();

        for item in items.iter().filter_map(|i| i.as_str()) {
            let path = base_dir.join(dir_name).join(item);

            if all_paths.contains(&path) {
                continue;
            }

            if path.is_file() {
                all_paths.push(path);
            } else if path.is_dir() {
                let queue_ref = match dep_queue.as_mut() {
                    Some(q) => Some(&mut **q),
                    None => None,
                };

                self.handle_dir_dependency(&path, all_paths, queue_ref);
            }
        }
    }

    fn handle_dir_dependency(
        self,
        path: &Path,
        all_paths: &mut Vec<PathBuf>,
        mut dep_queue: Option<&mut Vec<PathBuf>>,
    ) {
        let Ok(entries) = path.read_dir() else {
            return;
        };

        for entry in entries.flatten() {
            let entry_path = entry.path();

            if let Some(file_name) = entry_path.file_name().and_then(|n| n.to_str()) {
                all_paths.push(entry_path.clone());

                if let Some(ref mut queue) = dep_queue {
                    if file_name.eq_ignore_ascii_case(DEPENDENCY_FILENAME) {
                        if !all_paths.contains(&entry_path) {
                            queue.push(entry_path);
                        }
                    }
                }
            }
        }
    }
}

#[inline]
fn path_contains_template(path: &Path) -> bool {
    path.to_string_lossy().contains(TEMPLATE_PREFIX_FILENAME)
}

fn collect_dir_entries<'a>(dir: &'a Dir<'a>, entries: &mut Vec<&'a DirEntry<'a>>) {
    for entry in dir.entries() {
        entries.push(entry);
        if let DirEntry::Dir(sub_dir) = entry {
            collect_dir_entries(sub_dir, entries);
        }
    }
}

fn load_dependency_file(path: &Path) -> Result<Json, String> {
    let file = File::open(path).map_err(|e| format!("Failed to open {}: {}", path.display(), e))?;

    let reader = BufReader::new(file);

    serde_json::from_reader(reader)
        .map_err(|e| format!("Failed to parse JSON at {}: {}", path.display(), e))
}
