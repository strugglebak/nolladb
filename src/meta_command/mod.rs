use std::fmt;
use rustyline::Editor;
use crate::error::{Result, NollaDBError};
use crate::read_eval_print_loop::{RealEvalPrintLoopHelper};

#[derive(Debug, PartialEq)]
pub enum MetaCommand {
  Exit,
  Help,
  Open(String),
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
      ".help" => MetaCommand::Help,
      ".open" => MetaCommand::Open(command),
      _ => MetaCommand::Unknown,
    }
  }
}

// format meta command
impl fmt::Display for MetaCommand {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MetaCommand::Exit => f.write_str(".exit"),
      MetaCommand::Help => f.write_str(".help"),
      MetaCommand::Open(_) => f.write_str(".open"),
      MetaCommand::Unknown => f.write_str("Unknown command"),
    }
  }
}

pub fn handle_meta_command(
  command: MetaCommand,
  repl_helper: &mut Editor<RealEvalPrintLoopHelper>
) -> Result<String> {
  match command {
    MetaCommand::Exit => {
      repl_helper.append_history("history").unwrap();
      std::process::exit(0)
    },
    MetaCommand::Help => Ok(format!(
      "{}{}{}{}{}{}{}{}{}",
      "Special commands:\n",
      ".help            - Display help message\n",
      "-----------------------------------------",
      ".ast  <QUERY>    - Show the abstract syntax tree for QUERY.\n",
      ".exit            - Quits this application\n",
      ".open <FILENAME> - Close existing database and reopen FILENAME\n",
      ".read <FILENAME> - Read input from FILENAME\n",
      ".save <FILENAME> - Write in-memory database into FILENAME\n",
      ".tables          - List names of tables\n",
    )),
    MetaCommand::Open(args) => Ok(format!("To be implemented: {}", args)),
    MetaCommand::Unknown => Err(NollaDBError::UnknownCommand(format!(
      "{}{}",
      "Unknown command or invalid arguments.\n",
      "Enter '.help'\n"
    ))),
  }
}
