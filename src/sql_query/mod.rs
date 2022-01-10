mod query;

use sqlparser::parser::{Parser, ParserError};
use sqlparser::dialect::SQLiteDialect;
use sqlparser::ast::Statement;

use crate::error::{Result, NollaDBError};

use query::create::CreateQuery;

#[derive(Debug, PartialEq)]
pub enum SQLQuery {
  CreateTable(String),
  Select(String),
  Insert(String),
  Update(String),
  Delete(String),
  Unknown(String),
}

impl SQLQuery {
  pub fn new(command: String) -> SQLQuery {
    let args: Vec<&str> = command.split_whitespace().collect();
    let first_cmd = args[0].to_owned();
    match first_cmd.as_ref() {
      "create" => SQLQuery::CreateTable(command),
      "select" => SQLQuery::Select(command),
      "insert" => SQLQuery::Insert(command),
      "update" => SQLQuery::Update(command),
      "delete" => SQLQuery::Delete(command),
      _ => SQLQuery::Unknown(command),
    }
  }
}

pub fn handle_sql_query(sql_query: &str) -> Result<String> {
  let dialect = SQLiteDialect {};
  let mut ast =
    Parser::parse_sql(&dialect, &sql_query)
      .map_err(NollaDBError::from)?;

  // 目前仅支持单个 SQL 语句输入
  if ast.len() > 1 {
    return Err(
      NollaDBError::SQLParseError(
        ParserError::ParserError(
          format!(
            "Expected a single SQL query statement
            , but here are '{}' SQL query statements,
            we now only support one single SQL query in typing",
            ast.len()
          )
        )
      )
    );
  }

  let message: String;
  let statement = ast.pop().unwrap();
  match statement {
    Statement::CreateTable {
      ..
    } => {
      match CreateQuery::new(&statement) {
        Ok(create_query) => {
          let table_name = create_query.table_name.clone();
          // TODO: 创建表
          message = String::from("CREATE TABLE statement done.");
          println!("{}", message.to_string());
        },
        Err(error) => return Err(error),
      }
    },
    Statement::Query(_) => {
      // TODO: 在表中查询
      message = String::from("SELECT statement done.");
      println!("{}", message.to_string());
    },
    Statement::Insert {
      ..
    } => {
      // TODO: 在表中插入
      message = String::from("INSERT statement done.");
      println!("{}", message.to_string());
    },
    Statement::Update {
      ..
    } => {
      // TODO: 在表中更新
      message = String::from("UPDATE statement done.");
      println!("{}", message.to_string());
    },
    Statement::Delete {
      ..
    } => {
      // TODO: 在表中删除
      message = String::from("UPDATE statement done.");
      println!("{}", message.to_string());
    },
    _ => {
      return Err(
        NollaDBError::ToBeImplemented(
          "Other SQL statement will to be implemented soon.".to_string()
        )
      )
    },
  };

  Ok(message)
}
