#![warn(rust_2018_idioms, clippy::pedantic)]
mod build;
mod clone;
mod run;

use anyhow::{bail, Result};
use build::build;
use clone::clone;
use colored::Colorize;
use run::run;
use which::which;

fn ensure_depends() -> Result<()> {
    match (which("docker"), which("nixpacks")) {
        (Ok(_), Ok(_)) => Ok(()),
        (Err(_), Ok(_)) => {
            bail!("Docker is a required dependency. Install from https://www.docker.com/".red())
        }
        (Ok(_), Err(_)) => {
            bail!("Nixpacks is a required dependency. Install from https://www.nixpacks.com/".red())
        }
        (Err(_), Err(_)) => bail!(
            "Docker and Nixpacks are required dependencies. Install from https://www.docker.com and https://www.nixpacks.com".red()
        ),
    }
}

fn main() -> Result<()> {
    ensure_depends()?;

    let mut args = std::env::args();
    args.next();
    let url = match args.next() {
        Some(arg) => arg,
        None => bail!(
            format!("{}\nUsage: {}", "Git URL not found".red(), "gitrun [url: URL to the Git Repository] (args: Arguments passed while running the Docker container)".green())
        ),
    };
    let rest = args.collect::<Vec<String>>();

    let cloned_dir = clone(&url)?;
    let build_name = build(&cloned_dir.0, cloned_dir.1)?;
    run(&build_name, rest)?;
    Ok(())
}
