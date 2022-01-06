use std::result;
use thiserror::Error;
use sqlparser::parser::ParserError;

// 自定义 error
#[derive(Error, Debug, PartialEq)]
pub enum NollaDBError {
  #[error("General error:{0}")]
  General(String),
  #[error("Internal error:{0}")]
  Internal(String),
  #[error("Unknown command error:{0}")]
  UnknownCommand(String),
  #[error("Parse error:{0:?}")]
  Parse(#[from] ParserError),
  #[error("To be Implemented error:{0:?}")]
  ToBeImplemented(String),
}

pub type Result<T> = result::Result<T, NollaDBError>;

pub fn error_with_general(text: &str) -> NollaDBError {
  NollaDBError::General(text.to_owned())
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::rstest;
  use pretty_assertions::assert_eq;

  #[rstest]
  #[case("test error")]
  fn general_error_test(#[case] input: &str) {
    let expected = NollaDBError::General(input.to_string());
    let result = error_with_general(&input);

    assert_eq!(result, expected);
  }
}
