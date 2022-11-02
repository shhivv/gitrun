use std::{path::PathBuf, process::Command};

use anyhow::Ok;
use nanoid::nanoid;

pub fn build(repo_name: &str, cloned_dir: PathBuf) -> anyhow::Result<String> {
    let mut dockerfile = PathBuf::from(&cloned_dir);
    dockerfile.push("Dockerfile");

    let image_name = format!("{}-{}", &repo_name, nanoid!(18).to_lowercase());

    if dockerfile.exists() {
        println!("Dockerfile found. Building image.");
        Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg(&image_name)
            .arg(cloned_dir)
            .output()?;
        return Ok(image_name);
    }

    println!("Dockerfile not found. Building from source.");

    Command::new("nixpacks")
        .arg("build")
        .arg(cloned_dir)
        .arg("--name")
        .arg(&image_name)
        .output()?;

    Ok(image_name)
}
