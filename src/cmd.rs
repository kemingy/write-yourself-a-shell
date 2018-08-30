use std::convert::TryFrom;

use crate::error::{self, Error};

// A command consists of a binary and its arguments
#[derive(Debug, PartialEq)]
pub struct Cmd<'a> {
    pub binary: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for Cmd<'a> {
    type Error = error::Error;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let mut parts = line.split_whitespace();
        let binary = parts.nth(0).ok_or_else(|| Error::NoBinary)?;
        let args = parts.collect();

        Ok(Cmd { binary, args })
    }
}

type Cmds<'a> = Vec<Cmd<'a>>;

pub fn convert<'a>(line: &'a str) -> Result<Cmds<'a>, error::Error> {
    let commands = line.split(';');
    commands.map(|cmd| Cmd::try_from(cmd)).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_line() {
        assert_eq!(Cmd::try_from("").unwrap_err(), Error::NoBinary,);
    }

    #[test]
    fn test_single_binary() {
        assert_eq!(
            Cmd::try_from("echo").unwrap(),
            Cmd {
                binary: "echo",
                args: vec![]
            }
        );
    }

    #[test]
    fn test_binary_with_arguments() {
        assert_eq!(
            Cmd::try_from("echo 1 2 3").unwrap(),
            Cmd {
                binary: "echo",
                args: vec!["1", "2", "3"]
            }
        );
    }

    #[test]
    fn test_multiple_commands() {
        assert_eq!(
            convert("cat test.txt; echo hello").unwrap(),
            vec![
                Cmd {
                    binary: "cat",
                    args: vec!["test.txt"]
                },
                Cmd {
                    binary: "echo",
                    args: vec!["hello"]
                }
            ]
        );
    }
}
