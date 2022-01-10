use crate::error::{Result, NollaDBError};

#[derive(Debug, PartialEq)]
pub enum SQLCommand {
  CreateTable(String),
  Select(String),
  Insert(String),
  Update(String),
  Delete(String),
  Unknown(String),
}

impl SQLCommand {
  pub fn new(command: String) -> SQLCommand {
    let args: Vec<&str> = command.split_whitespace().collect();
    let first_cmd = args[0].to_owned();
    match first_cmd.as_ref() {
      "create" => SQLCommand::CreateTable(command),
      "select" => SQLCommand::Select(command),
      "insert" => SQLCommand::Insert(command),
      "update" => SQLCommand::Update(command),
      "delete" => SQLCommand::Delete(command),
      _ => SQLCommand::Unknown(command),
    }
  }
}

pub fn handle_sql_command(query: &str) -> Result<String> {
  let message: String = String::from("");

  Ok(message)
}
