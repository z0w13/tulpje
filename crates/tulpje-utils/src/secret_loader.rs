use std::collections::HashMap;
use std::fs;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn get_secrets(path: &str) -> HashMap<String, String> {
    let secrets_dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(err) => {
            println!("error accessing secrets dir {path}: {err}");
            return HashMap::new();
        }
    };

    let mut env_vars = HashMap::new();
    for entry in secrets_dir {
        let Ok(entry) = entry else {
            println!("error reading path {}", entry.unwrap_err());
            continue;
        };

        if entry.file_name().to_string_lossy().starts_with(".") {
            // skip hidden files
            println!("skipping hidden {}", entry.file_name().to_string_lossy());
            continue;
        }

        let var_name = entry.file_name().to_string_lossy().to_ascii_uppercase();
        let var_value = fs::read_to_string(entry.path()).expect("couldn't read file");

        println!("     - {}", var_name);
        env_vars.insert(var_name, var_value.trim().to_owned());
    }
    env_vars
}

fn main() {
    let path = std::env::var("SECRET_LOADER_PATH").unwrap_or(String::from("/run/secrets"));

    println!("* injecting env vars from secrets in {} ...", path);

    let env_vars = get_secrets(&path);
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let (cmd, args) = args.split_first().expect("no command specified");

    println!(
        "error starting command: {}",
        Command::new(cmd).envs(&env_vars).args(args).exec()
    );
}
