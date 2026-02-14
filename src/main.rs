use colored::Colorize;
use include_dir::DirEntry;
use std::path::{Component, PathBuf, MAIN_SEPARATOR_STR};
use std::process::exit;
use std::thread;
use std::time::Duration;

mod command;
mod exec;
mod git;
mod http;
mod lang;
mod os;
mod print;
mod read;
mod source;
mod string;
mod structure;
mod sys;
mod time;

use crate::command::Command;
use crate::exec::{exec_spawn, exec_vec_string_command, vec_string_command_to_colored_string};
use crate::lang::{Kind as LangKind, Lang};
use crate::os::{Kind as OSKind, OS};
use crate::print::{
    print_done, print_err, print_err_with_info, print_help, print_info, print_text, print_warn,
    print_warn_with_info,
};
use crate::read::{read_args, read_input};
use crate::source::Source;
use crate::string::{to_pascal_case, trim_newline};
use crate::structure::{
    Part as StructurePart, Partial as StructurePartial, Structure, Template, EXTENSION_TO_REMOVE,
    LANGUAGE_EXTENSION_TO_REPLACE, LANGUAGE_NAME_TO_REPLACE, MODULE_NAME_TO_REPLACE,
    PROJECT_NAME_TO_REPLACE, STRUCTURE_VERSION_TO_REPLACE, TEMPLATE_NAME_PASCAL_CASE_TO_REPLACE,
    TEMPLATE_NAME_TO_REPLACE, TEMPLATE_PREFIX_FILENAME, VENDORING_SCRIPT_TO_REPLACE,
};
use crate::sys::{
    copy_file, create_dir, create_file, cwd, dirname, file_stem, filename, parent_dir,
    path_to_colored, path_to_str, read_file, remove_dir, remove_file, remove_paths, trim_extension,
    write_file, VERSION,
};

fn main() {
    let os = OS::new();
    if !os.is_valid() {
        print_err("The current OS is not supported");
        exit(1);
    }

    let args = read_args();

    let command_args = if args.len() > 1 { &args[1..] } else { &[] };
    let command = Command::new(os.clone(), command_args);

    match command {
        Command::Help => print_help(),
        Command::Version => version(),
        Command::Update(mut source) => handle_update(os, &mut source),
        Command::UpdaterTask => handle_updater_task(os),
        Command::UpdaterTaskClearance => handle_updater_task_clearance(os),
        Command::Import(mut partial, mut source, names) => {
            handle_import(os, &mut partial, &mut source, &names)
        }
        Command::Add(template, names) => handle_add(os, template, &names),
        Command::Generator(structure) => handle_generator(structure),
        Command::Unknown => {
            print_err("The command is invalid");
            exit(1);
        }
    }
}

#[inline]
fn version() {
    println!("{}", VERSION);
}

fn handle_update(os: OS, source: &mut Source) {
    if !validate_source(source, "Checking latest Uwais source") {
        exit(1);
    }

    let source_filepath = match download_latest_release(source) {
        Ok(path) => path,
        Err(_) => exit(1),
    };

    update_binary(os, &source_filepath, source);
    print_done("Updating Uwais");
    source.clear();
}

fn validate_source(source: &Source, text: &str) -> bool {
    print_info(text);

    if !source.is_valid() {
        print_err_with_info(text, "The latest Uwais source is currently not available");
        return false;
    }

    true
}

fn download_latest_release(source: &mut Source) -> Result<PathBuf, ()> {
    let std_text = "Getting the latest update";

    print_info(std_text);

    match source.provide_latest_app_release() {
        Ok(filepath) => Ok(filepath),
        Err(err) => {
            print_err_with_info(std_text, &err);
            source.clear();
            Err(())
        }
    }
}

fn update_binary(os: OS, source_filepath: &PathBuf, source: &mut Source) {
    let std_text = "Updating Uwais";

    print_info(std_text);

    match os.kind {
        OSKind::Windows => update_windows_binary(os, source_filepath, std_text, source),
        OSKind::Linux | OSKind::MacOS => update_unix_binary(os, source_filepath, std_text, source),
        _ => {
            source.clear();
            exit(1);
        }
    }
}

fn update_windows_binary(os: OS, source_filepath: &PathBuf, std_text: &str, source: &mut Source) {
    let updater_task_filepath = os.get_app_updater_task_filepath_from_main_process();

    if let Err(err) = copy_file(source_filepath, &updater_task_filepath) {
        let err_string = err.to_string();

        if err_string.contains("os error 5") || err_string.contains("Access is denied") {
            print_err("Access denied - Administrator privileges required");
            print_warn("Please run this command as Administrator:");
            print_text("");
            print_text("  1. Open Command Prompt or PowerShell as Administrator");
            print_text("  2. Run: uwais update");
            print_text("");
            print_text("Alternatively, run the installer again to update:");
            print_text("");
            print_text("  Download the latest installer from:");
            print_text("  https://github.com/dalikewara/uwais/releases/latest");
        } else {
            print_err_with_info(std_text, &err);
        }

        source.clear();
        exit(1);
    }

    if !updater_task_filepath.is_file() {
        print_done("No latest update available");
        source.clear();
        exit(0);
    }

    let _ = exec_spawn(
        cwd(),
        &[
            path_to_str(&updater_task_filepath).as_str(),
            "--updater-task",
        ],
    );
}

