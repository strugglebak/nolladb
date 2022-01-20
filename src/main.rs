#[macro_use] extern crate prettytable;

mod error;
mod meta_command;
mod sql_query;
mod read_eval_print_loop;
mod table;
mod database;

use std::{env, process};

use rustyline::Editor;
use rustyline::error::ReadlineError;

use meta_command::handle_meta_command;
use sql_query::handle_sql_query;
use read_eval_print_loop::{
  RealEvalPrintLoopHelper,
  CommandType,
  get_config,
  get_command_type,
};
use database::Database;

fn main() -> rustyline::Result<()> {
  // 创建 database
  let args: Vec<String> = env::args().collect();
  // cargo run
  let args_right_number = 2;
  if args.len() != args_right_number {
    println!("Usage: {} DATABASE_NAME.db", "cargo run");
    process::exit(1)
  }
  let database_name = &args[args_right_number - 1];
  if !database_name.ends_with(".db") {
    println!("Database name should end with '.db'");
    process::exit(1)
  }
  let mut database = Database::new(database_name.to_string());

  // 创建 repl helper
  let repl_helper = RealEvalPrintLoopHelper::default();

  // 获取 repl 配置
  let repl_config = get_config();

  // 用 repl 配置和 repl helper 初始化 repl
  let mut repl = Editor::with_config(repl_config);
  repl.set_helper(Some(repl_helper));

  let history_file = ".history";

  // 加载历史记录
  if repl.load_history(history_file).is_err() {
    println!("No more history");
  }

  loop {
    let print = format!("nolladb>");
    repl
     .helper_mut()
     .expect("No helper found")
     .colored_prompt = format!("\x1b[1;34m{}\x1b[0m", print);

    let readline = repl.readline(&print);
    match readline {
      Ok(command) => {
        if command.len() == 0 { continue; }

        repl.add_history_entry(command.as_str());
        let command_type = get_command_type(&command.trim().to_owned());
        match command_type {
          CommandType::MetaCommand(cmd) => {
            match handle_meta_command(cmd, &mut repl) {
              Ok(response) => println!("{}", response),
              Err(error) => eprintln!("An error occurred: {:?}", error),
            }
          },
          CommandType::SQLQuery(_) => {
            match handle_sql_query(&command, &mut database) {
              Ok(response) => println!("{}", response),
              Err(error) => eprintln!("An error occurred: {:?}", error),
            }
          }
        }
      },
      Err(ReadlineError::Interrupted) => break,
      Err(ReadlineError::Eof) => break,
      Err(error) => {
        eprintln!("An error occurred: {:?}", error);
        break;
      }
    }
  }

  repl.append_history(history_file).unwrap();

  Ok(())
}
