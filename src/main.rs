use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write, stdin};

fn echo(inputs: Vec<&str>) {
    for arg in inputs.iter().skip(1) {
        print!("{} ", arg);
    }
    println!();
    io::stdout().flush().unwrap();
}

fn main() {
    let mut commands: HashMap<&str, i16> = HashMap::new();
    commands.insert("exit", 0);
    commands.insert("echo", 1);

    let mut input_command = String::new();

    while true {
        print!("$ ");
        io::stdout().flush().unwrap();

        input_command.clear();

        io::stdin()
            .read_line(&mut input_command)
            .expect("Failed to read line");

        let inputs: Vec<&str> = input_command.trim().split(' ').collect();

        if inputs.is_empty() {
            continue;
        }

        let command = inputs[0];

        if (!commands.contains_key(command)) {
            print!("{}: command not found\n", command);
            io::stdout().flush().unwrap();
            continue;
        }

        match commands.get(command) {
            Some(&0) => break,
            Some(&1) => echo(inputs),
            Some(&_) => (),
            None => (),
        }
    }
}
