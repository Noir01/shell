use std::{env, io::{self, Write}};

pub struct Shell {
}

impl Shell {
    pub fn new() -> Self {
        Shell {}
    }

    pub fn init(&mut self) -> () {
        unsafe {
            libc::signal(libc::SIGINT, Self::handle_interrupt as libc::sighandler_t);
        }

        env::set_var("PS1", String::from("$ "));

        self.run();
    }

    pub fn run(&mut self) {
        loop {
            print!("$ ");
            io::stdout().flush().unwrap();
        }
    }

    extern "C" fn handle_interrupt(_sig: libc::c_int) {
        
    }
}