fn update_unix_binary(os: OS, source_filepath: &PathBuf, std_text: &str, source: &mut Source) {
    let updater_task_filepath = os.get_app_updater_task_filepath_from_main_process();

    if let Err(err) = copy_file(source_filepath, &updater_task_filepath) {
        let err_string = err.to_string();

        if err_string.contains("Permission denied") || err_string.contains("os error 13") {
            print_err("Permission denied - Elevated privileges required");
            print_warn("Please run this command with sudo:");
            print_text("");
            print_text(&format!("  sudo uwais update"));
            print_text("");
            print_text("Or reinstall using the installation script:");
            print_text("");
            print_text("  curl -sSL https://raw.githubusercontent.com/dalikewara/uwais/master/install.sh | sh");
        } else {
            print_err_with_info(std_text, &err);
        }

        source.clear();
        exit(1);
    }

    if !updater_task_filepath.is_file() {
        print_done("No latest update available");
        source.clear();
        exit(0);
    }

    let _ = exec_spawn(
        cwd(),
        &[
            path_to_str(&updater_task_filepath).as_str(),
            "--updater-task",
        ],
    );
}

fn handle_updater_task(os: OS) {
    if !os
        .app_name
        .starts_with(os.app_updater_task_filename_prefix.as_str())
    {
        return;
    }

    loop {
        match os.kind {
            OSKind::Windows | OSKind::Linux | OSKind::MacOS => {
                let target_app_filepath = os.get_app_filepath_from_updater_task_child_process();

                if copy_file(&os.app_path, &target_app_filepath).is_ok() {
                    let _ = exec_spawn(
                        cwd(),
                        &[
                            path_to_str(&target_app_filepath).as_str(),
                            "--updater-task-clearance",
                        ],
                    );
                    break;
                }

                thread::sleep(Duration::from_millis(100));
            }
            _ => break,
        }
    }
}

fn handle_updater_task_clearance(os: OS) {
    if os
        .app_name
        .starts_with(os.app_updater_task_filename_prefix.as_str())
    {
        return;
    }

    loop {
        match os.kind {
            OSKind::Windows | OSKind::Linux | OSKind::MacOS => {
                let updater_task_filepath = os.get_app_updater_task_filepath_from_main_process();
                if !updater_task_filepath.is_file() {
                    return;
                }

                let _ = remove_file(&updater_task_filepath);
                break;
            }
            _ => break,
        }
    }
}

fn handle_import(os: OS, partial: &mut StructurePartial, source: &mut Source, names: &[String]) {
    if names.is_empty() {
        print_done("Nothing to be imported");
        return;
    }

    let current_dir = cwd();

    let mut current_lang = Lang::new_from_dir(os.clone(), current_dir.as_path());
    if !current_lang.is_valid() {
        print_err("The current project language is invalid");
        return;
    }

    if !validate_import_structure(partial) {
        return;
    }

    let source_dir = match provide_and_validate_source(source) {
        Some(dir) => dir,
        None => return,
    };

    let mut source_lang = Lang::new_from_dir(os, source_dir.as_path());

    if !validate_source_lang(&source_lang, &current_lang, source) {
        return;
    }

    let (source_partial_dir, current_partial_dir) = get_partial_directories(
        partial,
        &source_dir,
        &source_lang,
        &current_dir,
        &current_lang,
    );

    if !validate_partial_directories(&source_partial_dir, &current_partial_dir, partial, source) {
        return;
    }

    current_lang.compose_prop_from_dir(current_dir.as_path());
    source_lang.compose_prop_from_dir(source_dir.as_path());

    let std_text = "Importing";

    print_info(std_text);

    let mut commands_to_exec: Vec<Vec<String>> = Vec::new();

    for imported_name in names {
        if imported_name.is_empty() {
            print_warn_with_info("Empty imported name", "Skipping");
            continue;
        }

        process_import_entry(
            partial,
            &source_lang,
            &current_lang,
            &source_partial_dir,
            &current_partial_dir,
            imported_name,
            &mut commands_to_exec,
        );
    }

    execute_commands(&current_partial_dir, commands_to_exec);
    source.clear();
    print_done(std_text);
}

fn validate_import_structure(partial: &StructurePartial) -> bool {
    let mut std_text = "Checking structure version";

    print_info(std_text);

    if !partial.version.is_valid() {
        print_err_with_info(std_text, "The structure version is invalid");
        return false;
    }

    std_text = "Checking structure part";

    print_info(std_text);

    if !partial.is_valid() || !partial.part.is_valid() {
        print_err_with_info(std_text, "The structure part is invalid");
        return false;
    }

    true
}

fn provide_and_validate_source(source: &mut Source) -> Option<PathBuf> {
    let std_text = "Checking source";

    print_info(std_text);

    if !source.is_valid() {
        print_err_with_info(std_text, "The source is invalid");
        return None;
    }

    let std_text = "Providing source";

    print_info(std_text);

    let source_dir = match source.provide_dir() {
        Ok(dir) => dir,
        Err(err) => {
            print_err_with_info(std_text, err.as_str());
            source.clear();
            return None;
        }
    };

    let std_text = "Validating source";

    print_info(std_text);

    if !source_dir.is_dir() {
        print_err_with_info(std_text, "The source was not provided correctly");
        source.clear();
        return None;
    }

    Some(source_dir)
}

