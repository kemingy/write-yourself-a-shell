use std::io::{self, Write};
use std::process::Command;

#[derive(Debug)]
enum Error {
    NoBinary,
}

// A command consists of a binary and its arguments
struct Cmd<'a> {
    binary: &'a str,
    args: Vec<&'a str>,
}

impl<'a> Cmd<'a> {
    // Extract the command and its arguments from the commandline
    fn extract_from(line: &'a str) -> Result<Self, Error> {
        let mut parts = line.split_whitespace();
        let binary = parts.nth(0).ok_or_else(|| Error::NoBinary)?;
        let args = parts.collect();

        Ok(Cmd { binary, args })
    }
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut line = String::new();
        print!("> ");
        stdout.flush()?;
        stdin.read_line(&mut line)?;

        match Cmd::extract_from(&line) {
            Ok(cmd) => {
                let output = Command::new(cmd.binary).args(cmd.args).output()?;
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }
            Err(Error::NoBinary) => {}
        }
    }
}
