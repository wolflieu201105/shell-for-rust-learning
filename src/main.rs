use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write, stdin};

fn main() {
    let mut commands: HashMap<String, i32> = HashMap::new();

    let mut input_command = String::new();

    while true {
        print!("$ ");
        io::stdout().flush().unwrap();

        input_command.clear();

        io::stdin()
            .read_line(&mut input_command)
            .expect("Failed to read line");

        input_command = input_command.trim().to_string();

        if (!commands.contains_key(&input_command)) {
            print!("{}: command not found\n", input_command);
            io::stdout().flush().unwrap();
        }
    }
}
