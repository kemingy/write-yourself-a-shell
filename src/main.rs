#![feature(uniform_paths)]
#![feature(try_from)]

use std::convert::TryFrom;
use std::io::{self, Write};
use std::process::{Command, Stdio};

mod builtin;
mod cmd;
mod error;

use cmd::{Statement, Statements};
use error::Error;

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut line = String::new();
        print!("> ");
        stdout.flush()?;
        stdin.read_line(&mut line)?;

        match Statements::try_from(line.as_ref()) {
            Ok(statements) => {
                for statement in statements {
                    match statement {
                        Statement::Cmd { binary, args } => {
                            if binary == "exit" {
                                builtin::exit(args);
                            } else {
                                match Command::new(binary).args(args).output() {
                                    Ok(output) => {
                                        print!("{}", String::from_utf8_lossy(&output.stdout))
                                    }
                                    Err(e) => eprintln!("rush: {:?}", e),
                                }
                            }
                        }

                        Statement::Pipe { input, output } => {
                            let input = *input;
                            let output = *output;

                            match (input, output) {
                                (
                                    Statement::Cmd {
                                        binary: binary_in,
                                        args: args_in,
                                    },
                                    Statement::Cmd {
                                        binary: binary_out,
                                        args: args_out,
                                    },
                                ) => {
                                    let first_child = Command::new(binary_in)
                                        .args(args_in)
                                        .stdout(Stdio::piped())
                                        .spawn()?;
                                    let first_out =
                                        first_child.stdout.ok_or_else(|| Error::Spawn)?;
                                    let second_child = Command::new(binary_out)
                                        .args(args_out)
                                        .stdin(Stdio::from(first_out))
                                        .stdout(Stdio::piped())
                                        .spawn()?;
                                    let output = second_child.wait_with_output()?;
                                    print!("{}", String::from_utf8_lossy(&output.stdout));
                                }

                                _ => unimplemented!(),
                            }
                        }
                    }
                }
            }

            Err(Error::NoBinary) => {}

            Err(e) => eprintln!("rush: {:?}", e),
        }
    }
}
