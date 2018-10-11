use std::io::{self, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut line = String::new();
        print!("> ");
        stdout.flush()?;
        stdin.read_line(&mut line)?;
        let command: String = line.split_whitespace().take(1).collect();
        let args: Vec<_> = line.split_whitespace().skip(1).collect();
        let output = Command::new(command).args(&args).output()?;
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
