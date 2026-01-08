use std::{ffi::OsString, path::Path};

use colored::Colorize;
use serde::Serialize;

use crate::file_tree::FileTree;

/// Node package manager helper.
pub struct Npm;

/// Represents sub-commands that can be executed by [`Npm`].
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubCommand {
    /// Represents the install command.
    Install,
    /// Represents running a specific script.
    Run(String),
}

impl Npm {
    /// Checks if the specified lockfile exists in the provided file tree.
    pub fn lockfile_exists(app_file_tree: &FileTree, path: &Path) -> bool {
        let v = path
            .iter()
            .map(|s| s.to_os_string())
            .collect::<Vec<OsString>>();
        app_file_tree.path(&mut v.iter()).is_some()
    }

    /// Generates the command string for a given sub-command and optional workspace.
    pub fn run_command_string(sub_command: SubCommand, workspace: Option<&str>) -> String {
        match sub_command {
            SubCommand::Install => "npm install".to_string(),
            SubCommand::Run(script) => match workspace {
                Some(workspace) => format!("npm run {script} --workspace {workspace}"),
                None => format!("npm run {script}"),
            },
        }
    }
}

impl std::fmt::Display for Npm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "npm".bright_red())
    }
}

impl From<&str> for SubCommand {
    fn from(s: &str) -> Self {
        match s {
            "install" => SubCommand::Install,
            script => SubCommand::Run(script.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_run() {
        let expected_command = "npm install";
        let actual_command = Npm::run_command_string(SubCommand::Install, None);
        assert_eq!(expected_command, actual_command);

        let expected_command = "npm run package --workspace ui";
        let actual_command =
            Npm::run_command_string(SubCommand::Run("package".to_string()), Some("ui"));
        assert_eq!(expected_command, actual_command);
    }
}
