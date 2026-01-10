use colored::Colorize;
use std::path::MAIN_SEPARATOR_STR;

use crate::sys::VERSION;

const ARROW: &str = "=>";
const DONE_TAG: &str = "[DONE]";
const WARN_TAG: &str = "[WARN]";
const ERR_TAG: &str = "[ERR]";

const BASIC_COMMANDS: &[(&str, &str)] = &[
    ("version", "Get the current uwais version"),
    ("update", "Update to the latest uwais version"),
];

const GENERATOR_COMMANDS: &[(&str, &str, &str)] = &[
    ("go", "[VERSION]", "Generate new Golang project"),
    ("py", "[VERSION]", "Generate new Python project"),
    ("ts", "[VERSION]", "Generate new TypeScript project"),
    ("node", "[VERSION]", "Generate new NodeJS project"),
    ("rs", "[VERSION]", "Generate new Rust project"),
    // ("sveltekit", "[VERSION]", "Generate new SvelteKit project"),
];

#[inline]
pub fn print_help() {
    println!();
    print_welcome_banner();
    println!("\n\n{}", "Command:");
    print_commands();
    print_import_section();
    print_add_section();
    print_notes();
    print_usage_examples();
    println!();
}

#[inline]
fn print_welcome_banner() {
    println!(
        "{} {}{}",
        "Welcome to",
        format!("Uwais (v{})", VERSION).bright_white().bold(),
        "—formerly named AyaPingPing, a project structure generator for building applications that follow Clean Architecture and \
        Feature-Driven Design concept in various programming languages (such as Golang, Python, Typescript, etc.). It aims to be a seamless \
        and very simple project structure while avoiding unnecessary complexity.",
    );
}

#[inline]
fn print_commands() {
    for (cmd, desc) in BASIC_COMMANDS {
        println!("{:<30}{}", cmd.bright_cyan(), desc);
    }

    for (cmd, ver, desc) in GENERATOR_COMMANDS {
        println!(
            "{:<48}{}",
            format!("{} {}", cmd.bright_cyan(), ver.bright_black()),
            desc
        );
    }
}

#[inline]
fn print_import_section() {
    println!(
        "{:<48}{}",
        format!("{} {}", "import".bright_cyan(), "OPTION".bright_purple()),
        "Command to import function(s) from a source project"
    );

    print_option_commands("import");
}

#[inline]
fn print_add_section() {
    println!(
        "{:<48}{}",
        format!("{} {}", "add".bright_cyan(), "OPTION".bright_purple()),
        "Command to add template(s) to the current project"
    );

    print_option_commands("add");
}

#[inline]
fn print_option_commands(context: &str) {
    let options = [
        (
            "common",
            "NAMES...",
            "SOURCE",
            if context == "import" {
                "Import common function(s)"
            } else {
                "Add new common function(s) template"
            },
        ),
        (
            "domain",
            "NAMES...",
            "SOURCE",
            if context == "import" {
                "Import domain(s)"
            } else {
                "Add new domain(s) template"
            },
        ),
        (
            "feature",
            "NAMES...",
            "SOURCE",
            if context == "import" {
                "Import feature(s)"
            } else {
                "Add new feature(s) template"
            },
        ),
    ];

    for (opt, args1, args2, desc) in options {
        let indent = if opt == "feature" { 11 } else { 10 };
        println!(
            "{:<49}{}",
            format!(
                "{:>indent$} {} {}",
                opt.bright_purple(),
                args1,
                args2.bright_blue(),
                indent = indent
            ),
            desc
        );
    }
}

#[inline]
fn print_notes() {
    println!("\n\n{}", "Note:".bright_yellow());

    let notes = [
        format!(
            "{} {} {}",
            "-".bright_yellow(),
            "[ ]".bright_black(),
            "is OPTIONAL"
        ),
        format!(
            "{} {}",
            "-".bright_yellow(),
            "... represents multiple values separated by commas (no spaces allowed)"
        ),
        format!(
            "{} {}",
            "-".bright_yellow(),
            "VERSION is the structure version. Available versions (default is v4):"
        ),
        format!("{:>3} {}", "-".bright_yellow(), "v4"),
        format!(
            "{} A {} {}",
            "-".bright_yellow(),
            "SOURCE".bright_blue(),
            "project can be one of these:"
        ),
        format!(
            "{:>3} {}",
            "-".bright_yellow(),
            format!(
                "{}the{}project{}full{}path",
                MAIN_SEPARATOR_STR, MAIN_SEPARATOR_STR, MAIN_SEPARATOR_STR, MAIN_SEPARATOR_STR
            )
            .bright_blue()
        ),
        format!(
            "{:>3} {}",
            "-".bright_yellow(),
            "git@github.com:username/your/project.git".bright_blue()
        ),
        format!(
            "{:>3} {}",
            "-".bright_yellow(),
            "https://github.com/username/your/project.git".bright_blue()
        ),
    ];

    for note in notes {
        println!("{}", note);
    }
}

#[inline]
fn print_usage_examples() {
    println!("\n\n{}", "Usage:".bright_green());

    println!("Generate a new Golang project:\n");
    println!("{:>9} {}", "uwais".bright_cyan(), "go".bright_cyan());

    println!("\nImport features from another project:\n");
    println!(
        "{:>9} {} {} {} {}",
        "uwais".bright_cyan(),
        "import".bright_cyan(),
        "feature".bright_purple(),
        "user,product",
        format!(
            "{}my{}project{}path",
            MAIN_SEPARATOR_STR, MAIN_SEPARATOR_STR, MAIN_SEPARATOR_STR
        )
        .bright_blue()
    );

    println!("\nImport common functions from another project:\n");
    println!(
        "{:>9} {} {} {} {}",
        "uwais".bright_cyan(),
        "import".bright_cyan(),
        "common".bright_purple(),
        "validator,formatter",
        "git@github.com:username/my/project.git".bright_blue()
    );
}

#[inline]
pub fn print_input(text: &str) {
    print!("{}", text)
}

#[inline]
pub fn print_text(text: &str) {
    println!("{}", text)
}

#[inline]
pub fn print_info(text: &str) {
    println!("{} {}...", ARROW.bright_cyan(), text)
}

#[inline]
pub fn print_done(text: &str) {
    println!(
        "{} {}. {}",
        ARROW.bright_cyan(),
        text,
        DONE_TAG.bright_green().bold()
    )
}

#[inline]
pub fn print_warn(text: &str) {
    print_warn_with_info(text, "")
}

#[inline]
pub fn print_warn_with_info(text: &str, info: &str) {
    println!(
        "{} {}... {} !! {}.",
        ARROW.bright_yellow(),
        text.bright_yellow(),
        WARN_TAG.bright_yellow().bold(),
        info.yellow().bold()
    )
}

#[inline]
pub fn print_err(text: &str) {
    print_err_with_info(text, "")
}

#[inline]
pub fn print_err_with_info(text: &str, info: &str) {
    println!(
        "{} {}... {} !! {}.",
        ARROW.bright_red(),
        text.bright_red(),
        ERR_TAG.bright_red().bold(),
        info.red().bold()
    )
}
