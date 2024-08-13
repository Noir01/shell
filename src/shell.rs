use std::{env, io::{self, Write}};

pub struct Command {
    name: String,
    args: Vec<String>,
    stdin: Option<String>,
    stdout: Option<String>,
    stderr: Option<String>,
}

pub struct Shell {
}

impl Shell {
    pub fn new() -> Self {
        Shell {}
    }

    pub fn init(&mut self) {
        unsafe {
            libc::signal(libc::SIGINT, Self::handle_interrupt as libc::sighandler_t);
        }

        env::set_var("PS1", String::from("$ "));

        self.run();
    }

    pub fn run(&mut self) {
        loop {
            // Emit PS1
            print!("{} ", env::var("PS1").unwrap());
            io::stdout().flush().unwrap();

            // Read input
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let commands: Vec<Command> = self.parse(input);

            for command in commands {
                if command.name.to_lowercase() == "exit" {
                    return;
                }
            }
        }
    }

    fn parse(&self, input: String) -> Vec<Command> {
        let mut res: Vec<Command> = Vec::new();

        let input = input.trim();
    
        let mut tokens: Vec<String> = Vec::new();
        let mut current_token: String = String::new();
        let mut in_quotes: bool = false;
        let mut escape_next: bool = false;

        for c in input.chars() {
            if escape_next {
                current_token.push(c);
                escape_next = false;
            }
            else if c == '\\' {
                escape_next = true;
            }
            else if c == '"' {
                if in_quotes {
                    tokens.push(current_token);
                    current_token = String::new();
                }
                in_quotes = !in_quotes;
            }
            else if (c == ' ' || c == '\t' || c == '\n') && !in_quotes {
                if current_token.len() > 0 {
                    tokens.push(current_token);
                    current_token = String::new();
                }
            }
            else {
                current_token.push(c);
            }
        }

        if current_token.len() > 0 {
            tokens.push(current_token);
        }

        let mut arguments: Vec<String> = Vec::new();
        let mut current_command: Option<Command> = None;

        let mut token_iter = tokens.iter();
        while let Some(token) = token_iter.next() {
            match token.as_str() {
                "|" => {
                    if let Some(command) = current_command {
                        res.push(command);
                    }
                    current_command = None;
                    arguments.clear();
                }
                "2>" => {
                    if let Some(next_token) = token_iter.next() {
                        if let Some(ref mut command) = current_command {
                            command.stderr = Some(next_token.clone());
                        }
                    } else {
                        // TODO: Raise error
                    }
                }
                ">" => {
                    if let Some(next_token) = token_iter.next() {
                        if let Some(ref mut command) = current_command {
                            command.stdout = Some(next_token.clone());
                        }
                    } else {
                        // TODO: Raise error
                    }
                }
                "<" => {
                    if let Some(next_token) = token_iter.next() {
                        if let Some(ref mut command) = current_command {
                            command.stdin = Some(next_token.clone());
                        }
                    } else {
                        // TODO: Raise error
                    }
                }
                _ => {
                    if current_command.is_none() {
                        current_command = Some(Command {
                            name: token.clone(),
                            args: Vec::new(),
                            stdin: None,
                            stdout: None,
                            stderr: None,
                        });
                    } else {
                        arguments.push(token.clone());
                    }
                }
            }
        }

        if !current_command.is_none() {
            res.push(current_command.unwrap());
        }
    
        // Add return statement
        res
    }

    extern "C" fn handle_interrupt(_sig: libc::c_int) {
        
    }
}

