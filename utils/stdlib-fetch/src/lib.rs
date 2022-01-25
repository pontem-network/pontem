use anyhow::{anyhow, Result};
use std::path::Path;
use std::fs;
use std::process::{Command, Stdio};

fn run(path: &str, cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd)
        .current_dir(path)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?
        .wait()?;

    if !status.success() {
        return Err(anyhow!("Failed to run {} {} {:?}", path, cmd, args));
    }

    Ok(())
}

pub struct FetchConfig<'a> {
    pub git_repo: &'a str,      // Git repository link.
    pub rev: Option<&'a str>,   // Git revision.
    pub path_to_clone: &'a str, // Path to clone repo.
    pub build_with_dove: bool,  // Build result repo with dove.
}

// Fetch standard library.
pub fn fetch(config: FetchConfig) -> Result<()> {
    let path: &Path = config.path_to_clone.as_ref();
    if path.exists() {
        fs::remove_dir_all(path)?;
    }

    run(
        ".",
        "git",
        &["clone", config.git_repo, config.path_to_clone],
    )?;

    if config.rev.is_some() {
        run(
            config.path_to_clone,
            "git",
            &["checkout", config.rev.unwrap()],
        )?;
    }

    if config.build_with_dove {
        run(config.path_to_clone, "dove", &["build", "-b"])?;
    }

    Ok(())
}
