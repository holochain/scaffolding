use std::{ffi::OsString, path::Path};

use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::FileTree,
};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum PackageManager {
    Bun,
    #[default]
    Npm,
    Pnpm,
    Yarn,
}

impl PackageManager {
    pub fn choose() -> ScaffoldResult<PackageManager> {
        let managers = [
            PackageManager::Bun,
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
            PackageManager::Bun => "bun install".to_string(),
            PackageManager::Npm => "npm install".to_string(),
            PackageManager::Pnpm => "pnpm add".to_string(),
            PackageManager::Yarn => "yarn add".to_string(),
        }
    }

    pub fn run_script(&self, script: &str) -> String {
        match self {
            PackageManager::Bun => format!("bun run {script}"),
            PackageManager::Npm => format!("npm run {script}"),
            PackageManager::Pnpm => format!("pnpm run {script}"),
            PackageManager::Yarn => format!("yarn {script}"),
        }
    }

    pub fn run_workspace_script(&self, workspace: &str, script: &str) -> String {
        match self {
            PackageManager::Bun => format!("bun run --filter {workspace} {script}"),
            PackageManager::Npm => format!("npm run {script} --workspace {workspace}"),
            PackageManager::Yarn => format!("yarn workspace {workspace} {script}"),
            PackageManager::Pnpm => format!("pnpm --filter {workspace} {script}"),
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
                "Invalid value: {value}, expected vanilla, svelte, vue, lit or headless"
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
}
