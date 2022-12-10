use anyhow::Result;
use colored::Colorize;
use std::process::{Command, Stdio};

pub fn run(image_name: &str, rest: Vec<String>) -> Result<()> {
    println!(
        "{}",
        format!("Running image {}\n", image_name).bright_blue()
    );

    Command::new("docker")
        .arg("run")
        .args(rest)
        .arg(image_name)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}
