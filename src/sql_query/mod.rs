pub mod query;

use sqlparser::parser::{Parser, ParserError};
use sqlparser::dialect::SQLiteDialect;
use sqlparser::ast::Statement;

use crate::error::{Result, NollaDBError};
use crate::database::Database;
use crate::table::{Table};

use query::create::{CreateQuery};
use query::insert::{InsertQuery};

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
    if args.len() == 0 {
      return SQLQuery::Unknown(command);
    }
    let first_cmd = args[0].to_owned();
    match first_cmd.to_lowercase().as_ref() {
      "create" => SQLQuery::CreateTable(command),
      "select" => SQLQuery::Select(command),
      "insert" => SQLQuery::Insert(command),
      "update" => SQLQuery::Update(command),
      "delete" => SQLQuery::Delete(command),
      _ => SQLQuery::Unknown(command),
    }
  }
}

pub fn get_sql_ast(sql_query: &str) -> Result<Statement> {
  let dialect = SQLiteDialect {};
  let mut ast =
    Parser::parse_sql(&dialect, &sql_query)
      .map_err(NollaDBError::from)?;

  if ast.len() == 0 {
    return Err(
      NollaDBError::SQLParseError(
        ParserError::ParserError(
          format!("Expected a correct SQL query statement")
        )
      )
    );
  } else if ast.len() > 1 {
    // 目前仅支持单个 SQL 语句输入
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

  Ok(ast.pop().unwrap())
}

pub fn handle_sql_query(sql_query: &str, database: &mut Database) -> Result<String> {
  let message: String;
  match get_sql_ast(sql_query) {
    Ok(statement) => {
      match statement {
        Statement::CreateTable {
          ..
        } => {
          match CreateQuery::new(&statement) {
            Ok(create_query) => {
              let table_name = create_query.table_name.clone();

              // 检查表是否已经被创建
              if database.has_table(table_name.to_string()) {
                return Err(NollaDBError::Internal(
                  format!(
                    "Can not create table, because table '{}' already exists",
                    table_name
                  )
                ));
              }

              // 创建表
              let table = Table::new(create_query);
              // 打印表 schema
              let _ = table.print_column_of_schema();
              // 把表插入到数据库中
              database.tables.insert(table_name.to_string(), table);

              message = String::from("CREATE TABLE statement done");
            },
            Err(error) => return Err(error),
          }
        },
        Statement::Query(_) => {
          message = String::from("SELECT statement done");
        },
        Statement::Insert {
          ..
        } => {
          match InsertQuery::new(&statement) {
            Ok(insert_query) => {
              let InsertQuery {
                table_name,
                table_column_names,
                table_column_values,
              } = insert_query;

              // 检查表是否已经被创建
              if !database.has_table(table_name.to_string()) {
                return Err(NollaDBError::Internal(
                  format!(
                    "Table '{}' does not exist",
                    table_name
                  )
                ));
              }

              // 在对应表中执行插入操作
              let table = database.get_table_mut(table_name.to_string()).unwrap();
              // 检查要插入的 column name 是否在表中存在
              if !table_column_names
                .iter()
                .all(|column_name| table.has_column(column_name.to_string())) {
                return Err(NollaDBError::Internal(format!(
                  "Can not insert, because some of the columns do not exist"
                )));
              }

              // TODO: 这里有一种情况是 SQL 里面没有指定列名，那么就按照顺序写入

              for table_column_value in table_column_values {
                // 1. 检查要插入的 column value 的个数是否和 column name 一致
                let v_len = table_column_value.len();
                let n_len = table_column_names.len();
                if v_len != n_len {
                  return Err(NollaDBError::Internal(
                    format!(
                      "{} values for {} columns",
                      v_len,
                      n_len
                    )
                  ));
                }

                // 2. 检查唯一约束
                if let Err(error) =
                  table.check_unique_constraint(&table_column_names, &table_column_value) {
                  return Err(NollaDBError::Internal(
                    format!(
                      "Unique key constraint violation: {}",
                      error
                    )
                  ));
                }

                // 3. 以上 2 点检查完毕，说明没有唯一约束，可以插入
                table.insert_row(&table_column_names, &table_column_value);
              }

              // 打印插入完成后的表数据
              let _ = table.print_table_data();

              message = String::from("INSERT statement done");
            },
            Err(error) => return Err(error),
          }
        },
        Statement::Update {
          ..
        } => {
          // TODO: 在表中更新
          message = String::from("UPDATE statement done");
        },
        Statement::Delete {
          ..
        } => {
          // TODO: 在表中删除
          message = String::from("DELETE statement done");
        },
        _ => {
          return Err(
            NollaDBError::ToBeImplemented(
              "Other SQL statement will to be implemented soon".to_string()
            )
          );
        },
      };

    },
    Err(error) => return Err(error),
  }

  Ok(message)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::result::Result;
  use rstest::rstest;
  use pretty_assertions::{assert_eq};

  #[rstest]
  #[case("SELECT * FROM test;", "SELECT statement done")]
  #[case("DELETE FROM test WHERE id=1;", "DELETE statement done")]
  #[case("UPDATE test SET name='xxx' WHERE id=1;", "UPDATE statement done")]
  fn test_handle_query_statement_sql(
    #[case] input: &str,
    #[case] expected: &str,
  ) {
    let mut database = Database::new("testdb".to_string());
    let _ = match handle_sql_query(input, &mut database) {
      Ok(response) => assert_eq!(response, expected),
      Err(error) => {
        eprintln!("Error: {}", error);
        assert!(false)
      }
    };
  }

  #[rstest]
  #[case(
    "testdb",
    "CREATE TABLE test (
      id INTEGER PRIMARY KEY,
      name TEXT
    );",
    "INSERT INTO test (name) Values ('xxx');",
    "INSERT statement done",
  )]
  #[case(
    "testdb",
    "CREATE TABLE test (
      name TEXT
    );",
    "INSERT INTO test (name) Values ('xxx');",
    "INSERT statement done",
  )]
  fn test_handle_insert_sql(
    #[case] database_name: &str,
    #[case] query: &str,
    #[case] insert_query: &str,
    #[case] expected: &str,
  ) {
    let _ = match
      insert_table_into_database_and_insert_data_into_table(
        database_name,
        query,
        insert_query,
      ) {
        Ok(response) => assert_eq!(response, expected),
        Err(error) => {
          eprintln!("Error: {}", error);
          assert!(false)
        },
    };
  }

  fn insert_table_into_database_and_insert_data_into_table(
    database_name: &str,
    query: &str,
    insert_query: &str,
  ) -> Result<String, NollaDBError> {
    let mut database = Database::new(database_name.to_string());
    let dialect = SQLiteDialect {};
    let mut ast = Parser::parse_sql(&dialect, &query).unwrap();
    let create_query = CreateQuery::new(&ast.pop().unwrap()).unwrap();

    database.tables.insert(
      create_query.table_name.to_string(),
      Table::new(create_query),
    );

    match handle_sql_query(&insert_query, &mut database) {
      Ok(response) => Ok(response),
      Err(error) => Err(error),
    }
  }
}
