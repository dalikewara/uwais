use crate::lang::Lang;
use crate::os::OS;
use crate::source::Source;
use crate::string::split_text;
use crate::structure::{
    Part as StructurePart, Partial as StructurePartial, Structure, Template as StructureTemplate,
    Version as StructureVersion,
};
use crate::sys::cwd;

#[derive(Debug)]
pub enum Command {
    Help,
    Version,
    Update(Source),
    UpdaterTask,
    UpdaterTaskClearance,
    Import(StructurePartial, Source, Vec<String>),
    Generator(Structure),
    Add(StructureTemplate, Vec<String>),
    Unknown,
}

impl Command {
    pub fn new(os: OS, args: &[String]) -> Self {
        if args.is_empty() {
            return Self::Help;
        }

        match args.len() {
            1 => Self::parse_single_arg(&os, &args[0]),
            2 => Self::parse_two_args(&os, &args[0], &args[1]),
            3 => Self::parse_three_args(&os, args),
            4 => Self::parse_four_args(&os, args),
            _ => Self::Unknown,
        }
    }

    fn parse_single_arg(os: &OS, arg: &str) -> Self {
        let arg = arg.trim();
        match arg {
            "" | "help" => Self::Help,
            "version" => Self::Version,
            "update" => Self::Update(Source::new_latest_app_release(os.clone())),
            "--updater-task" => Self::UpdaterTask,
            "--updater-task-clearance" => Self::UpdaterTaskClearance,
            _ => Self::try_parse_language_command(os, arg, None),
        }
    }

    fn parse_two_args(os: &OS, arg1: &str, arg2: &str) -> Self {
        let arg1 = arg1.trim();
        let arg2 = arg2.trim();

        if arg1.is_empty() || arg2.is_empty() {
            return Self::Unknown;
        }

        Self::try_parse_language_command(os, arg1, Some(arg2))
    }

    fn parse_three_args(os: &OS, args: &[String]) -> Self {
        let command = args[0].trim();
        let subcommand = args[1].trim();
        let names = args[2].trim();

        if command.is_empty() || subcommand.is_empty() || names.is_empty() {
            return Self::Unknown;
        }

        match command {
            "add" => Self::parse_add_command(os, subcommand, names),
            _ => Self::Unknown,
        }
    }

    fn parse_four_args(os: &OS, args: &[String]) -> Self {
        let command = args[0].trim();
        let subcommand = args[1].trim();
        let names = args[2].trim();
        let source = args[3].trim();

        if command.is_empty() || subcommand.is_empty() || names.is_empty() || source.is_empty() {
            return Self::Unknown;
        }

        match command {
            "import" => Self::parse_import_command(os, subcommand, names, source),
            _ => Self::Unknown,
        }
    }

    fn try_parse_language_command(os: &OS, lang_name: &str, version: Option<&str>) -> Self {
        if lang_name.trim().is_empty() {
            return Self::Unknown;
        }

        let lang = Lang::new(os.clone(), lang_name);
        if !lang.is_valid() {
            return Self::Unknown;
        }

        let structure_version = version
            .filter(|v| !v.trim().is_empty())
            .map(StructureVersion::from)
            .unwrap_or_default();

        Self::Generator(Structure::new(structure_version, lang))
    }

    fn parse_add_command(os: &OS, part: &str, names: &str) -> Self {
        if part.is_empty() || names.is_empty() {
            return Self::Unknown;
        }

        let current_dir = cwd();
        let lang = Lang::new_from_dir(os.clone(), &current_dir);
        let template =
            StructureTemplate::new(StructureVersion::default(), StructurePart::from(part), lang);

        let name_list: Vec<String> = parse_comma_separated(names)
            .into_iter()
            .map(|n| n.trim().to_string())
            .filter(|n| !n.is_empty())
            .collect();
        if name_list.is_empty() {
            return Self::Unknown;
        }

        Self::Add(template, name_list)
    }

    fn parse_import_command(os: &OS, part: &str, names: &str, source_url: &str) -> Self {
        if part.is_empty() || names.is_empty() || source_url.is_empty() {
            return Self::Unknown;
        }

        let partial = StructurePartial::new(StructureVersion::default(), StructurePart::from(part));
        let source = Source::new(os.clone(), source_url);

        let name_list: Vec<String> = parse_comma_separated(names)
            .into_iter()
            .map(|n| n.trim().to_string())
            .filter(|n| !n.is_empty())
            .collect();
        if name_list.is_empty() {
            return Self::Unknown;
        }

        Self::Import(partial, source, name_list)
    }
}

#[inline]
fn parse_comma_separated(input: &str) -> Vec<String> {
    split_text(input, ",")
}
