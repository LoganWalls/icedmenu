use crate::item;
use std::io;
use std::process::Command;

pub struct Callback {
    program: String,
    args: Vec<String>,
}

const QUERY_VAR_NAME: &str = "$QUERY";

impl Callback {
    pub fn new(cli_args: Vec<String>) -> Self {
        let program = cli_args
            .get(0)
            .unwrap_or_else(|| unreachable!("Clap should force at least one argument for callback"))
            .to_string();
        let args = cli_args.iter().skip(1).map(String::from).collect();
        Self { program, args }
    }

    pub fn call(&mut self, query: &str) -> Vec<item::Item> {
        let output = Command::new(&self.program)
            .args(
                self.args
                    .iter()
                    .map(|a| if a == QUERY_VAR_NAME { query } else { a }),
            )
            .output()
            .expect("Error running callback");
        item::parse_items(io::Cursor::new(output.stdout)).expect("Problem parsing callback output")
    }
}
