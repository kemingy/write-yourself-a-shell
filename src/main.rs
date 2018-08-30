#![feature(uniform_paths)]
#![feature(try_from)]

use std::convert::TryFrom;
use std::io::{self, Write};
use std::process::Command;

mod cmd;
mod error;

use cmd::Cmds;
use error::Error;

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut line = String::new();
        print!("> ");
        stdout.flush()?;
        stdin.read_line(&mut line)?;

        match Cmds::try_from(line.as_ref()) {
            Ok(cmds) => {
                for cmd in cmds {
                    let output = Command::new(cmd.binary).args(cmd.args).output()?;
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
            }
            Err(Error::NoBinary) => {}
        }
    }
}
