use std::{ffi::OsString, path::Path};

use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use serde::Serialize;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::FileTree,
};

/// Represents different package managers that can be used.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub enum PackageManager {
    Bun,
    #[default]
    Npm,
    Pnpm,
    Yarn,
}

/// Represents sub-commands that can be executed by a package manager.
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub enum SubCommand {
    /// Represents the install command.
    Install,
    /// Represents running a specific script.
    Run(String),
}

impl PackageManager {
    pub fn choose() -> ScaffoldResult<PackageManager> {
        let managers = [
            PackageManager::Bun,
            PackageManager::Npm,
            PackageManager::Pnpm,
            PackageManager::Yarn,
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose a package manager: (Use arrow-keys. Return to submit)")
            .default(0)
            .items(&managers)
            .interact()?;
        Ok(managers[selection])
    }

    /// Checks if the specified lockfile exists in the provided file tree.
    pub fn lockfile_exists(app_file_tree: &FileTree, path: &Path) -> bool {
        let v = path
            .iter()
            .map(|s| s.to_os_string())
            .collect::<Vec<OsString>>();
        app_file_tree.path(&mut v.iter()).is_some()
    }

    /// Generates the command string for a given sub-command and optional workspace.
    pub fn run_command_string(&self, sub_command: SubCommand, workspace: Option<&str>) -> String {
        match sub_command {
            SubCommand::Install => self.install_command_string().to_string(),
            SubCommand::Run(script) => match self {
                PackageManager::Bun => match workspace {
                    Some(workspace) => format!("bun run --filter {workspace} {script}"),
                    None => format!("bun run {}", script),
                },
                PackageManager::Npm => match workspace {
                    Some(workspace) => format!("npm run {script} --workspace {workspace}"),
                    None => format!("npm run {}", script),
                },
                PackageManager::Pnpm => match workspace {
                    Some(workspace) => format!("pnpm --filter {workspace} {script}"),
                    None => format!("pnpm run {script}"),
                },
                PackageManager::Yarn => match workspace {
                    Some(workspace) => format!("yarn workspace {workspace} {script}"),
                    None => format!("yarn {script}"),
                },
            },
        }
    }

    fn install_command_string(&self) -> &str {
        match self {
            PackageManager::Npm => "npm install",
            PackageManager::Yarn => "yarn install",
            PackageManager::Pnpm => "pnpm install",
            PackageManager::Bun => "bun install",
        }
    }
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PackageManager::Bun => "bun".truecolor(139, 69, 19),
            PackageManager::Npm => "npm".bright_red(),
            PackageManager::Pnpm => "pnpm".red().yellow(),
            PackageManager::Yarn => "yarn".cyan(),
        };
        write!(f, "{str}")
    }
}

impl std::str::FromStr for PackageManager {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> ScaffoldResult<PackageManager> {
        match s.to_ascii_lowercase().as_str() {
            "bun" => Ok(PackageManager::Bun),
            "npm" => Ok(PackageManager::Npm),
            "pnpm" => Ok(PackageManager::Pnpm),
            "yarn" => Ok(PackageManager::Yarn),
            value => Err(ScaffoldError::MalformedTemplate(format!(
                "Invalid value: {value}, expected bun, npm, pnpm, or yarn"
            ))),
        }
    }
}

impl TryFrom<&FileTree> for PackageManager {
    type Error = ScaffoldError;

