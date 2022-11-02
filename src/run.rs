use anyhow::Result;
use std::process::{Command, Stdio};

pub fn run(image_name: &str) -> Result<()> {
    println!("Running image {}\n", image_name);

    Command::new("docker")
        .arg("run")
        .arg(image_name)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}
