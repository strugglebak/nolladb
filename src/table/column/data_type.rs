use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum DataType {
  Integer,
  Text,
  Bool,
  Real,
  None,
  Invalid,
}

impl DataType {
  pub fn new(command: String) -> DataType {
    match command.to_lowercase().as_ref() {
      "integer" => DataType::Integer,
      "text" => DataType::Text,
      "bool" => DataType::Bool,
      "real" => DataType::Real,
      "none" => DataType::None,
      _ => {
        eprintln!("Invalid datatype: {}", command);
        return DataType::Invalid;
      },
    }
  }
}

impl fmt::Display for DataType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      DataType::Integer => f.write_str("Integer"),
      DataType::Text => f.write_str("Text"),
      DataType::Bool => f.write_str("Boolean"),
      DataType::Real => f.write_str("Real"),
      DataType::None => f.write_str("None"),
      DataType::Invalid => f.write_str("Invalid"),
    }
  }
}
