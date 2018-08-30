use std::convert::TryFrom;
use std::vec;

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

#[derive(Debug, PartialEq)]
pub struct Cmds<'a>(Vec<Cmd<'a>>);

// We cannot implement TryFrom for a type from the standard library (vec in
// sthis case). Neither `Vec<T>` nor the `TryFrom` trait is defined by us.
// See https://github.com/rust-lang/rust/issues/24745
// A common pattern is to use a newtype instead and implement `TryFrom` for that
// instead.
// See https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html
// See https://doc.rust-lang.org/rust-by-example/generics/new_types.html
// See https://github.com/rust-unofficial/patterns/blob/master/patterns/newtype.md
impl<'a> TryFrom<&'a str> for Cmds<'a> {
    type Error = error::Error;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let commands = line.split(';');
        // `std::iter::FromIterator` is not implemented for our newtype. We could either implement it or
        // specify what type we want to collect into and then wrap the result into an `Ok()`.
        let v: Result<Vec<_>, _> = commands.map(|cmd| Cmd::try_from(cmd)).collect();
        Ok(Cmds(v?))
    }
}

// Since we defined a newtype for our `Cmds` (see above),
// we also need to define all necessary traits for that.
// In order to iterate over `Cmds`, we implement `IntoIterator`,
// which is used when writing `for cmd in cmds`.
impl<'a> IntoIterator for Cmds<'a> {
    type Item = Cmd<'a>;
    type IntoIter = vec::IntoIter<Cmd<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_line() {
        match Cmd::try_from("") {
            Err(Error::NoBinary) => assert!(true),
            _ => assert!(false),
        }
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
            Cmds::try_from("cat test.txt; echo hello").unwrap(),
            Cmds(vec![
                Cmd {
                    binary: "cat",
                    args: vec!["test.txt"]
                },
                Cmd {
                    binary: "echo",
                    args: vec!["hello"]
                }
            ])
        );
    }
}
