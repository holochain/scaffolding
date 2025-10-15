use std::path::Path;

use colored::Colorize;
use git2::{IndexAddOption, Repository, RepositoryInitOptions};

use crate::error::ScaffoldResult;

pub fn setup_git_environment<P: AsRef<Path>>(path: P) -> ScaffoldResult<()> {
    if let Err(e) = (|| {
        let repo = Repository::init_opts(path, RepositoryInitOptions::new().initial_head("main"))?;
        let mut index = repo.index()?;
        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
        index.write()?;
        Ok::<_, git2::Error>(())
    })() {
        println!(
            "{}{}",
            "Warning: Failed to set up git repository: ".yellow(),
            e.to_string().yellow()
        );
    }

    Ok(())
}

pub fn is_inside_git_repo<P: AsRef<Path>>(dir: P) -> bool {
    Repository::open(dir).is_ok()
}

pub fn gitignore() -> &'static str {
    include_str!("./gitignore")
}
