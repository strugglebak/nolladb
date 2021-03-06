
#[macro_use] extern crate prettytable;
// #[macro_use] extern crate log;

mod intro_message;
mod error;
mod meta_command;
mod sql_query;
mod read_eval_print_loop;
mod table;
mod database;

use std::{env, process};

use rustyline::Editor;
use rustyline::error::ReadlineError;
use env_logger::Env;

use intro_message::intro_message;
use meta_command::{MetaCommand, handle_meta_command};
use sql_query::handle_sql_query;
use read_eval_print_loop::{
  RealEvalPrintLoopHelper,
  CommandType,
  get_config,
  get_command_type,
};
use database::Database;
use database::database_manager::DatabaseManager;

fn main() -> rustyline::Result<()> {

  let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "auto");
  env_logger::init_from_env(env);

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

  // 初始化 database 相关
  let mut database:Database;
  let mut database_manager:DatabaseManager;
  let database_manager_file = String::from(".dmf");

  match Database::start(database_name.clone(), database_manager_file.clone()) {
    Ok((new_database, new_database_manager)) => {
      database = new_database;
      database_manager = new_database_manager;
    },
    Err(error) => {
      eprintln!("An error occurred: {:?}", error);
      process::exit(1)
    }
  }

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

  intro_message();

  loop {
    let print = format!("nolladb>");
    repl
     .helper_mut()
     .expect("No helper found")
     .colored_prompt = format!("\x1b[1;34m{}\x1b[0m", print);

    let readline = repl.readline(&print);
    match readline {
      Ok(command) => {
        if command.split_whitespace().collect::<Vec<&str>>().len() == 0 { continue; }

        repl.add_history_entry(command.as_str());
        let command_type = get_command_type(&command.trim().to_owned());
        match command_type {
          CommandType::MetaCommand(cmd) => {
            match handle_meta_command(
              cmd,
              &mut repl,
              &mut database,
              &mut database_manager
            ) {
              Ok(response) => {
                match response {
                  MetaCommand::Open(new_database_name) => {
                    match Database::open_mut(&mut database_manager, new_database_name) {
                      Ok(new_database) => {
                        println!("Opening {}...", new_database.database_name);
                        // 这里 clone 去掉引用，拿到的就是引用指向的数据内容
                        database = new_database.clone();
                        println!("Opening {} done", database.database_name);
                      },
                      Err(error) => eprintln!("An error occurred: {:?}", error),
                    }
                  },
                  MetaCommand::Read(database_name) => {
                    match Database::start(database_name.clone(), database_manager_file.clone()) {
                      Ok((new_database, new_database_manager)) => {
                        database = new_database;
                        database_manager = new_database_manager;
                      },
                      Err(error) => {
                        eprintln!("An error occurred: {:?}", error);
                      }
                    }
                  },
                  MetaCommand::Save(database_name) => {
                    match Database::end(
                      database_name.clone(),
                      &database,
                      database_manager_file.clone(),
                      &database_manager
                    ) {
                      Ok(()) => (),
                      Err(error) => eprintln!("An error occurred: {:?}", error),
                    }
                  },
                  _ => (),
                }
              },
              Err(error) => eprintln!("An error occurred: {:?}", error),
            }
          },
          CommandType::SQLQuery(_) => {
            match handle_sql_query(&command, &mut database) {
              Ok(response) => {
                println!("{}", response);
                // SQL statement 执行成功后要更新 database 以及 database_manager
                database_manager.database.insert(
                  database.database_name.clone(),
                  database.clone()
                );
              },
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
