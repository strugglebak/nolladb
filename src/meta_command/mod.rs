use std::fmt;
use rustyline::Editor;
use crate::error::{Result, NollaDBError};
use crate::read_eval_print_loop::{RealEvalPrintLoopHelper};

#[derive(Debug, PartialEq)]
pub enum MetaCommand {
  Exit,
  Quit,
  Help,
  Tables,
  Open(String),
  Read(String),
  Save(String),
  Ast(String),
  Unknown,
}

impl MetaCommand {
  pub fn new(command: String) -> MetaCommand {
    let args: Vec<&str> = command.split_whitespace().collect();
    // to_owned 将 &str 转变成 String
    let first_cmd = args[0].to_owned();
    // as_ref 将 String 转变成 &str
    match first_cmd.as_ref() {
      ".exit" => MetaCommand::Exit,
      ".quit" => MetaCommand::Quit,
      ".help" => MetaCommand::Help,
      ".tables" => MetaCommand::Tables,
      ".open" => MetaCommand::Open(command),
      ".read" => MetaCommand::Read(command),
      ".save" => MetaCommand::Save(command),
      ".ast" => MetaCommand::Ast(command),
      _ => MetaCommand::Unknown,
    }
  }
}

// format meta command
impl fmt::Display for MetaCommand {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MetaCommand::Exit => f.write_str(".exit"),
      MetaCommand::Quit => f.write_str(".quit"),
      MetaCommand::Help => f.write_str(".help"),
      MetaCommand::Tables => f.write_str(".tables"),
      MetaCommand::Open(_) => f.write_str(".open"),
      MetaCommand::Read(_) => f.write_str(".read"),
      MetaCommand::Save(_) => f.write_str(".save"),
      MetaCommand::Ast(_) => f.write_str(".ast"),
      MetaCommand::Unknown => f.write_str("Unknown command"),
    }
  }
}

fn handle_exit_or_quit_meta_command(
  repl_helper: &mut Editor<RealEvalPrintLoopHelper>
) -> Result<String> {
  repl_helper.append_history("history").unwrap();
  std::process::exit(0)
}

pub fn handle_meta_command(
  command: MetaCommand,
  repl_helper: &mut Editor<RealEvalPrintLoopHelper>
) -> Result<String> {
  match command {
    MetaCommand::Exit => {
      handle_exit_or_quit_meta_command(repl_helper)
    },
    MetaCommand::Quit => {
      handle_exit_or_quit_meta_command(repl_helper)
    },
    MetaCommand::Help => Ok(format!(
      "{}{}{}{}{}{}{}{}{}",
      "Special commands:\n",
      ".help            - Display help message\n",
      "---------------------------------------\n",
      ".ast  <QUERY>    - Show the abstract syntax tree for QUERY.\n",
      ".exit            - Quits this application\n",
      ".open <FILENAME> - Close existing database and reopen FILENAME\n",
      ".read <FILENAME> - Read input from FILENAME\n",
      ".save <FILENAME> - Write in-memory database into FILENAME\n",
      ".tables          - List names of tables\n",
    )),
    MetaCommand::Tables => Ok(format!("To be implemented: {}", ".tables".to_string())),
    MetaCommand::Open(args) => Ok(format!("To be implemented: {}", args)),
    MetaCommand::Read(args) => Ok(format!("To be implemented: {}", args)),
    MetaCommand::Save(args) => Ok(format!("To be implemented: {}", args)),
    MetaCommand::Ast(args) => Ok(format!("To be implemented: {}", args)),
    MetaCommand::Unknown => Err(NollaDBError::UnknownCommand(format!(
      "Unknown command or invalid arguments. Enter '.help'"
    ))),
  }
}
