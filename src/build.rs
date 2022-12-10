use std::{path::PathBuf, process::Command};

use anyhow::Ok;
use colored::Colorize;
use nanoid::nanoid;
use spinners::Spinner;

pub fn build(repo_name: &str, cloned_dir: PathBuf) -> anyhow::Result<String> {
    let mut dockerfile = PathBuf::from(&cloned_dir);
    dockerfile.push("Dockerfile");

    let image_name = format!("{}-{}", &repo_name, nanoid!(18).to_lowercase());


    if dockerfile.exists() {
        let mut sp = Spinner::new(
            spinners::Spinners::Arc,
            format!("{}","Building image from Dockerfile".blue()),
        );
        Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg(&image_name)
            .arg(cloned_dir)
            .output()?;
        sp.stop_with_message(format!("{}","Built image from Dockerfile".bright_blue()));
        return Ok(image_name);
    }
    
    let mut sp = Spinner::new(
        spinners::Spinners::Arc,
        format!("{}","Building image from source".blue()),
    );
    Command::new("nixpacks")
        .arg("build")
        .arg(cloned_dir)
        .arg("--name")
        .arg(&image_name)
        .output()?;
    
    sp.stop_with_message(format!("{}","Built image from source".bright_blue()));

    Ok(image_name)
}