fn validate_source_lang(source_lang: &Lang, current_lang: &Lang, source: &mut Source) -> bool {
    let std_text = "Validating source";

    if !source_lang.is_valid() {
        print_err_with_info(std_text, "The source project language is invalid");
        source.clear();
        return false;
    }

    if current_lang.kind != source_lang.kind {
        print_err_with_info(
            std_text,
            "The language of the current project and the source project do not match",
        );
        source.clear();
        return false;
    }

    true
}

fn get_partial_directories(
    partial: &StructurePartial,
    source_dir: &PathBuf,
    source_lang: &Lang,
    current_dir: &PathBuf,
    current_lang: &Lang,
) -> (PathBuf, PathBuf) {
    let partial_dir = partial.get_dir();
    let source_src_dir = source_dir.join(&source_lang.src_dir_name);
    let source_partial_dir = source_src_dir.join(&partial_dir);
    let current_src_dir = current_dir.join(&current_lang.src_dir_name);
    let current_partial_dir = current_src_dir.join(&partial_dir);

    (source_partial_dir, current_partial_dir)
}

fn validate_partial_directories(
    source_partial_dir: &PathBuf,
    current_partial_dir: &PathBuf,
    partial: &StructurePartial,
    source: &mut Source,
) -> bool {
    let std_text = "Validating source";
    let partial_dir = partial.get_dir();

    if !source_partial_dir.is_dir() {
        print_err_with_info(
            std_text,
            "The source does not follow the Uwais project structure",
        );
        print_err_with_info(
            std_text,
            &format!(
                "Expected `{}` directory to exist inside the given source project",
                path_to_colored(partial_dir.to_str().unwrap_or_default())
            ),
        );
        source.clear();
        return false;
    }

    if !current_partial_dir.is_dir() {
        print_err_with_info(
            std_text,
            "The current project does not follow the Uwais project structure",
        );
        print_err_with_info(
            std_text,
            &format!(
                "Expected `{}` directory to exist inside your current project",
                path_to_colored(partial_dir.to_str().unwrap_or_default())
            ),
        );
        source.clear();
        return false;
    }

    true
}

fn process_import_entry(
    partial: &StructurePartial,
    source_lang: &Lang,
    current_lang: &Lang,
    source_partial_dir: &PathBuf,
    current_partial_dir: &PathBuf,
    imported_name: &str,
    commands_to_exec: &mut Vec<Vec<String>>,
) {
    let source_src_dir = parent_dir(source_partial_dir);
    let current_src_dir = parent_dir(current_partial_dir);
    let partial_dir = partial.get_dir();

    let (partial_path, source_partial_path) = resolve_import_path(
        partial,
        imported_name,
        &partial_dir,
        &source_src_dir,
        source_lang,
    );

    if !source_partial_path.exists() {
        return;
    }

    let partial_path_str = partial_path.to_str().unwrap_or_default();

    let mut current_partial_path = current_src_dir.join(&partial_path);
    if current_partial_path.exists() {
        let new_current_partial_path = handle_existing_import(
            &current_partial_path,
            &current_partial_dir,
            partial_path_str,
            current_lang,
        );

        if current_partial_path == new_current_partial_path {
            return;
        }

        current_partial_path = new_current_partial_path
    }

    let processed_entries = collect_import_entries(
        &source_partial_path,
        &current_partial_path,
        partial,
        source_partial_dir,
        imported_name,
        commands_to_exec,
        current_lang,
    );

    process_import_files(&processed_entries, source_lang, current_lang);
}

fn resolve_import_path(
    partial: &StructurePartial,
    imported_name: &str,
    partial_dir: &PathBuf,
    source_src_dir: &PathBuf,
    source_lang: &Lang,
) -> (PathBuf, PathBuf) {
    let mut partial_path = partial_dir.join(imported_name);
    let mut source_partial_path = source_src_dir.join(&partial_path);

    if !source_partial_path.exists() {
        match partial.part {
            StructurePart::Common | StructurePart::Domain => {
                let extension = source_lang.get_main_file_extension();
                if !imported_name.ends_with(&extension) {
                    partial_path = partial_dir.join(format!("{}{}", imported_name, extension));
                    source_partial_path = source_src_dir.join(&partial_path);

                    if !source_partial_path.exists() {
                        print_warn_with_info(
                            &format!(
                                "The imported file/directory `{}` does not exist in the source project",
                                path_to_colored(partial_path.to_str().unwrap_or_default())
                            ),
                            "Skipping",
                        );
                    }
                }
            }
            _ => {
                print_warn_with_info(
                    &format!(
                        "The imported file/directory `{}` does not exist in the source project",
                        path_to_colored(partial_path.to_str().unwrap_or_default())
                    ),
                    "Skipping",
                );
            }
        }
    }

    (partial_path, source_partial_path)
}

