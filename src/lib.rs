use std::io;
use std::process::{Command, Stdio};


pub fn read_command() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)
        .expect("Failed to read line");
    buffer
}

pub fn parse_command(result: &String) -> Vec<&str> {
    let parsed_result: Vec<&str> = result.split_whitespace()
                                         .collect();
    parsed_result
}

pub fn execute_command(command: &Vec<&str>) {
    Command::new(command[0])
        .stdout(Stdio::inherit())
        .output()
        .expect("failed");
}

