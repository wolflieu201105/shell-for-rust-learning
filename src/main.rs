use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write, stdin};

fn main() {
    let mut commands: HashMap<&str, i16> = HashMap::new();
    commands.insert("exit", 0);

    let mut input_command = String::new();

    while true {
        print!("$ ");
        io::stdout().flush().unwrap();

        input_command.clear();

        io::stdin()
            .read_line(&mut input_command)
            .expect("Failed to read line");

        let input: &str = input_command.trim();

        if (!commands.contains_key(&input)) {
            print!("{}: command not found\n", input);
            io::stdout().flush().unwrap();
            continue;
        }

        match commands.get(input) {
            Some(&0) => break,
            Some(&_) => (),
            None => (),
        }
    }
}
