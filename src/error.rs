use std::result;
use thiserror::Error;
use sqlparser::parser::ParserError;

// 自定义 error
#[derive(Error, Debug, PartialEq)]
pub enum NollaDBError {
  #[error("General error: {0}")]
  General(String),
  #[error("Internal error: {0}")]
  Internal(String),
  #[error("Unknown command error: {0}")]
  UnknownCommand(String),
  #[error("SQL Parse error: {0:?}")]
  SQLParseError(#[from] ParserError),
  #[error("To be Implemented error: {0}")]
  ToBeImplemented(String),
}

pub type Result<T> = result::Result<T, NollaDBError>;

#[allow(dead_code)]
pub fn nolladb_error(text: &str) -> NollaDBError {
  NollaDBError::General(text.to_owned())
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::rstest;
  use pretty_assertions::assert_eq;

  #[rstest]
  #[case("test error")]
  fn test_nolladb_error(#[case] input: &str) {
    let expected = NollaDBError::General(input.to_string());
    let result = nolladb_error(&input);

    assert_eq!(result, expected);
  }

  #[rstest]
  #[case("General error")]
  fn test_nolladb_general_error(#[case] input: &str) {
      let expected = format!("General error: {}", input);
      let result = format!("{}", NollaDBError::General(input.to_string()));

      assert_eq!(result, expected);
  }

  #[rstest]
  #[case("Internal error")]
  fn test_nolladb_internal_error(#[case] input: &str) {
      let expected = format!("Internal error: {}", input);
      let result = format!("{}", NollaDBError::Internal(input.to_string()));

      assert_eq!(result, expected);
  }

  #[rstest]
  #[case("Unknown command error")]
  fn test_nolladb_unknown_command_error(#[case] input: &str) {
      let expected = format!("Unknown command error: {}", input);
      let result = format!("{}", NollaDBError::UnknownCommand(input.to_string()));

      assert_eq!(result, expected);
  }

  #[rstest]
  #[case("SQL parse error")]
  fn test_nolladb_sql_parse_error(#[case] input: &str) {
      let expected = format!("SQL Parse error: ParserError(\"{}\")", input);
      let result = format!("{}", NollaDBError::SQLParseError(ParserError::ParserError(input.to_string())));

      assert_eq!(result, expected);
  }

  #[rstest]
  #[case("To be implemented")]
  fn test_nolladb_to_be_implemented_error(#[case] input: &str) {
      let expected = format!("To be Implemented error: {}", input);
      let result = format!("{}", NollaDBError::ToBeImplemented(input.to_string()));

      assert_eq!(result, expected);
  }
}
