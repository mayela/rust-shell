use std::io::{self, Write};
use rshell::{read_command, execute_command, parse_command};

fn main(){
    loop {
        print!("-> ");
        io::stdout().flush().unwrap();
        let command = read_command();
        let parsed_result = parse_command(&command);
        execute_command(&parsed_result);
    }
}
