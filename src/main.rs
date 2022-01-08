mod error;
mod meta_command;
mod sql_command;
mod read_eval_print_loop;

use rustyline::Editor;
use rustyline::error::ReadlineError;

use meta_command::handle_meta_command;
use read_eval_print_loop::{
  get_config,
  RealEvalPrintLoopHelper,
};

fn main() {
  // 获取 repl 配置
  let repl_config = get_config();
  // 创建 repl helper
  let repl_helper = RealEvalPrintLoopHelper::default();

  // 用 repl 配置和 repl helper 初始化 repl
  let mut repl = Editor::with_config(repl_config);
  repl.set_helper(Some(repl_helper));

  // 加载历史记录
  if repl.load_history("history").is_err() {
    println!("No more history.");
  }

  loop {
    let print = format!("nolladb>");
    repl
     .helper_mut()
     .expect("No helper found.")
     .colored_prompt = format!("\x1b[1;34m{}\x1b[0m", print);

    let readline = repl.readline(&print);
    match readline {
      Ok(command) => {
        // TODO:
        println!("{}", command);
      },
      Err(ReadlineError::Interrupted) => break,
      Err(ReadlineError::Eof) => break,
      Err(error) => {
        eprintln!("An error occurred: {:?}", error);
        break;
      }
    }
  }
}
