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
    if args.len() == 0 {
      return MetaCommand::Unknown;
    }
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
  repl_helper.append_history(".history").unwrap();
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

#[cfg(test)]
mod test {
  use super::*;
  use std::result::Result;
  use rstest::rstest;
  use pretty_assertions::assert_eq;
  use crate::read_eval_print_loop::{get_config, RealEvalPrintLoopHelper};

  #[rstest]
  #[case(MetaCommand::Help)]
  fn test_help_meta_command(#[case] input: MetaCommand) {
    let mut repl = init_repl().unwrap();
    let result = handle_meta_command(input, &mut repl);

    assert_eq!(result.is_ok(), true);
  }

  #[rstest]
  #[case(".open test.db")]
  fn test_open_meta_command(#[case] command: &str) {
    let mut repl = init_repl().unwrap();
    let input = MetaCommand::Open(command.to_string());
    let result = handle_meta_command(input, &mut repl);

    assert_eq!(result.is_ok(), true);
  }

  #[rstest]
  #[case(MetaCommand::Unknown)]
  fn test_unknown_meta_command(#[case] input: MetaCommand) {
    let mut repl = init_repl().unwrap();
    let result = handle_meta_command(input, &mut repl);

    assert_eq!(result.is_err(), true);
  }

  #[rstest]
  #[case(MetaCommand::Exit, ".exit")]
  #[case(MetaCommand::Quit, ".quit")]
  #[case(MetaCommand::Help, ".help")]
  #[case(MetaCommand::Tables, ".tables")]
  #[case(MetaCommand::Unknown, "Unknown command")]
  fn test_display_meta_command_1(
    #[case] input: MetaCommand,
    #[case] expected: &str,
  ) {
    assert_eq!(format!("{}", input), expected.to_string());
  }

  #[rstest]
  #[case(MetaCommand::Open(".open test.db".to_string()), ".open")]
  #[case(MetaCommand::Read(".read test.db".to_string()), ".read")]
  #[case(MetaCommand::Save(".save test.db".to_string()), ".save")]
  #[case(MetaCommand::Ast(".ast SELECT * from test;".to_string()), ".ast")]
  fn test_display_meta_command_2(
    #[case] input: MetaCommand,
    #[case] expected: &str,
  ) {
    assert_eq!(format!("{}", input), expected.to_string());
  }

  fn init_repl() -> Result<Editor<RealEvalPrintLoopHelper>, ()> {
    let repl_helper = RealEvalPrintLoopHelper::default();
    let repl_config = get_config();
    let mut repl = Editor::with_config(repl_config);
    repl.set_helper(Some(repl_helper));

    Ok(repl)
  }
}
