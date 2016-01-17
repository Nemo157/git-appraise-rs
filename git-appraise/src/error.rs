use std;
use git2;
use serde_json;

#[derive(Debug)]
pub enum Error {
  External(Box<std::error::Error>),
  NotFound,
}

impl std::error::Error for Error {
  fn description(&self) -> &str {
    match self {
      &Error::External(ref err) => err.description(),
      &Error::NotFound => "Not found",
    }
  }

  fn cause(&self) -> Option<&std::error::Error> {
    match self {
      &Error::External(ref err) => Some(&**err),
      _ => None,
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      &Error::External(ref err) => err.fmt(f),
      &Error::NotFound => "Not Found".fmt(f),
    }
  }
}

impl std::convert::From<git2::Error> for Error {
  fn from(e: git2::Error) -> Error {
    Error::External(Box::new(e))
  }
}

impl std::convert::From<serde_json::error::Error> for Error {
  fn from(e: serde_json::error::Error) -> Error {
    Error::External(Box::new(e))
  }
}
