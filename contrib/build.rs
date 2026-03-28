use std::env;
use std::path::PathBuf;
use std::process::Command;

fn check_output(prog: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let result = Command::new(prog).args(args).output()?;
    if !result.status.success() {
        return Err(format!(
            "command exited unsuccesfully ({})",
            result
                .status
                .code()
                .map(|code| code.to_string())
                .unwrap_or_else(|| "??".to_string()),
        )
        .into());
    }

    Ok(String::from_utf8(result.stdout)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=../../contrib/build.rs");

    // sqlx migrations in tulpje-handler
    if env::var("CARGO_PKG_NAME").is_ok_and(|name| name == "tulpje-handler") {
        println!("cargo::rerun-if-changed=../../migrations");
    }

    if PathBuf::from("../../.git").is_dir() {
        println!("cargo::rerun-if-changed=../../.git/HEAD");
        println!(
            "cargo::rerun-if-changed=../../.git/{}",
            check_output("git", &["symbolic-ref", "HEAD"])?
        );
    }

    if env::var("TULPJE_VERSION_EXTRA").is_err() {
        let rev = check_output("git", &["rev-parse", "--short", "HEAD"])?;
        let dirty = !check_output("git", &["status", "--porcelain"])?.is_empty();

        println!(
            "cargo::rustc-env=TULPJE_VERSION_EXTRA={}{}",
            rev,
            if dirty { "-dirty" } else { "" }
        );
    }

    Ok(())
}
