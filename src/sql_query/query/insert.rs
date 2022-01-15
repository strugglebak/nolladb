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
  pub table_column_names: Vec<String>,
  pub table_column_values: Vec<Vec<String>>,
}

impl InsertQuery {
  pub fn new(statement: &Statement) -> Result<InsertQuery> {
    #[allow(unused_assignments)]
    let mut option_table_name: Option<String> = None;
    let mut table_column_names: Vec<String> = vec![];
    let mut table_column_values: Vec<Vec<String>> = vec![];

    match statement {
      Statement::Insert {
        table_name: name,
        columns,
        source,
        ..
      } => {
        option_table_name = Some(name.to_string());
        for column in columns {
          table_column_names.push(column.to_string());
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
            // 解析类似于
            //-- Values stored as TEXT, INTEGER, INTEGER, REAL, TEXT.
            // INSERT INTO t1 VALUES('500.0', '500.0', '500.0', '500.0', '500.0');
            // 的语句
            // body 里面是解析之后的 INSERT 之后的 ast
            // 把里面对应的表达式抽出来然后一个一个转成字符串
            if let SetExpr::Values(values) = body {
              #[allow(irrefutable_let_patterns)]
              if let Values(expressions) = values {
                for expression in expressions {
                  let mut table_column_value: Vec<String> = vec![];
                  for expr in expression {
                    match expr {
                      Expr::Value(v) => match v {
                        Value::Number(n, _) => table_column_value.push(n.to_string()),
                        Value::Boolean(b) => match *b {
                          true => table_column_value.push("true".to_string()),
                          false => table_column_value.push("false".to_string()),
                        },
                        Value::SingleQuotedString(sqs) => table_column_value.push(sqs.to_string()),
                        Value::Null => table_column_value.push("Null".to_string()),
                        _ => {},
                      },
                      Expr::Identifier(i) => table_column_value.push(i.to_string()),
                      _ => {},
                    }
                  }

                  table_column_values.push(table_column_value);
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
        table_column_names,
        table_column_values,
      }),
      _ => return Err(NollaDBError::Internal("Parsing INSERT SQL query error".to_string())),
    }
  }
}
