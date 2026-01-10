use colored::{ColoredString, Colorize};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};

use crate::sys::cwd;

const VERSION_FLAG: &str = "--version";
const VERSION_CMD: &str = "version";
const V_FLAG: &str = "-v";

#[inline]
pub fn exec<P: AsRef<Path>>(current_dir: P, command: &[&str]) -> Result<ExitStatus, String> {
    prepare_command(current_dir, command)?
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|err| err.to_string())
}

#[inline]
pub fn exec_no_std_out<P: AsRef<Path>>(
    current_dir: P,
    command: &[&str],
) -> Result<ExitStatus, String> {
    prepare_command(current_dir, command)?
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|err| err.to_string())
}

#[inline]
pub fn exec_spawn<P: AsRef<Path>>(current_dir: P, command: &[&str]) -> Result<Child, String> {
    prepare_command(current_dir, command)?
        .spawn()
        .map_err(|err| err.to_string())
}

pub fn exec_vec_string_command<P: AsRef<Path>>(
    current_dir: P,
    command: &[String],
) -> Result<ExitStatus, String> {
    if command.is_empty() {
        return Err("The command array is empty".to_string());
    }

    let command_ref: Vec<&str> = command.iter().map(|s| s.as_str()).collect();

    exec(current_dir, &command_ref)
}

#[inline]
pub fn vec_string_command_to_colored_string(commands: &[String]) -> ColoredString {
    commands.join(" ").bright_cyan()
}

pub fn check_command<P: AsRef<Path>>(command_dir: P, command: &str) -> bool {
    if command.trim().is_empty() {
        return false;
    }

    let args = match get_command_check_args(command) {
        Some(args) => args,
        None => return false,
    };

    exec_no_std_out(command_dir, &args)
        .map(|status| status.success())
        .unwrap_or(false)
}

fn get_command_check_args(command: &str) -> Option<Vec<&str>> {
    let base_cmd = extract_base_command(command);
    match base_cmd.as_str() {
        "python" | "python3" | "py" | "git" => Some(vec![command, VERSION_FLAG]),
        "pip" | "pip3" => Some(vec![command, VERSION_FLAG]),
        "npm" | "npx" => Some(vec![command, V_FLAG]),
        "node" | "cargo" => Some(vec![command, VERSION_FLAG]),
        "go" => Some(vec![command, VERSION_CMD]),
        "npx tsc" | "tsc" | "npx tsc-alias" | "tsc-alias" => Some(vec![command, VERSION_FLAG]),
        _ => None,
    }
}

fn extract_base_command(command: &str) -> String {
    let mut base = if let Some(last_part) = command.rsplit(&['/', '\\'][..]).next() {
        last_part
    } else {
        command
    };

    if base.ends_with(".exe") {
        base = base.trim_end_matches(".exe");
    } else if base.ends_with(".cmd") {
        base = base.trim_end_matches(".cmd");
    } else if base.ends_with(".ps1") {
        base = base.trim_end_matches(".ps1");
    }

    base.to_string()
}

fn resolve_command_path<P: AsRef<Path>>(current_dir: P, cmd_name: &Path) -> std::path::PathBuf {
    if cmd_name.is_absolute() {
        return cmd_name.to_path_buf();
    }

    if cmd_name.components().count() > 1 {
        let full_path = current_dir.as_ref().join(cmd_name);
        if full_path.exists() {
            return full_path;
        }
    }

    cmd_name.to_path_buf()
}

fn prepare_command<P: AsRef<Path>>(current_dir: P, command: &[&str]) -> Result<Command, String> {
    if command.is_empty() {
        return Err("The command array is empty".to_string());
    }

    let raw_cmd = command[0].trim();
    if raw_cmd.is_empty() {
        return Err("The command name is empty".to_string());
    }

    let cmd_path = Path::new(raw_cmd)
        .components()
        .collect::<std::path::PathBuf>();

    let current_dir = current_dir.as_ref();
    let command_dir = if current_dir.is_dir() {
        current_dir.to_path_buf()
    } else {
        cwd()
    };

    let canonical = command_dir.canonicalize().unwrap_or(command_dir);
    let final_dir = {
        let path_str = canonical.to_string_lossy();
        if path_str.starts_with(r"\\?\") {
            PathBuf::from(&path_str[4..])
        } else {
            canonical
        }
    };

    let resolved_cmd = resolve_command_path(&final_dir, &cmd_path);
    let mut cmd = Command::new(resolved_cmd);

    if command.len() > 1 {
        cmd.args(&command[1..]);
    }

    cmd.current_dir(final_dir);

    Ok(cmd)
}
