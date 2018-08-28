use crate::error::Error;

// A command consists of a binary and its arguments
#[derive(Debug, PartialEq)]
pub struct Cmd<'a> {
    pub binary: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> Cmd<'a> {
    // Extract the command and its arguments from the commandline
    pub fn extract_from(line: &'a str) -> Result<Self, Error> {
        let mut parts = line.split_whitespace();
        let binary = parts.nth(0).ok_or_else(|| Error::NoBinary)?;
        let args = parts.collect();

        Ok(Cmd { binary, args })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_line() {
        assert_eq!(
            Cmd::extract_from("").unwrap_err(),
            Error::NoBinary,
        );
    }

    #[test]
    fn test_single_binary() {
        assert_eq!(
            Cmd::extract_from("echo").unwrap(),
            Cmd {
                binary: "echo",
                args: vec![]
            }
        );
    }

    #[test]
    fn test_binary_with_arguments() {
        assert_eq!(
            Cmd::extract_from("echo 1 2 3").unwrap(),
            Cmd {
                binary: "echo",
                args: vec!["1", "2", "3"]
            }
        );
    }
}
