use crate::item;
use std::io;
use std::process::Command;

pub struct Callback {
    command: Command,
}

const QUERY_VAR_NAME: &str = "QUERY";

impl Callback {
    pub fn new(args: String) -> Self {
        let mut command = if cfg!(target_os = "windows") {
            Command::new("cmd")
        } else {
            let mut c = Command::new("sh");
            c.arg("-c");
            c
        };
        // Although args can contain multiple args, it is a string
        // passed directly to sh or cmd, so it is a single arg from
        // the Command perspective.
        command.arg(args);

        Self { command }
    }

    pub fn call(&mut self, query: &str) -> Vec<item::Item> {
        let output = self
            .command
            .env(QUERY_VAR_NAME, query)
            .output()
            .expect("Error running callback");
        item::parse_items(io::Cursor::new(output.stdout)).expect("Problem parsing callback output")
    }
}