fn handle_existing_import(
    current_partial_path: &PathBuf,
    current_partial_dir: &PathBuf,
    partial_path_str: &str,
    current_lang: &Lang,
) -> PathBuf {
    print_warn(&format!(
        "Cannot import `{}` because it already exists in the current project",
        path_to_colored(partial_path_str)
    ));
    print_text("");

    let renaming_confirmation = read_input(&format!(
        "Do you want to copy and rename the imported file/directory? (y/{}) ",
        "N".bright_white().bold()
    ));

    print_text("");

    if renaming_confirmation.trim().to_lowercase() != "y" {
        print_warn(&format!("Skipping: {}", path_to_colored(partial_path_str)));
        return current_partial_path.clone();
    }

    prompt_for_new_name(current_partial_path, current_partial_dir, current_lang)
}

fn prompt_for_new_name(
    current_partial_path: &PathBuf,
    current_partial_dir: &PathBuf,
    current_lang: &Lang,
) -> PathBuf {
    print_text("");

    let mut prompt_text = "Copy and rename to: ";
    let extension = current_lang.get_main_file_extension();

    loop {
        let mut new_name = read_input(prompt_text);
        if new_name.is_empty() {
            prompt_text = "Name can't be empty, please enter a valid one: ";
            continue;
        }

        if current_partial_path.is_file() && !new_name.ends_with(&extension) {
            new_name = format!("{}{}", new_name, extension);
        }

        let new_path = current_partial_dir.join(&new_name);
        if new_path.exists() {
            prompt_text =
                "A file/directory with that name already exists. Please enter a different name: ";
            continue;
        }

        print_text("");

        return parent_dir(current_partial_path).join(new_name);
    }
}

fn collect_import_entries(
    source_partial_path: &PathBuf,
    current_partial_path: &PathBuf,
    partial: &StructurePartial,
    source_partial_dir: &PathBuf,
    imported_name: &str,
    commands_to_exec: &mut Vec<Vec<String>>,
    current_lang: &Lang,
) -> Vec<[PathBuf; 2]> {
    let mut processed_entries: Vec<[PathBuf; 2]> = Vec::new();

    if source_partial_path.is_file() {
        processed_entries.push([source_partial_path.clone(), current_partial_path.clone()]);
    } else if source_partial_path.is_dir() {
        if let Ok(entries) = source_partial_path.read_dir() {
            for entry in entries.flatten() {
                processed_entries.push([
                    entry.path(),
                    current_partial_path.join(filename(entry.path().as_path())),
                ]);
            }

            collect_dependencies(
                partial,
                parent_dir(source_partial_dir),
                imported_name,
                &mut processed_entries,
                commands_to_exec,
                current_lang,
            );
        }
    }

    processed_entries
}

fn collect_dependencies(
    partial: &StructurePartial,
    source_src_dir: PathBuf,
    imported_name: &str,
    processed_entries: &mut Vec<[PathBuf; 2]>,
    commands_to_exec: &mut Vec<Vec<String>>,
    current_lang: &Lang,
) {
    match partial.collect_feature_dependencies(source_src_dir.as_path(), imported_name) {
        Ok((dependency_paths, externals)) => {
            for dependency_path in dependency_paths {
                processed_entries.push([
                    dependency_path.clone(),
                    cwd().join(
                        dependency_path
                            .strip_prefix(source_src_dir.clone())
                            .unwrap()
                            .to_path_buf(),
                    ),
                ]);
            }

            for external_dependency in externals {
                if !external_dependency.is_empty() {
                    for command in
                        current_lang.generate_package_install_commands(&vec![&external_dependency])
                    {
                        commands_to_exec.push(command);
                    }
                }
            }
        }
        Err(err) => print_warn_with_info(
            "Failed to retrieve the dependency file",
            &format!("{} => This may cause missing dependencies", err),
        ),
    }
}

fn process_import_files(
    processed_entries: &[[PathBuf; 2]],
    source_lang: &Lang,
    current_lang: &Lang,
) {
    for [src, dst] in processed_entries {
        if src.is_file() {
            process_import_file(src, dst, source_lang, current_lang);
            register_rust_module(dst, current_lang);
        } else if src.is_dir() {
            create_directory_with_feedback(dst);
            register_rust_module(dst, current_lang);
        }
    }
}

fn process_import_file(src: &PathBuf, dst: &PathBuf, source_lang: &Lang, current_lang: &Lang) {
    print_info(&format!("Create file `{}`", path_to_colored(dst)));

    if dst.exists() {
        print_warn_with_info(
            &format!("File `{}` already exists", path_to_colored(dst)),
            "Skipping.",
        );
        return;
    }

    let mut content = match read_file(src) {
        Ok(c) => c,
        Err(_) => {
            print_warn_with_info(
                &format!(
                    "Failed to read content from `{}` in the source",
                    path_to_colored(dst)
                ),
                "Skipping",
            );
            return;
        }
    };

    content = replace_module_names(content, source_lang, current_lang);

    match create_file(dst, &content) {
        Ok(_) => print_done(&format!("Create file `{}`", path_to_colored(dst))),
        Err(_) => {
            print_warn_with_info(
                &format!("Failed to create the file `{}`", path_to_colored(dst)),
                "Skipping",
            );
        }
    }
}

