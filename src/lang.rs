use colored::Colorize;
use serde_json::Value as Json;
use std::path::{Path, PathBuf, MAIN_SEPARATOR_STR};
use toml::Value as Toml;

use crate::exec::check_command;
use crate::os::{Kind as OSKind, OS};
use crate::read::read_input;
use crate::string::join_text;
use crate::sys::{cwd, dirname, is_dir_has, read_file};

pub static PACKAGE_NAME_PLACEHOLDER: &str = "{{PACKAGE_NAME}}";
pub static MODULE_NAME_PLACEHOLDER: &str = "{{MODULE_NAME}}";

const PYTHON_COMMANDS: &[&str] = &["python", "python3"];
const PYTHON_COMMANDS_WINDOWS: &[&str] = &["py", "python.exe", "python3.exe", "py.exe"];
const PIP_COMMANDS: &[&str] = &["pip", "pip3"];
const PIP_COMMANDS_WINDOWS: &[&str] = &["pip.exe", "pip3.exe"];
const VALID_PYTHON_COMMANDS: &[&str] = &[
    "python",
    "python.exe",
    "python3",
    "python3.exe",
    "py",
    "py.exe",
];
const NPM_COMMANDS: &[&str] = &["npm"];
const NPM_COMMANDS_WINDOWS: &[&str] = &["npm.cmd", "npm.ps1"];
const NPX_COMMANDS: &[&str] = &["npx"];
const NPX_COMMANDS_WINDOWS: &[&str] = &["npx.cmd", "npx.ps1"];

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Kind {
    Go,
    Python,
    TypeScript,
    NodeJS,
    Rust,
    SvelteKit,
    #[default]
    Unknown,
}

#[derive(Debug, Default, Clone)]
pub struct Prop {
    pub project_name: String,
    pub module_name: String,
    pub is_use_vendoring: bool,
    project_dir: PathBuf,
}

#[derive(Debug, Default, Clone)]
pub struct Command {
    pub base: String,
    pub package_manager: String,
    pub module_init: Vec<Vec<String>>,
    pub dependency_install: Vec<Vec<String>>,
    pub vendoring: Vec<Vec<String>>,
    pub build: Vec<Vec<String>>,
    pub running: Vec<Vec<String>>,
    base_custom: String,
    original_base: String,
    package_install: Vec<Vec<String>>,
}

#[derive(Debug, Default, Clone)]
pub struct Lang {
    pub kind: Kind,
    pub name: String,
    pub structure_dir_name: String,
    pub src_dir_name: String,
    pub mod_file_names: Vec<String>,
    pub mod_prefix: String,
    pub extensions: Vec<String>,
    pub command: Command,
    pub prop: Prop,
    os: OS,
}

impl Lang {
    pub fn new(os: OS, name: &str) -> Self {
        let lang_config = match name {
            "go" => (Kind::Go, "Golang", "go", "", vec![], "", vec![".go"]),
            "python" | "py" => (
                Kind::Python,
                "Python",
                "python",
                "",
                vec![],
                "",
                vec![".py"],
            ),
            "typescript" | "ts" => (
                Kind::TypeScript,
                "TypeScript",
                "typescript",
                "",
                vec![],
                "",
                vec![".ts"],
            ),
            "nodejs" | "node" => (
                Kind::NodeJS,
                "NodeJS",
                "nodejs",
                "",
                vec![],
                "",
                vec![".js"],
            ),
            "rust" | "rs" => (
                Kind::Rust,
                "Rust",
                "rust",
                "src",
                vec!["lib.rs", "mod.rs"],
                "pub mod",
                vec![".rs"],
            ),
            "sveltekit" => (
                Kind::SvelteKit,
                "SvelteKit",
                "sveltekit",
                "src",
                vec![],
                "",
                vec![".ts", ".svelte"],
            ),
            _ => return Self::default(),
        };

        Self {
            os,
            kind: lang_config.0,
            name: lang_config.1.to_string(),
            structure_dir_name: lang_config.2.to_string(),
            src_dir_name: lang_config.3.to_string(),
            mod_file_names: lang_config.4.iter().map(|s| s.to_string()).collect(),
            mod_prefix: lang_config.5.to_string(),
            extensions: lang_config.6.iter().map(|s| s.to_string()).collect(),
            ..Self::default()
        }
    }