    fn try_from(app_file_tree: &FileTree) -> ScaffoldResult<PackageManager> {
        if PackageManager::lockfile_exists(app_file_tree, Path::new("bun.lockb")) {
            Ok(PackageManager::Bun)
        } else if PackageManager::lockfile_exists(app_file_tree, Path::new("package-lock.json")) {
            Ok(PackageManager::Npm)
        } else if PackageManager::lockfile_exists(app_file_tree, Path::new("pnpm-lock.yaml")) {
            Ok(PackageManager::Pnpm)
        } else if PackageManager::lockfile_exists(app_file_tree, Path::new("yarn.lock")) {
            Ok(PackageManager::Yarn)
        } else {
            PackageManager::choose()
        }
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
    use build_fs_tree::{dir, file};

    use super::*;

    fn setup_filetree(lockfile: &str) -> FileTree {
        dir! {
            ".github" => dir! {},
            "dnas" => dir! {},
            "tests" => dir! {},
            "ui" => dir! {},
            "workdir" => dir! {},
            ".gitignore" => file!(""),
            "Cargo.toml" => file!(""),
            "flake.nix" => file!(""),
            lockfile => file!(""),
            "package.json" => file!(""),
            "README.md" => file!(""),
        }
    }

    #[test]
    fn test_try_from_file_tree_bun() {
        let app_file_tree = setup_filetree("bun.lockb");
        let package_manager = PackageManager::try_from(&app_file_tree).unwrap();
        assert_eq!(package_manager, PackageManager::Bun);
    }

    #[test]
    fn test_try_from_file_tree_npm() {
        let app_file_tree = setup_filetree("package-lock.json");
        let package_manager = PackageManager::try_from(&app_file_tree).unwrap();
        assert_eq!(package_manager, PackageManager::Npm);
    }

    #[test]
    fn test_try_from_file_tree_yarn() {
        let app_file_tree = setup_filetree("yarn.lock");
        let package_manager = PackageManager::try_from(&app_file_tree).unwrap();
        assert_eq!(package_manager, PackageManager::Yarn);
    }

    #[test]
    fn test_try_from_file_tree_pnpm() {
        let app_file_tree = setup_filetree("pnpm-lock.yaml");
        let package_manager = PackageManager::try_from(&app_file_tree).unwrap();
        assert_eq!(package_manager, PackageManager::Pnpm);
    }

    #[test]
    fn test_run_with_npm() {
        let app_file_tree = setup_filetree("package-lock.json");
        let package_manager = PackageManager::try_from(&app_file_tree).unwrap();
        let expected_command = "npm install";
        let actual_command = package_manager.run_command_string(SubCommand::Install, None);
        assert_eq!(expected_command, actual_command);

        let expected_command = "npm run package --workspace ui";
        let actual_command =
            package_manager.run_command_string(SubCommand::Run("package".to_string()), Some("ui"));
        assert_eq!(expected_command, actual_command);
    }

    #[test]
    fn test_run_with_yarn() {
        let app_file_tree = setup_filetree("yarn.lock");
        let package_manager = PackageManager::try_from(&app_file_tree).unwrap();
        let expected_command = "yarn install";
        let actual_command = package_manager.run_command_string(SubCommand::Install, None);
        assert_eq!(expected_command, actual_command);

        let expected_command = "yarn workspace ui package";
        let actual_command =
            package_manager.run_command_string(SubCommand::Run("package".to_string()), Some("ui"));
        assert_eq!(expected_command, actual_command);
    }

    #[test]
    fn test_run_with_pnpm() {
        let app_file_tree = setup_filetree("pnpm-lock.yaml");
        let package_manager = PackageManager::try_from(&app_file_tree).unwrap();
        let expected_command = "pnpm install";
        let actual_command = package_manager.run_command_string(SubCommand::Install, None);
        assert_eq!(expected_command, actual_command);

        let expected_command = "pnpm --filter ui package";
        let actual_command =
            package_manager.run_command_string(SubCommand::Run("package".to_string()), Some("ui"));
        assert_eq!(expected_command, actual_command);
    }

    #[test]
    fn test_run_with_bun() {
        let app_file_tree = setup_filetree("bun.lockb");
        let package_manager = PackageManager::try_from(&app_file_tree).unwrap();
        let expected_command = "bun install";
        let actual_command = package_manager.run_command_string(SubCommand::Install, None);
        assert_eq!(expected_command, actual_command);

        let expected_command = "bun run --filter ui package";
        let actual_command =
            package_manager.run_command_string(SubCommand::Run("package".to_string()), Some("ui"));
        assert_eq!(expected_command, actual_command);
    }
}
