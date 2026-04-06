#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let builtin_commands = ["echo", "type", "exit"];

     loop {
     print!("$ ");

     io::stdout().flush().unwrap();

     

     let mut command = String::new();
     io::stdin().read_line(&mut command).unwrap();
    command = command.trim().to_string();
    if command == "exit" {break;};

    if command.starts_with("echo ") {
    println!("{}", &command[5..]);
    continue;
    }

    if command.starts_with("type ") {
        let target = &command[5..];
        if builtin_commands.contains(&target) {
    println!("{} is a shell builtin", target);
        } else {
    println!("{}: not found", target);
        }
        continue;
    }

    println!("{}: command not found", command.trim());
    }    
}
