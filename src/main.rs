use std::io::{self, Write};
use std::process::Command;

// Extract the command and its arguments from the commandline
fn get_command(line: String) -> (String, Vec<String>) {
    let command: String = line.split_whitespace().take(1).collect();
    let args: Vec<String> = line.split_whitespace().skip(1).map(String::from).collect();
    (command, args)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut line = String::new();
        print!("> ");
        stdout.flush()?;
        stdin.read_line(&mut line)?;
        let (command, args) = get_command(line);
        let output = Command::new(command).args(&args).output()?;
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
