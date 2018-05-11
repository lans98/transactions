use std::error::Error;

use std::fmt;
use std::fmt::{
  Display,
  Formatter
};

#[derive(Debug)]
pub struct ConcurrencyError {
  message: String
}

impl ConcurrencyError {
  pub fn new(msg: &str) -> Self {
    ConcurrencyError { message: String::from(msg) }
  }
}

impl Error for ConcurrencyError {
  fn description(&self) -> &str {
    self.message.as_str()
  }
}

impl Display for ConcurrencyError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub type ResultC<T> = Result<T, ConcurrencyError>;
