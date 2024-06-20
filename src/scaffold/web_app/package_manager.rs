use std::{ffi::OsString, path::Path};

use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::FileTree,
};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum PackageManager {
    #[default]
    Npm,
    Pnpm,
    Yarn,
}

impl PackageManager {
    pub fn choose() -> ScaffoldResult<PackageManager> {
        let managers = [
            PackageManager::Npm,
            PackageManager::Yarn,
            PackageManager::Pnpm,
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose a package manager: (Use arrow-keys. Return to submit)")
            .default(0)
            .items(&managers)
            .interact()?;
        Ok(managers[selection])
    }

    pub fn lockfile_exists(app_file_tree: &FileTree, path: &Path) -> bool {
        let v = path
            .iter()
            .map(|s| s.to_os_string())
            .collect::<Vec<OsString>>();
        app_file_tree.path(&mut v.iter()).is_some()
    }

    pub fn install_script(&self) -> String {
        match self {
            PackageManager::Npm => format!("npm install"),
            PackageManager::Pnpm => format!("pnpm add"),
            PackageManager::Yarn => format!("yarn add"),
        }
    }

    pub fn run_script(&self, script: &str) -> String {
        match self {
            PackageManager::Npm => format!("npm run {script}"),
            PackageManager::Pnpm => format!("pnpm run {script}"),
            PackageManager::Yarn => format!("yarn {script}"),
        }
    }

    pub fn run_workspace_script(&self, workspace: &str, script: &str) -> String {
        match self {
            PackageManager::Npm => format!("npm run {script} --workspace {workspace}"),
            PackageManager::Yarn => format!("yarn workspace {workspace} {script}"),
            PackageManager::Pnpm => format!("pnpm run {script} --filter {workspace}"),
        }
    }
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PackageManager::Npm => "npm".bright_red(),
            PackageManager::Pnpm => "pnpm".red().on_yellow(),
            PackageManager::Yarn => "yarn".cyan(),
        };
        write!(f, "{str}")
    }
}

impl std::str::FromStr for PackageManager {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> ScaffoldResult<PackageManager> {
        match s.to_ascii_lowercase().as_str() {
            "npm" => Ok(PackageManager::Npm),
            "pnpm" => Ok(PackageManager::Pnpm),
            "yarn" => Ok(PackageManager::Yarn),
            value => Err(ScaffoldError::MalformedTemplate(format!(
                "Invalid value: {value}, expected vanilla, svelte, vue, lit or headless"
            ))),
        }
    }
}

impl TryFrom<&FileTree> for PackageManager {
    type Error = ScaffoldError;

    fn try_from(app_file_tree: &FileTree) -> ScaffoldResult<PackageManager> {
        let npm_lockfile = Path::new("package-lock.json");
        let yarn_lockfile = Path::new("yarn.lock");
        let pnpm_lockfile = Path::new("pnpm-lock.yaml");

        if PackageManager::lockfile_exists(app_file_tree, &npm_lockfile) {
            Ok(PackageManager::Npm)
        } else if PackageManager::lockfile_exists(app_file_tree, &yarn_lockfile) {
            Ok(PackageManager::Yarn)
        } else if PackageManager::lockfile_exists(app_file_tree, &pnpm_lockfile) {
            Ok(PackageManager::Pnpm)
        } else {
            PackageManager::choose()
        }
    }
}

#[cfg(test)]
mod tests {
    use build_fs_tree::{dir, file};

    use super::*;

    fn setup_file_tree_with_lockfile(lockfile: &str) -> FileTree {
        dir! {
            ".gitignore" => file!(""),
            "workdir" => dir!{},
            "Cargo.toml" => file!(""),
            "package.json" => file!(""),
            lockfile => file!(""),
            "dnas" => dir! {},
        }
    }

    #[test]
    fn test_try_from_file_tree_npm() {
        let app_file_tree = setup_file_tree_with_lockfile("package-lock.json");
        let package_manager = PackageManager::try_from(&app_file_tree).unwrap();
        assert_eq!(package_manager, PackageManager::Npm);
    }

    #[test]
    fn test_try_from_file_tree_yarn() {
        let app_file_tree = setup_file_tree_with_lockfile("yarn.lock");
        let package_manager = PackageManager::try_from(&app_file_tree).unwrap();
        assert_eq!(package_manager, PackageManager::Yarn);
    }

    #[test]
    fn test_try_from_file_tree_pnpm() {
        let app_file_tree = setup_file_tree_with_lockfile("pnpm-lock.yaml");
        let package_manager = PackageManager::try_from(&app_file_tree).unwrap();
        assert_eq!(package_manager, PackageManager::Pnpm);
    }
}