    pub fn new_from_dir(os: OS, dir: &Path) -> Self {
        if !dir.is_dir() {
            return Self::default();
        }

        if Self::is_python_project(dir) {
            return Self::new(os, "python");
        }

        if Self::is_go_project(dir) {
            return Self::new(os, "go");
        }

        if Self::is_rust_project(dir) {
            return Self::new(os, "rust");
        }

        if Self::is_typescript_project(dir) {
            return Self::new(os, "typescript");
        }

        if Self::is_nodejs_project(dir) {
            return Self::new(os, "nodejs");
        }

        Self::default()
    }

    #[inline]
    fn is_python_project(dir: &Path) -> bool {
        is_dir_has(dir, &[], &["py"])
            || is_dir_has(
                dir,
                &["requirements.txt", "pyproject.toml", "setup.py"],
                &[],
            )
            || is_dir_has(dir, &["main.py", "__init__.py"], &[])
    }

    #[inline]
    fn is_go_project(dir: &Path) -> bool {
        is_dir_has(dir, &[], &["go"]) || is_dir_has(dir, &["go.mod", "go.sum", "main.go"], &[])
    }

    #[inline]
    fn is_rust_project(dir: &Path) -> bool {
        is_dir_has(dir, &[], &["rs"])
            || is_dir_has(dir, &["Cargo.toml"], &[])
            || dir.join("src/main.rs").exists()
    }

    #[inline]
    fn is_typescript_project(dir: &Path) -> bool {
        is_dir_has(dir, &[], &["ts"])
            || is_dir_has(dir, &["tsconfig.json"], &[])
            || (is_dir_has(dir, &["package.json"], &[]) && is_dir_has(dir, &["main.ts"], &[]))
    }

    #[inline]
    fn is_nodejs_project(dir: &Path) -> bool {
        is_dir_has(dir, &[], &["js"]) || is_dir_has(dir, &["package.json", "main.js"], &[])
    }
}

impl Lang {
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.kind != Kind::Unknown
    }

    #[inline]
    pub fn get_main_file_extension(&self) -> String {
        self.extensions.first().cloned().unwrap_or_default()
    }
}

impl Lang {
    #[inline]
    pub fn generate_package_install_commands(&self, packages: &Vec<&str>) -> Vec<Vec<String>> {
        self.command
            .package_install
            .iter()
            .flat_map(|cmd_template| {
                packages.iter().map(move |package| {
                    cmd_template
                        .iter()
                        .map(|c| c.replace(PACKAGE_NAME_PLACEHOLDER, package))
                        .collect()
                })
            })
            .collect()
    }

    #[inline]
    pub fn generate_module_init_commands(&self, module_name: &str) -> Vec<Vec<String>> {
        self.command
            .module_init
            .iter()
            .map(|cmd| {
                cmd.iter()
                    .map(|c| c.replace(MODULE_NAME_PLACEHOLDER, module_name))
                    .collect()
            })
            .collect()
    }
}

impl Lang {
    pub fn compose_prop_from_input(&mut self) {
        self.prop.project_name = self.read_project_name();
        self.prop.project_dir = PathBuf::from(&self.prop.project_name);

        match self.kind {
            Kind::Go => self.compose_go_props(),
            Kind::Python => self.compose_python_props(),
            _ => {}
        }

        self.compose_command();
    }

