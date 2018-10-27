use std::io::{self, Write};
use rshell;

fn main(){
    loop {
        print!("-> ");
        io::stdout().flush().unwrap();
        let commands = rshell::read_commands();
        let parsed_commands = rshell::parse_commands(&commands);
        rshell::execute_commands(parsed_commands);
    }
}