fn replace_module_names(mut content: String, source_lang: &Lang, current_lang: &Lang) -> String {
    if !source_lang.prop.module_name.is_empty() && !current_lang.prop.module_name.is_empty() {
        if let LangKind::Go = current_lang.kind {
            let patterns = [
                format!("import \"{}/", source_lang.prop.module_name),
                format!("\t\"{}/", source_lang.prop.module_name),
                format!("    \"{}/", source_lang.prop.module_name),
                format!("   \"{}/", source_lang.prop.module_name),
            ];

            let replacements = [
                format!("import \"{}/", current_lang.prop.module_name),
                format!("\t\"{}/", current_lang.prop.module_name),
                format!("    \"{}/", current_lang.prop.module_name),
                format!("   \"{}/", current_lang.prop.module_name),
            ];

            for (pattern, replacement) in patterns.iter().zip(replacements.iter()) {
                content = content.replace(pattern, replacement);
            }
        }
    }

    content
}

fn create_directory_with_feedback(path: &PathBuf) {
    match create_dir(path) {
        Ok(_) => print_done(&format!("Create dir `{}`", path_to_colored(path))),
        Err(_) => {
            print_warn_with_info(
                &format!("Failed to create the directory `{}`", path_to_colored(path)),
                "Skipping",
            );
        }
    }
}

fn register_rust_module(dst: &PathBuf, current_lang: &Lang) {
    if !dst.exists() {
        return;
    }

    if let LangKind::Rust = current_lang.kind {
        let (dst_parent_dir, dst_mod_name) = if dst.is_file() {
            (parent_dir(dst), file_stem(dst))
        } else {
            (dst.to_path_buf(), dirname(dst))
        };

        update_rust_mod_files(&dst_parent_dir, &dst_mod_name, current_lang);
    }
}

fn update_rust_mod_files(parent_dir: &PathBuf, mod_name: &str, lang: &Lang) {
    if mod_name == "mod" {
        return;
    }

    for mod_file_name in &lang.mod_file_names {
        let mod_file_path = parent_dir.join(mod_file_name);
        if !mod_file_path.is_file() {
            continue;
        }

        if let Ok(mut mod_content) = read_file(&mod_file_path) {
            let mod_statement = format!("{} {};", lang.mod_prefix, mod_name);
            if mod_content.contains(&mod_statement) {
                continue;
            }

            mod_content = format!("{}\n{}\n", trim_newline(&mod_content), mod_statement);

            if write_file(&mod_file_path, &mod_content).is_ok() {
                print_done(&format!(
                    "Mod/lib file `{}` inside `{}` has been updated",
                    path_to_colored(mod_file_name),
                    path_to_colored(parent_dir)
                ));
            }
        }
    }
}

fn execute_commands(working_dir: &PathBuf, commands: Vec<Vec<String>>) {
    for command_vec in commands {
        let cmd_str = vec_string_command_to_colored_string(&command_vec);

        print_info(&format!("Exec `{}`", cmd_str));

        match exec_vec_string_command(working_dir, &command_vec) {
            Ok(_) => print_done(&format!("Exec `{}`", cmd_str)),
            Err(err) => print_warn_with_info(
                &format!("Exec `{}`", cmd_str),
                &format!("{} => This may cause missing dependencies", err),
            ),
        }
    }
}

fn handle_generator(mut structure: Structure) {
    if let Err(err) = structure.validate() {
        print_err(&err);
        return;
    }

    print_text("");

    structure.lang.compose_prop_from_input();

    display_project_summary(&structure);

    if !confirm_generation() {
        print_done("Aborted");
        return;
    }

    let std_text = "Generating";

    print_info(std_text);

    let (dir_components, dir_entries) = match get_structure_components(&structure) {
        Ok(result) => result,
        Err(err) => {
            print_err(&err);
            return;
        }
    };

    let project_path = PathBuf::from(&structure.lang.prop.project_name);

    if project_path.is_dir() {
        print_err_with_info(
            std_text,
            &format!(
                "The directory `{}` already exists",
                path_to_colored(&project_path)
            ),
        );
        return;
    }

    if let Err(_) = create_project_directory(&project_path) {
        return;
    }

    if !generate_project_files(&structure, &dir_components, &dir_entries, &project_path) {
        return;
    }

    execute_post_generation(&structure, &project_path);
    execute_setup_commands(&mut structure, &project_path);
    display_success_message(&structure);
    print_done(std_text);
}

fn display_project_summary(structure: &Structure) {
    print_text("");
    print_text("");
    print_info("Project Summary");
    print_text("");
    print_text(&format!(
        "Language: {}",
        structure.lang.name.bright_magenta().bold()
    ));
    print_text(&format!(
        "Structure version: {}",
        structure.version.name().bright_yellow().bold()
    ));
    print_text(&format!(
        "Project name: {}",
        structure.lang.prop.project_name
    ));

    match structure.lang.kind {
        LangKind::Go => {
            print_text(&format!("Go module: {}", structure.lang.prop.module_name));
            print_text(if structure.lang.prop.is_use_vendoring {
                "Vendoring: yes"
            } else {
                "Vendoring: no"
            });
        }
        LangKind::Python => {
            print_text(if structure.lang.prop.is_use_vendoring {
                "Virtual environment (venv): yes"
            } else {
                "Virtual environment (venv): no"
            });
            print_text(&format!(
                "Python command: {}",
                structure.lang.command.base.bright_cyan()
            ));
        }
        _ => {}
    }

    print_text("");
    print_text("");
}

