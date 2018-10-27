use rshell;

fn main() {
    loop {
        rshell::print_shell_prompt();
        let commands = rshell::read_commands();
        let parsed_commands = rshell::parse_commands(&commands);
        rshell::execute_commands(parsed_commands);
    }
}
