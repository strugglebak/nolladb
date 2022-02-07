use std::fmt;
use rustyline::Editor;
use crate::error::{Result, NollaDBError};
use crate::read_eval_print_loop::{RealEvalPrintLoopHelper};
use crate::sql_query::get_sql_ast;
use crate::database::Database;
use crate::database::database_manager::DatabaseManager;

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
) -> Result<MetaCommand> {
  repl_helper.append_history(".history").unwrap();
  std::process::exit(0)
}

fn get_str_after_meta_command(
  ref args: String,
  error_message: &str,
) -> Result<String> {
  let mut args_vec = args.split_whitespace().collect::<Vec<&str>>();
  if args_vec.len() == 1 {
    return Err(NollaDBError::UnknownCommand(error_message.to_string()));
  }
  let result = args_vec.split_off(1).join(" ");
  Ok(result)
}

pub fn handle_meta_command(
  command: MetaCommand,
  repl_helper: &mut Editor<RealEvalPrintLoopHelper>,
  database: &mut Database,
  database_manager: &mut DatabaseManager,
) -> Result<MetaCommand> {
  match command {
    MetaCommand::Exit => handle_exit_or_quit_meta_command(repl_helper),
    MetaCommand::Quit => handle_exit_or_quit_meta_command(repl_helper),
    MetaCommand::Help => {
      println!(
        "{}{}{}{}{}{}{}{}{}",
        "Special commands:\n",
        ".help            - Display help message\n",
        "---------------------------------------\n",
        ".ast  <QUERY>    - Show the abstract syntax tree for QUERY\n",
        ".exit            - Quits this application\n",
        ".open <FILENAME> - Close existing database and reopen FILENAME\n",
        ".read <FILENAME> - Read input from FILENAME\n",
        ".save <FILENAME> - Write in-memory database into FILENAME\n",
        ".tables          - List names of tables\n",
      );
      Ok(command)
    },
    MetaCommand::Tables => {
      let table_names = database.get_all_tables(
        &database_manager,
        database.database_name.clone()
      ).unwrap();

      for table_name in table_names {
        println!("{}", table_name);
      }
      Ok(command)
    },
    MetaCommand::Open(args) => {
      match get_str_after_meta_command(
        args.to_string(),
        ".open <FILENAME>: FILENAME should not be empty",
      ) {
        Ok(args) => Ok(MetaCommand::Open(args)),
        Err(error) => return Err(error),
      }
    },
    MetaCommand::Read(args) => {
      match get_str_after_meta_command(
        args.to_string(),
        ".read <FILENAME>: FILENAME should not be empty",
      ) {
        Ok(args) => Ok(MetaCommand::Read(args)),
        Err(error) => return Err(error),
      }
    },
    MetaCommand::Save(args) => {
      match get_str_after_meta_command(
        args.to_string(),
        ".save <FILENAME>: FILENAME should not be empty",
      ) {
        Ok(args) => Ok(MetaCommand::Save(args)),
        Err(error) => return Err(error),
      }
    },
    MetaCommand::Ast(ref args) => {
      match get_str_after_meta_command(
        args.to_string(),
        ".ast <QUERY>: QUERY should not be empty",
      ) {
        Ok(query) => {
          match get_sql_ast(&query.to_string()) {
            Ok(print_ast) => println!("{:#?}", print_ast),
            Err(error) => return Err(error),
          }
        }
        Err(error) => return Err(error),
      }
      Ok(command)
    },
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
  use crate::error::{Result as CustomResult};

  #[rstest]
  #[case(MetaCommand::Help)]
  fn test_help_meta_command(#[case] input: MetaCommand) {
    let result = gen_result(input);
    assert_eq!(result.is_ok(), true);
  }

  #[rstest]
  #[case(".open test.db")]
  fn test_open_meta_command(#[case] command: &str) {
    let input = MetaCommand::Open(command.to_string());
    let result = gen_result(input);
    assert_eq!(result.is_ok(), true);
  }

  #[rstest]
  #[case(MetaCommand::Unknown)]
  fn test_unknown_meta_command(#[case] input: MetaCommand) {
    let result = gen_result(input);
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

  fn gen_result(input: MetaCommand) -> CustomResult<MetaCommand> {
    let mut repl = init_repl().unwrap();
    let mut database = Database::new("test".to_string());
    let mut database_manager = DatabaseManager::new();
    match handle_meta_command(
      input,
      &mut repl,
      &mut database,
      &mut database_manager
    ) {
      Ok(result) => Ok(result),
      Err(error) => return Err(error),
    }
  }
}
