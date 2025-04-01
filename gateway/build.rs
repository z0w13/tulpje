use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-env-changed=TULPJE_VERSION_EXTRA");

    if PathBuf::from(".git").is_dir() {
        println!("cargo::rerun-if-changed=.git/HEAD");
        println!(
            "cargo::rerun-if-changed=.git/{}",
            String::from_utf8(
                Command::new("git")
                    .args(["symbolic-ref", "HEAD"])
                    .output()?
                    .stdout,
            )?
        );
    }

    if env::var("TULPJE_VERSION_EXTRA").is_err() {
        let rev = String::from_utf8(
            Command::new("git")
                .args(["rev-parse", "--short", "HEAD"])
                .output()?
                .stdout,
        )?;
        let dirty = !Command::new("git")
            .args(["status", "--porcelain"])
            .output()?
            .stdout
            .is_empty();

        println!(
            "cargo::rustc-env=TULPJE_VERSION_EXTRA={}{}",
            rev,
            if dirty { "-dirty" } else { "" }
        );
    }

    Ok(())
}
