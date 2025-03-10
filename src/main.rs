use std::io::{self, IsTerminal, Write};
use std::process::Command;

static PROMPT: &[u8] = ">> ".as_bytes();
static EXIT: &str = "exit";

#[derive(Debug)]
struct Cmd {
    program: String,
    args: Vec<String>,
}

impl Cmd {
    fn from_string(text: &str) -> Self {
        let mut chunks = text.split_whitespace();
        Self {
            program: chunks.next().unwrap().to_string(),
            args: chunks.map(|s| s.to_string()).collect(),
        }
    }

    pub fn run(&self) -> io::Result<()> {
        match Command::new(&self.program).args(&self.args).spawn() {
            Ok(mut child) => {
                child.wait()?;
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
        Ok(())
    }
}

fn show_prompt() -> io::Result<()> {
    let mut output = io::stdout();
    if output.is_terminal() {
        output.write(PROMPT)?;
        output.flush()?;
    }
    Ok(())
}

fn read_stdin() -> io::Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buf)?;
    Ok(buf)
}

fn main() -> io::Result<()> {
    loop {
        show_prompt()?;
        let line = read_stdin()?;
        if line.trim() == EXIT {
            break;
        }
        let cmd = Cmd::from_string(&line);
        cmd.run()?;
    }
    Ok(())
}
