#![feature(uniform_paths)]
#![feature(try_from)]

use std::convert::TryFrom;
use std::io::{self, Write};
use std::process::Command;

mod builtin;
mod cmd;
mod error;

use cmd::Cmds;
use error::Error;

fn main() -> Result<(), Error> {
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
                    if cmd.binary == "exit" {
                        builtin::exit(cmd.args);
                    } else {
                        match Command::new(cmd.binary).args(cmd.args).output() {
                            Ok(output) => print!("{}", String::from_utf8_lossy(&output.stdout)),
                            Err(e) => eprintln!("rush: {:?}", e),
                        }
                    }
                }
            }
            Err(Error::NoBinary) => {}
            Err(e) => eprintln!("rush: {:?}", e),
        }
    }
}
