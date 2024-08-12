use std::{env, io::{self, stdin, Write}, iter::Iterator, path::Path, process::Command};

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

        match command {
            "cd" => {
                let new_dir = args.iter().peekable().peek().map_or("/", |x| *x); // Use iter() to convert args into an iterator
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(root) {
                    eprintln!("{}", e);
                }
            },
            "exit" => return,
            command => match Command::new(command).args(&args).status() {
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
}
