use sqlparser::ast::{
  Statement,
  Query,
  SetExpr,
  Values,
  Expr,
  Value,
};

use crate::error::{Result, NollaDBError};

#[derive(Debug)]
pub struct InsertQuery {
  pub table_name: String,
  pub table_metadata_columns: Vec<String>,
  pub table_rows: Vec<Vec<String>>,
}

impl InsertQuery {
  pub fn new(statement: &Statement) -> Result<InsertQuery> {
    #[allow(unused_assignments)]
    let mut option_table_name: Option<String> = None;
    let mut table_metadata_columns: Vec<String> = vec![];
    let mut table_rows: Vec<Vec<String>> = vec![];

    match statement {
      Statement::Insert {
        table_name: name,
        columns,
        source,
        ..
      } => {
        option_table_name = Some(name.to_string());
        for column in columns {
          table_metadata_columns.push(column.to_string());
        }

        // &Query
        match &**source {
          Query {
            body,
            // order_by,
            // limit,
            // offset,
            // fetch,
            ..
          } => {
            // body 里面是解析之后的 INSERT 之后的 ast
            // 把里面对应的表达式抽出来然后一个一个转成字符串
            if let SetExpr::Values(values) = body {
              #[allow(irrefutable_let_patterns)]
              if let Values(expressions) = values {
                for expression in expressions {
                  let mut table_row: Vec<String> = vec![];
                  for expr in expression {
                    match expr {
                      Expr::Value(v) => match v {
                        Value::Number(n, _) => table_row.push(n.to_string()),
                        Value::Boolean(b) => match *b {
                          true => table_row.push("true".to_string()),
                          false => table_row.push("false".to_string()),
                        },
                        Value::SingleQuotedString(sqs) => table_row.push(sqs.to_string()),
                        Value::Null => table_row.push("Null".to_string()),
                        _ => {},
                      },
                      Expr::Identifier(i) => table_row.push(i.to_string()),
                      _ => {},
                    }
                  }

                  table_rows.push(table_row);
                }
              };
            };
          }
        }
      },
      _ => return Err(NollaDBError::Internal("Parsing INSERT SQL query error".to_string())),
    }

    match option_table_name {
      Some(table_name) => Ok(InsertQuery {
        table_name,
        table_metadata_columns,
        table_rows,
      }),
      _ => return Err(NollaDBError::Internal("Parsing INSERT SQL query error".to_string())),
    }
  }
}