    fn read_project_name(&self) -> String {
        let mut prompt = "Enter project name (ex: myproject): ";

        loop {
            let name = read_input(prompt);
            if name.is_empty() {
                prompt = "Project name cannot be empty. Enter another name: ";
                continue;
            }
            if name.contains(' ') {
                prompt = "Project name cannot contain space. Enter another name: ";
                continue;
            }
            if Path::new(&name).exists() {
                prompt = "Project already exists. Enter another name: ";
                continue;
            }

            return name;
        }
    }

    fn compose_go_props(&mut self) {
        let mut prompt = "Enter Go module name (ex: github.com/user/project): ";

        loop {
            let module = read_input(prompt);
            if module.is_empty() {
                prompt = "Go module cannot be empty. Enter another name: ";
                continue;
            }

            self.prop.module_name = module;
            break;
        }

        self.prop.is_use_vendoring =
            !self.ask_to_disable_feature("vendoring", "We use vendoring by default");
    }

    fn compose_python_props(&mut self) {
        self.determine_base_command();

        self.prop.is_use_vendoring = !self.ask_to_disable_feature(
            "virtual environment (venv)",
            "We use virtual environment (venv) by default",
        );

        if self.confirm_custom_python_command() {
            self.read_custom_python_command();
        }
    }

    fn ask_to_disable_feature(&self, _feature_name: &str, message: &str) -> bool {
        let response = read_input(&format!(
            "{}, Type '{}' and press {} if you don't want to use it: ",
            message,
            "n".bright_white().bold(),
            "Enter".bright_white().bold()
        ));

        response.trim().eq_ignore_ascii_case("n")
    }

    fn confirm_custom_python_command(&self) -> bool {
        let response = read_input(&format!(
            "\nBy default, we use the '{}' command to run post-installation setup. However, on some systems, this may point to an irrelevant Python version. If you want to use a different command (e.g., 'python3'), type '{}' and press {}. Otherwise, the default will be used: ",
            self.command.base.bright_white(),
            "y".bright_white().bold(),
            "Enter".bright_white().bold()
        ));

        response.trim().eq_ignore_ascii_case("y")
    }

    fn read_custom_python_command(&mut self) {
        let mut prompt = "Enter Python command you want to use: ".to_string();

        loop {
            let cmd = read_input(&prompt).trim().to_string();
            if cmd.is_empty() {
                prompt =
                    "Python command cannot be empty. Please enter a valid command: ".to_string();
                continue;
            }

            if !VALID_PYTHON_COMMANDS.contains(&cmd.as_str()) {
                prompt = format!(
                    "`{}` is not a recognized Python command. Please enter a valid command: ",
                    cmd.bright_red()
                );
                continue;
            }

            if !check_command(&self.prop.project_dir, &cmd) {
                prompt = format!(
                    "`{}` command does not exist on this system. Please enter an existing one: ",
                    cmd.bright_red()
                );
                continue;
            }

            self.command.base = cmd.clone();
            self.command.base_custom = cmd;
            break;
        }
    }

    pub fn compose_prop_from_dir(&mut self, dir: &Path) {
        if !dir.is_dir() {
            self.prop = Prop::default();
            self.compose_command();
            return;
        }

        self.prop.project_dir = dir.to_path_buf();
        self.prop.project_name = dirname(dir);

        match self.kind {
            Kind::Go => self.extract_go_props(dir),
            Kind::Python => self.extract_python_props(dir),
            Kind::TypeScript | Kind::NodeJS | Kind::SvelteKit => {
                self.extract_package_json_props(dir)
            }
            Kind::Rust => self.extract_rust_props(dir),
            _ => {}
        }

        self.compose_command();
    }

    fn extract_go_props(&mut self, dir: &Path) {
        if let Ok(content) = read_file(dir.join("go.mod").as_path()) {
            self.prop.module_name = content
                .lines()
                .find(|l| l.starts_with("module "))
                .and_then(|line| {
                    line.trim_start_matches("module ")
                        .trim()
                        .split_whitespace()
                        .next()
                })
                .unwrap_or("")
                .to_string();
        }

        self.prop.is_use_vendoring = cwd().join("vendor").is_dir();
    }

