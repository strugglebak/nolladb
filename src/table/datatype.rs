use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum Datatype {
  Integer,
  Bool,
  Text,
  Real,
  None,
  Invalid,
}

impl Datatype {
  pub fn new(command: String) -> Datatype {
    match command.to_lowercase().as_ref() {
      "integer" => Datatype::Integer,
      "bool" => Datatype::Bool,
      "text" => Datatype::Text,
      "real" => Datatype::Real,
      "none" => Datatype::None,
      _ => {
        eprintln!("Invalid datatype: {}", command)
        return Datatype::Invalid;
      },
    }
  }
}

impl fmt::Display for Datatype {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Datatype::Integer => f.write_str("Integer"),
      Datatype::Bool => f.write_str("Boolean"),
      Datatype::Text => f.write_str("Text"),
      Datatype::Real => f.write_str("Real"),
      Datatype::None => f.write_str("None"),
      Datatype::Invalid => f.write_str("Invalid"),
    }
  }
}
