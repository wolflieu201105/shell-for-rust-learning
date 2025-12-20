use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write, stdin};

enum Command {
    Empty = 0,
    Exit = 1,
    Echo = 2,
    Type = 3,
}

fn command_to_code(input: &str) -> Option<Command> {
    match input {
        "" => Some(Command::Empty),
        "exit" => Some(Command::Exit),
        "echo" => Some(Command::Echo),
        "type" => Some(Command::Type),
        _ => None,
    }
}

fn not_command(inputs: Vec<&str>) {
    print!("{}: command not found\n", inputs[0]);
    io::stdout().flush().unwrap();
}

fn echo_command(inputs: Vec<&str>) {
    for arg in inputs.iter().skip(1) {
        print!("{} ", arg);
    }
    println!();
    io::stdout().flush().unwrap();
}

fn type_command(inputs: Vec<&str>) {
    for arg in inputs.iter().skip(1) {
        if command_to_code(arg).is_none() {
            println!("{}: not found", arg);
        } else {
            println!("{} is a shell builtin", arg);
        }
    }
    io::stdout().flush().unwrap();
}

fn main() {

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

        let command_code = command_to_code(command);

        match command_code {
            Some(Command::Empty) => (),
            Some(Command::Exit) => break,
            Some(Command::Echo) => echo_command(inputs),
            Some(Command::Type) => type_command(inputs),
            None => not_command(inputs),
        }
    }
}