fn confirm_generation() -> bool {
    let confirmation = read_input(&format!(
        "Do you want to continue? (y/{}) ",
        "N".bright_white().bold()
    ));

    print_text("");

    confirmation.to_lowercase() == "y"
}

fn get_structure_components(
    structure: &Structure,
) -> Result<(Vec<Component<'_>>, Vec<DirEntry<'_>>), String> {
    let dir_components = structure
        .included_dir
        .components()
        .map_err(|e| e.to_string())?;
    let dir_entries = structure.get_entries().map_err(|e| e.to_string())?;

    Ok((dir_components, dir_entries))
}

fn create_project_directory(project_path: &PathBuf) -> Result<(), ()> {
    match create_dir(project_path) {
        Ok(_) => {
            print_done(&format!("Create dir `{}`", path_to_colored(project_path)));
            Ok(())
        }
        Err(err) => {
            print_err_with_info(
                &format!("Create dir `{}`", path_to_colored(project_path)),
                &err,
            );
            let _ = remove_dir(project_path);
            Err(())
        }
    }
}

fn generate_project_files(
    structure: &Structure,
    dir_components: &[Component],
    dir_entries: &[DirEntry],
    project_path: &PathBuf,
) -> bool {
    for structure_entry in dir_entries {
        let structure_entry_components: Vec<Component> =
            structure_entry.path().components().collect();
        let project_entry_components: Vec<Component> =
            if structure_entry_components.starts_with(dir_components) {
                structure_entry_components[dir_components.len()..].to_vec()
            } else {
                Vec::new()
            };

        if project_entry_components.is_empty() {
            print_err("Failed to find the project entry parts");
            let _ = remove_dir(project_path);
            return false;
        }

        let project_entry_path = build_project_entry_path(project_path, &project_entry_components);

        if structure_entry.as_dir().is_some() {
            if let Err(err) = create_dir(&project_entry_path) {
                print_err_with_info(
                    &format!("Create dir `{}`", path_to_colored(&project_entry_path)),
                    &err,
                );
                let _ = remove_dir(project_path);
                return false;
            }

            print_done(&format!(
                "Create dir `{}`",
                path_to_colored(&project_entry_path)
            ));

            continue;
        }

        if !create_project_file(structure_entry, &project_entry_path, structure) {
            let _ = remove_dir(project_path);
            return false;
        }
    }
    true
}

fn build_project_entry_path(project_path: &PathBuf, components: &[Component]) -> PathBuf {
    let mut path = project_path.clone();

    for component in components {
        let component_str = component.as_os_str().to_str().unwrap_or_default();
        if component_str.ends_with(EXTENSION_TO_REMOVE) {
            path.push(trim_extension(component_str, EXTENSION_TO_REMOVE));
        } else {
            path.push(component_str);
        }
    }

    path
}

fn create_project_file(
    structure_entry: &DirEntry,
    project_entry_path: &PathBuf,
    structure: &Structure,
) -> bool {
    let content = match structure_entry.as_file() {
        Some(f) => match f.contents_utf8() {
            Some(c) => c.to_owned(),
            None => {
                print_err(&format!(
                    "Failed to read file content: {}",
                    path_to_colored(&structure_entry.path())
                ));
                return false;
            }
        },
        None => {
            print_err(&format!(
                "Failed to get file: {}",
                path_to_colored(&structure_entry.path())
            ));
            return false;
        }
    };

    let processed_content = replace_template_placeholders(content, structure);

    match create_file(project_entry_path, &processed_content) {
        Ok(_) => {
            print_done(&format!(
                "Create file `{}`",
                path_to_colored(project_entry_path)
            ));
            true
        }
        Err(err) => {
            print_err_with_info(
                &format!("Create file `{}`", path_to_colored(project_entry_path)),
                &err,
            );
            false
        }
    }
}

fn replace_template_placeholders(mut content: String, structure: &Structure) -> String {
    content = content.replace(STRUCTURE_VERSION_TO_REPLACE, &structure.version.name());
    content = content.replace(PROJECT_NAME_TO_REPLACE, &structure.lang.prop.project_name);
    content = content.replace(MODULE_NAME_TO_REPLACE, &structure.lang.prop.module_name);
    content = content.replace(LANGUAGE_NAME_TO_REPLACE, &structure.lang.structure_dir_name);
    content = content.replace(
        LANGUAGE_EXTENSION_TO_REPLACE,
        &structure.lang.get_main_file_extension(),
    );

    let vendoring_scripts: Vec<String> = structure
        .lang
        .command
        .vendoring
        .iter()
        .map(|script| script.join(" "))
        .collect();
    content = content.replace(VENDORING_SCRIPT_TO_REPLACE, &vendoring_scripts.join(" && "));

    content
}

