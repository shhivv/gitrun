use anyhow::{self, bail, Ok};
use colored::Colorize;
use regex::Regex;
use spinners::Spinner;
use std::{fs, path::PathBuf, process::Command};

pub fn clone(url: &str) -> anyhow::Result<(String, PathBuf)> {
    let base = std::env::var("UserProfile").unwrap_or_else(|_| "~".to_string());
    let mut gitrun_home = PathBuf::from(base);
    gitrun_home.push(".gitrun");

    if !gitrun_home.exists() {
        fs::create_dir(&gitrun_home)?;
    }

    // https://stackoverflow.com/a/56144879/15445312
    let re = Regex::new(
        r"((git@|http(s)?://)([\w\.@]+)(/|:))([\w,\-,_]+)/([\w,\-,_]+)(.git){0,1}((/){0,1})",
    )?;
    re.captures(url).unwrap();
    let repo_name: &str;
    if let Some(caps) = re.captures(url) {
        if let Some(name) = caps.get(7) {
            repo_name = name.as_str();
        } else {
            bail!("Invalid Git repository URL".red())
        }
    } else {
        bail!("Invalid Git repository URL".red())
    }

    let mut cloned_dir = gitrun_home;
    cloned_dir.push(repo_name);

    if cloned_dir.exists() {
        return Ok((repo_name.to_string(), cloned_dir));
    }

    let mut sp = Spinner::new(
        spinners::Spinners::Arc,
        format!("{}", format!("{} {}", "Cloning", url).blue()),
    );

    Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(&cloned_dir)
        .output()?;

    sp.stop_with_message(format!("{}", format!("{} {}", "Cloned", url).bright_blue()));

    Ok((repo_name.to_string(), cloned_dir))
}
