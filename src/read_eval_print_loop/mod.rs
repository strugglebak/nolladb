use std::borrow::Cow::{self, Borrowed, Owned};

use rustyline_derive::{Completer, Helper};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::validate::{Validator, ValidationContext, ValidationResult};
use rustyline::{Context};
use rustyline::error::ReadlineError;

#[derive(Completer, Helper)]
pub struct RealEvalPrintLoopHelper {
  pub colored_prompt: String,
  pub hinter: HistoryHinter,
  pub highlighter: MatchingBracketHighlighter,
}

impl Default for RealEvalPrintLoopHelper {
  fn default() -> Self {
    Self {
      colored_prompt: "".to_owned(),
      hinter: HistoryHinter {},
      highlighter: MatchingBracketHighlighter::new(),
    }
  }
}

// 这里必须要实现 Hinter 这个 trait，不然上面的 derive Helper 会报错
impl Hinter for RealEvalPrintLoopHelper {
  type Hint = String;

  // 参数: 当前编辑行 line 以及光标位置 position
  // 返回: 一个需要展示的字符串
  //      如果当前用户输入没有 hint 可用就返回 None
  fn hint(
    &self,
    line: &str,
    position: usize,
    // lifetime 语法省略，不然会报错
    context: &Context<'_>
  ) -> Option<String> {
    self.hinter.hint(line, position, context)
  }
}

// 这里必须要实现 Highlighter 这个 trait，不然上面的 derive Helper 会报错
impl Highlighter for RealEvalPrintLoopHelper {
  // Lifetime bounds
  // 即 'b 在 's 的 lifetime 范围内
  // 而 'b 也在 'p 的 lifetime 范围内
  // 参数: prompt
  // 返回: highlight 版本
  // 作用: 高亮 prompt
  fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
    &'s self,
    prompt: &'p str,
    default: bool
  // Cow Clone-On-Write 智能指针
  // 主要是对 large data 进行优化时使用，即 copy large data
  // 这个指针的作用是尽可能迟地 copy large data
  // 如果给多个变量赋值同一个 large data，对于 Cow 来讲只需要通过其中一个变量
  // 来对这个 large data 进行写操作
  ) -> Cow<'b, str> {
    match default {
      true => Borrowed(&self.colored_prompt),
      false => Borrowed(prompt)
    }
  }

  // 参数: hint
  // 返回: highlight 版本
  // 作用: 高亮 hint
  fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
    Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
  }

  // 参数: line, 光标 position
  // 返回: highlight 版本
  fn highlight<'l>(&self, line: &'l str, position: usize) -> Cow<'l, str>{
    self.highlighter.highlight(line, position)
  }

  // 参数: line, 光标 position
  // 返回: line 是否需要被高亮
  // 作用: 优化 字符输入 和 光标移动 时的刷新
  fn highlight_char(&self, line: &str, position: usize) -> bool {
    self.highlighter.highlight_char(line, position)
  }
}

// 这里必须要实现 Validator 这个 trait，不然上面的 derive Helper 会报错
// rustyline 用 Validator 这个 trait 提供的 validate 函数的作用在于
// 当按下 enter 键时，是否要结束当前正在编辑的这个行为，然后把当前 line 的 buffer
// 给到 Editor::readline 或者其他变量调用
impl Validator for RealEvalPrintLoopHelper {
  fn validate(&self, context: &mut ValidationContext) ->
    Result<ValidationResult, ReadlineError>
  {
    use ValidationResult::{Incomplete, Valid};
    let input = context.input();
    let result = if input.starts_with(".") {
      Valid(None)
    } else if !input.starts_with(";") {
      Incomplete
    } else {
      Valid(None)
    };

    Ok(result)
  }
}