fn execute_post_generation(structure: &Structure, project_path: &PathBuf) {
    for command_vec in &structure.command_post_generation {
        let cmd_str = vec_string_command_to_colored_string(command_vec);
        print_info(&format!("Exec `{}`", cmd_str));

        match exec_vec_string_command(project_path, command_vec) {
            Ok(_) => print_done(&format!("Exec `{}`", cmd_str)),
            Err(err) => print_warn_with_info(
                &format!("Exec `{}`", cmd_str),
                &format!(
                    "{} => This may cause the project not configures well and not working",
                    err
                ),
            ),
        }
    }
}

fn execute_setup_commands(structure: &mut Structure, project_path: &PathBuf) {
    let mut commands_to_execute: Vec<Vec<String>> = Vec::new();

    match structure.lang.kind {
        LangKind::Go => {
            commands_to_execute.extend(
                structure
                    .lang
                    .generate_module_init_commands(&structure.lang.prop.module_name),
            );
            commands_to_execute.extend(structure.lang.command.dependency_install.clone());
            if structure.lang.prop.is_use_vendoring {
                commands_to_execute.extend(structure.lang.command.vendoring.clone());
            }
        }
        LangKind::Python => {
            if structure.lang.prop.is_use_vendoring {
                execute_and_log_commands(&structure.lang.command.vendoring, project_path);
                structure.lang.compose_command();
            }
            commands_to_execute.extend(structure.lang.command.dependency_install.clone());
        }
        LangKind::TypeScript => {
            execute_and_log_commands(&structure.lang.command.dependency_install, project_path);
            structure.lang.compose_command();
            commands_to_execute.extend(structure.lang.command.build.clone());
        }
        LangKind::NodeJS => {
            commands_to_execute.extend(structure.lang.command.dependency_install.clone());
            commands_to_execute.extend(structure.lang.command.build.clone());
        }
        LangKind::Rust => {
            commands_to_execute.extend(
                structure
                    .lang
                    .generate_module_init_commands(&structure.lang.prop.module_name),
            );
            let packages = vec![
                "chrono",
                "once_cell",
                "actix_web",
                "serde --features derive",
                "serde_json",
                "async_trait",
            ];
            commands_to_execute.extend(structure.lang.generate_package_install_commands(&packages));
        }
        _ => {}
    }

    execute_and_log_commands(&commands_to_execute, project_path);
}

fn execute_and_log_commands(commands: &Vec<Vec<String>>, project_path: &PathBuf) {
    for command_vec in commands {
        let cmd_str = vec_string_command_to_colored_string(&command_vec);

        print_info(&format!("Exec `{}`", cmd_str));

        match exec_vec_string_command(project_path, &command_vec) {
            Ok(_) => print_done(&format!("Exec `{}`", cmd_str)),
            Err(err) => print_warn_with_info(
                &format!("Exec `{}`", cmd_str),
                &format!(
                    "{} => This may cause the project not configures well and not working",
                    err
                ),
            ),
        }
    }
}

fn display_success_message(structure: &Structure) {
    let running_scripts: Vec<String> = structure
        .lang
        .command
        .running
        .iter()
        .map(|script| script.join(" "))
        .collect();

    print_text("");
    print_info("Project Generated");
    print_text("");
    print_text("To get started, you can enter to your project directory:");
    print_text("");
    print_text(&format!(
        "{:>4}{} {}",
        " ",
        "cd".bright_cyan(),
        structure.lang.prop.project_name.bright_cyan()
    ));
    print_text("");
    print_text("then, run your application (for example):");
    print_text("");
    print_text(&format!(
        "{:>4}{}",
        " ",
        running_scripts.join(" && ").bright_cyan()
    ));
    print_text("");
    print_text("Once the application is running, it will be available at:");
    print_text("");
    print_text(&format!(
        "{:>4}{}",
        " ",
        "http://localhost:8080".bright_blue()
    ));
    print_text("");
    print_text("For example, you can try:");
    print_text("");
    print_text(&format!(
        "{:>4}{} {}",
        " ",
        "curl".bright_cyan(),
        "http://localhost:8080/todos".bright_blue()
    ));
    print_text("");
}

fn handle_add(_os: OS, mut template: Template, names: &[String]) {
    if names.is_empty() {
        print_done("Nothing to be added");
        return;
    }

    if let Err(err) = template.validate() {
        print_err(&err);
        return;
    }

    let current_dir = cwd();

    template.lang.compose_prop_from_dir(&current_dir);

    let std_text = "Adding template(s)";

    print_info(std_text);

    let (dir_components, dir_entries) = match get_template_components(&template) {
        Ok(result) => result,
        Err(err) => {
            print_err(&err);
            return;
        }
    };

    let mut created_target_paths: Vec<PathBuf> = Vec::new();

    for name in names {
        if name.is_empty() {
            print_warn_with_info("Empty template name", "Skipping.");
            continue;
        }

        if !process_template(
            name,
            &template,
            &dir_components,
            &dir_entries,
            &current_dir,
            &mut created_target_paths,
        ) {
            remove_paths(created_target_paths);
            return;
        }
    }

    print_done(std_text);
}

fn get_template_components(
    template: &Template,
) -> Result<(Vec<Component<'_>>, Vec<DirEntry<'_>>), String> {
    let dir_components = template
        .included_dir
        .components()
        .map_err(|e| e.to_string())?;
    let dir_entries = template.get_entries().map_err(|e| e.to_string())?;
    Ok((dir_components, dir_entries))
}