    fn extract_python_props(&mut self, dir: &Path) {
        if let Ok(content) = read_file(dir.join("pyproject.toml").as_path()) {
            if let Ok(parsed) = content.parse::<Toml>() {
                self.prop.module_name = parsed
                    .get("project")
                    .and_then(|p| p.get("name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
            }
        }

        if self.prop.module_name.is_empty() {
            self.prop.module_name = dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
        }

        self.prop.is_use_vendoring = cwd().join("venv").is_dir();
    }

    fn extract_package_json_props(&mut self, dir: &Path) {
        if let Ok(content) = read_file(dir.join("package.json").as_path()) {
            if let Ok(parsed) = serde_json::from_str::<Json>(&content) {
                self.prop.module_name = parsed
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
            }
        }
    }

    fn extract_rust_props(&mut self, dir: &Path) {
        if let Ok(content) = read_file(dir.join("Cargo.toml").as_path()) {
            if let Ok(parsed) = content.parse::<Toml>() {
                self.prop.module_name = parsed
                    .get("package")
                    .and_then(|p| p.get("name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
            }
        }
    }

    pub fn compose_command(&mut self) {
        self.determine_original_base_command();
        self.determine_base_command();
        self.determine_package_manager_command();
        self.determine_module_init_command();
        self.determine_dependency_install_command();
        self.determine_package_install_command();
        self.determine_vendoring_command();
        self.determine_build_command();
        self.determine_running_command();
    }
}

impl Lang {
    fn determine_original_base_command(&mut self) {
        let candidates: &[&str] = match self.kind {
            Kind::Go => &["go"],
            Kind::Python => {
                if !self.command.base_custom.is_empty() {
                    &[&self.command.base_custom]
                } else if self.os.kind == OSKind::Windows {
                    &[PYTHON_COMMANDS_WINDOWS, PYTHON_COMMANDS].concat()
                } else {
                    PYTHON_COMMANDS
                }
            }
            Kind::NodeJS | Kind::TypeScript | Kind::SvelteKit => &["node"],
            Kind::Rust => &["cargo"],
            _ => &[],
        };

        self.command.original_base = self
            .find_first_available_command(candidates)
            .unwrap_or_default();
    }

    fn determine_base_command(&mut self) {
        let candidates = self.build_base_command_candidates();

        self.command.base = self
            .find_first_available_command_owned(&candidates)
            .unwrap_or_default();
        if self.command.base.is_empty() {
            self.command.base = self.command.original_base.clone();
        }
    }

    fn build_base_command_candidates(&self) -> Vec<String> {
        match self.kind {
            Kind::Go => vec!["go".to_string()],
            Kind::Python => {
                if !self.command.base_custom.is_empty() {
                    self.build_python_command_candidates(
                        &[&self.command.base_custom],
                        &[&self.command.base_custom],
                    )
                } else {
                    self.build_python_command_candidates(PYTHON_COMMANDS, PYTHON_COMMANDS_WINDOWS)
                }
            }
            Kind::NodeJS | Kind::TypeScript | Kind::SvelteKit => vec!["node".to_string()],
            Kind::Rust => vec!["cargo".to_string()],
            _ => vec![],
        }
    }

    fn determine_package_manager_command(&mut self) {
        let candidates = match self.kind {
            Kind::Go => vec!["go".to_string()],
            Kind::Python => {
                self.build_python_command_candidates(PIP_COMMANDS, PIP_COMMANDS_WINDOWS)
            }
            Kind::NodeJS | Kind::TypeScript | Kind::SvelteKit => {
                self.build_common_command_candidates(NPM_COMMANDS, NPM_COMMANDS_WINDOWS)
            }
            Kind::Rust => vec!["cargo".to_string()],
            _ => vec![],
        };

        self.command.package_manager = self
            .find_first_available_command_owned(&candidates)
            .unwrap_or_default();
    }

    fn build_common_command_candidates(
        &self,
        base_cmds: &[&str],
        windows_cmds: &[&str],
    ) -> Vec<String> {
        let mut commands = Vec::new();

        let cmd_list = if self.os.kind == OSKind::Windows {
            windows_cmds.iter().chain(base_cmds.iter())
        } else {
            base_cmds.iter().chain([].iter())
        };

        for cmd in cmd_list {
            commands.push(cmd.to_string());
        }

        commands
    }

    fn build_python_command_candidates(
        &self,
        base_cmds: &[&str],
        windows_cmds: &[&str],
    ) -> Vec<String> {
        let mut commands = Vec::new();

        let cmd_list = if self.os.kind == OSKind::Windows {
            windows_cmds.iter().chain(base_cmds.iter())
        } else {
            base_cmds.iter().chain([].iter())
        };

        for cmd in cmd_list {
            commands.push(if self.prop.is_use_vendoring {
                self.build_venv_command(cmd)
            } else {
                cmd.to_string()
            });
        }

        commands
    }

    #[inline]
    fn build_venv_command(&self, cmd: &str) -> String {
        let prefix = if self.os.kind == OSKind::Windows {
            ["venv", "Scripts", cmd]
        } else {
            ["venv", "bin", cmd]
        };

        join_text(&prefix, MAIN_SEPARATOR_STR)
    }

    fn determine_module_init_command(&mut self) {
        self.command.module_init = match self.kind {
            Kind::Go if !self.command.base.is_empty() => {
                vec![vec![
                    self.command.base.clone(),
                    "mod".to_string(),
                    "init".to_string(),
                    MODULE_NAME_PLACEHOLDER.to_string(),
                ]]
            }
            Kind::Rust if !self.command.package_manager.is_empty() => {
                vec![vec![
                    self.command.package_manager.clone(),
                    "init".to_string(),
                ]]
            }
            _ => vec![],
        };
    }

    fn determine_dependency_install_command(&mut self) {
        self.command.dependency_install = match self.kind {
            Kind::Go => vec![vec![
                self.command.package_manager.clone(),
                "mod".to_string(),
                "tidy".to_string(),
            ]],
            Kind::Python => vec![
                vec![
                    self.command.package_manager.clone(),
                    "install".to_string(),
                    "-r".to_string(),
                    "requirements.txt".to_string(),
                ],
                vec![
                    self.command.package_manager.clone(),
                    "freeze".to_string(),
                    ">".to_string(),
                    "requirements.txt".to_string(),
                ],
            ],
            Kind::NodeJS | Kind::TypeScript | Kind::SvelteKit => {
                vec![vec![
                    self.command.package_manager.clone(),
                    "install".to_string(),
                ]]
            }
            _ => vec![],
        };
    }

    fn determine_package_install_command(&mut self) {
        self.command.package_install = match self.kind {
            Kind::Go => vec![vec![
                self.command.package_manager.clone(),
                "get".to_string(),
                PACKAGE_NAME_PLACEHOLDER.to_string(),
            ]],
            Kind::Python => vec![
                vec![
                    self.command.package_manager.clone(),
                    "install".to_string(),
                    PACKAGE_NAME_PLACEHOLDER.to_string(),
                ],
                vec![
                    self.command.package_manager.clone(),
                    "freeze".to_string(),
                    ">".to_string(),
                    "requirements.txt".to_string(),
                ],
            ],
            Kind::NodeJS | Kind::TypeScript | Kind::SvelteKit => {
                vec![vec![
                    self.command.package_manager.clone(),
                    "install".to_string(),
                    PACKAGE_NAME_PLACEHOLDER.to_string(),
                ]]
            }
            Kind::Rust => vec![vec![
                self.command.package_manager.clone(),
                "add".to_string(),
                PACKAGE_NAME_PLACEHOLDER.to_string(),
            ]],
            _ => vec![],
        };
    }

    fn determine_vendoring_command(&mut self) {
        self.command.vendoring = match self.kind {
            Kind::Go => vec![vec![
                self.command.original_base.clone(),
                "mod".to_string(),
                "vendor".to_string(),
            ]],
            Kind::Python => vec![vec![
                self.command.original_base.clone(),
                "-m".to_string(),
                "venv".to_string(),
                "venv".to_string(),
            ]],
            _ => vec![],
        };
    }

    fn determine_build_command(&mut self) {
        if self.kind == Kind::TypeScript {
            self.determine_typescript_build_command();
        }
    }

    fn determine_typescript_build_command(&mut self) {
        self.command.build = Vec::new();

        if let Some(tsc) = self.find_node_command("tsc") {
            self.command.build.push(vec![tsc]);
        }

        if let Some(tsc_alias) = self.find_node_command("tsc-alias") {
            self.command.build.push(vec![tsc_alias]);
        }
    }

    fn find_node_command(&self, cmd: &str) -> Option<String> {
        let candidates = self.build_node_command_candidates(cmd);

        self.find_first_available_command_owned(&candidates)
    }

    fn build_node_command_candidates(&self, cmd: &str) -> Vec<String> {
        let mut candidates = if self.os.kind == OSKind::Windows {
            vec![
                format!(
                    ".{}node_modules{}.bin{}{}.cmd",
                    MAIN_SEPARATOR_STR, MAIN_SEPARATOR_STR, MAIN_SEPARATOR_STR, cmd
                ),
                format!(
                    ".{}node_modules{}.bin{}{}.ps1",
                    MAIN_SEPARATOR_STR, MAIN_SEPARATOR_STR, MAIN_SEPARATOR_STR, cmd
                ),
            ]
        } else {
            vec![]
        };

        let mut npx_commands = Vec::new();

        for npx_command in self.build_common_command_candidates(NPX_COMMANDS, NPX_COMMANDS_WINDOWS)
        {
            npx_commands.push(format!("{} {}", npx_command, cmd));
        }

        candidates.extend(npx_commands);
        candidates.push(join_text(
            &[".", "node_modules", ".bin", cmd],
            MAIN_SEPARATOR_STR,
        ));
        candidates.push(cmd.to_string());

        candidates
    }

    fn determine_running_command(&mut self) {
        let (base_cmd, args): (&String, Vec<String>) = match self.kind {
            Kind::Go => (
                &self.command.base,
                vec!["run".to_string(), "main.go".to_string()],
            ),
            Kind::Python => (&self.command.base, vec!["main.py".to_string()]),
            Kind::TypeScript => (
                &self.command.base,
                vec![join_text(&[".", "dist", "main.js"], MAIN_SEPARATOR_STR)],
            ),
            Kind::NodeJS => (&self.command.base, vec!["main.js".to_string()]),
            Kind::Rust => (&self.command.package_manager, vec!["run".to_string()]),
            Kind::SvelteKit => (
                &self.command.package_manager,
                vec!["run".to_string(), "dev".to_string()],
            ),
            _ => {
                self.command.running = vec![];
                return;
            }
        };

        if !check_command(&self.prop.project_dir, base_cmd) {
            self.command.running = vec![];
            return;
        }

        let mut cmd_vec = vec![base_cmd.clone()];

        cmd_vec.extend(args);

        self.command.running = vec![cmd_vec];
    }

    #[inline]
    fn find_first_available_command(&self, candidates: &[&str]) -> Option<String> {
        candidates
            .iter()
            .find(|&&cmd| check_command(&self.prop.project_dir, cmd))
            .map(|&cmd| cmd.to_string())
    }

    #[inline]
    fn find_first_available_command_owned(&self, candidates: &[String]) -> Option<String> {
        candidates
            .iter()
            .find(|cmd| check_command(&self.prop.project_dir, cmd))
            .cloned()
    }
}
