use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

const BUILTIN_COMMANDS: [&str; 4] = ["exit", "echo", "type", "pwd"];

fn main() {
    loop {
        print!("Dan^_^: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        if tokens[0] == "exit" {
            break;
        }

        eval_command(tokens[0], &tokens[1..]);
    }
}

fn eval_command(command: &str, args: &[&str]) {
    if command == "echo" {
        println!("{}", args.join(" "));
        return;
    }

    if command == "type" {
        let Some(target) = args.first().copied() else {
            println!("type: not found");
            return;
        };

        if BUILTIN_COMMANDS.contains(&target) {
            println!("{} is a shell builtin", target);
        } else if let Some(path) = find_executable_in_path(target) {
            println!("{} is {}", target, path.display());
        } else {
            println!("{}: not found", target);
        }
        return;
    }

    if command == "pwd" {
        match env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(err) => eprintln!("pwd: {}", err),
        }
        return;
    }

    if let Some(path) = find_executable_in_path(command) {
        if let Err(err) = Command::new(path).args(args).status() {
            eprintln!("{}: {}", command, err);
        }
    } else {
        println!("{}: command not found", command);
    }
}

fn find_executable_in_path(command: &str) -> Option<PathBuf> {
    let path_var = env::var("PATH").ok()?;

    for dir in path_var.split(':') {
        let candidate = PathBuf::from(dir).join(command);
        let Ok(metadata) = fs::metadata(&candidate) else {
            continue;
        };

        if metadata.is_file() && (metadata.permissions().mode() & 0o111) != 0 {
            return Some(candidate);
        }
    }

    None
}