fn process_template(
    name: &str,
    template: &Template,
    dir_components: &[Component],
    dir_entries: &[DirEntry],
    current_dir: &PathBuf,
    created_target_paths: &mut Vec<PathBuf>,
) -> bool {
    let name_pascal_case = to_pascal_case(name);

    for dir_entry in dir_entries {
        let entry_components: Vec<Component> = dir_entry.path().components().collect();
        let project_entry_components: Vec<Component> =
            if entry_components.starts_with(dir_components) {
                entry_components[dir_components.len()..].to_vec()
            } else {
                Vec::new()
            };

        if project_entry_components.is_empty() {
            print_err("Failed to find project entry parts");
            return false;
        }

        let project_entry_component_vec: Vec<String> = project_entry_components
            .iter()
            .filter_map(|c| {
                let s = c.as_os_str().to_str().unwrap_or_default().to_string();
                if !s.ends_with(EXTENSION_TO_REMOVE) {
                    Some(s.replace(TEMPLATE_PREFIX_FILENAME, name))
                } else {
                    None
                }
            })
            .collect();

        let project_entry_component_str = project_entry_component_vec.join(MAIN_SEPARATOR_STR);

        let (target_filename, target_content) =
            match extract_template_content(template, dir_entry, name) {
                Some(result) => result,
                None => continue,
            };

        let target_content =
            replace_template_content(target_content, template, name, &name_pascal_case);
        let target_path = current_dir
            .join(&project_entry_component_str)
            .join(&target_filename);

        if target_path.exists() {
            print_warn_with_info(
                &format!("File `{}` already exists", path_to_colored(&target_path)),
                "Skipping.",
            );
            continue;
        }

        if !create_template_file(
            &target_path,
            &target_content,
            name,
            &project_entry_component_str,
            created_target_paths,
        ) {
            return false;
        }

        if template.lang.kind == LangKind::Rust {
            update_rust_mod_files(
                &parent_dir(&target_path),
                &file_stem(&target_path),
                &template.lang,
            );
        }
    }

    true
}

fn extract_template_content(
    template: &Template,
    dir_entry: &DirEntry,
    name: &str,
) -> Option<(String, String)> {
    let file = dir_entry.as_file()?;
    let content = file.contents_utf8()?.to_string();
    let entry_filepath = dir_entry.path();
    let entry_filepath_str = entry_filepath.to_string_lossy();

    let filename = match template.part {
        StructurePart::Common | StructurePart::Domain => {
            if entry_filepath_str.contains(&template.part.dir_name()) {
                let relative_path = entry_filepath
                    .strip_prefix(template.part.dir_name())
                    .unwrap_or(entry_filepath);
                let relative_path_str = relative_path.to_string_lossy();
                if relative_path_str.contains(TEMPLATE_PREFIX_FILENAME) {
                    file_stem(relative_path).replace(TEMPLATE_PREFIX_FILENAME, name)
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        StructurePart::Feature => {
            if entry_filepath_str.contains(&template.part.dir_name())
                || entry_filepath_str.contains(StructurePart::Domain.dir_name())
            {
                let relative_path = entry_filepath
                    .strip_prefix(template.part.dir_name())
                    .unwrap_or(entry_filepath);
                file_stem(relative_path).replace(TEMPLATE_PREFIX_FILENAME, name)
            } else {
                return None;
            }
        }
        _ => return None,
    };

    Some((filename, content))
}

fn replace_template_content(
    mut content: String,
    template: &Template,
    name: &str,
    name_pascal_case: &str,
) -> String {
    content = content.replace(STRUCTURE_VERSION_TO_REPLACE, &template.version.name());
    content = content.replace(PROJECT_NAME_TO_REPLACE, &template.lang.prop.project_name);
    content = content.replace(MODULE_NAME_TO_REPLACE, &template.lang.prop.module_name);
    content = content.replace(LANGUAGE_NAME_TO_REPLACE, &template.lang.structure_dir_name);
    content = content.replace(
        LANGUAGE_EXTENSION_TO_REPLACE,
        &template.lang.get_main_file_extension(),
    );
    content = content.replace(TEMPLATE_NAME_TO_REPLACE, name);
    content = content.replace(TEMPLATE_NAME_PASCAL_CASE_TO_REPLACE, name_pascal_case);
    content
}

fn create_template_file(
    target_path: &PathBuf,
    target_content: &str,
    name: &str,
    project_entry_component_str: &str,
    created_target_paths: &mut Vec<PathBuf>,
) -> bool {
    let parent = parent_dir(target_path);
    if !parent.exists() {
        let _ = create_dir(&parent);
    }

    match create_file(target_path, target_content) {
        Ok(_) => {
            print_done(&format!("Create file `{}`", path_to_colored(target_path)));
            created_target_paths.push(target_path.clone());
            if project_entry_component_str.contains(name) {
                created_target_paths.push(parent);
            }
            true
        }
        Err(err) => {
            print_warn_with_info(
                &format!("Failed to create file `{}`", path_to_colored(target_path)),
                &err,
            );
            false
        }
    }
}
