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

  fn hint(
    &self,
    line: &str,
    position: usize,
    context: &Context<'_>
  ) -> Option<String> {
    self.hinter.hint(line, position, context)
  }
}

// 这里必须要实现 Highlighter 这个 trait，不然上面的 derive Helper 会报错
impl Highlighter for RealEvalPrintLoopHelper {
  // TODO:
}

// 这里必须要实现 Validator 这个 trait，不然上面的 derive Helper 会报错
impl Validator for RealEvalPrintLoopHelper {
  // TODO:
}
