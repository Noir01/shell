use std::{io::{self, stdin, Write}, process::Command};

fn main() {
    loop {
        let mut input = String::new();
        print!("shell> ");
        io::stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        match Command::new(command).args(&args).status() {
            Ok(status) => {
                if !status.success() {
                    eprintln!("Command exited with status: {}", status);
                }
            }
            Err(e) => {
                eprintln!("Failed to execute command: {}", e);
            }
        }
    }
}
