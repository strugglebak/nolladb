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
    // as_ref 将 String 转变成 &str
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
