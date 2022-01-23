use sqlparser::ast::{
  Statement,
  Query,
  SetExpr,
  SelectItem,
  Expr,
  TableFactor,
  ObjectName,
  BinaryOperator,
};
use crate::error::{Result, NollaDBError};

#[derive(Debug)]
struct BinaryOp {
  left: String,
  op: BinaryOperator,
  right: String,
}

impl BinaryOp {
  pub fn new() -> Self {
    Self {
      left: "".to_string(),
      op: BinaryOperator::Eq,
      right: "".to_string(),
    }
  }
}

#[derive(Debug)]
pub struct SelectQuery {
  select_column_names: Vec<String>,
  select_table_names: Vec<String>,
  select_table_condition: BinaryOp,
}

impl SelectQuery {
  pub fn new(statement: &Statement) -> Result<SelectQuery> {
    let mut select_column_names: Vec<String> = vec![];
    let mut select_table_names: Vec<String> = vec![];
    let mut select_table_condition = BinaryOp::new();

    match statement {
      Statement::Query(source) => {
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
            // SELECT x FROM t1 WHERE a = b
            // 的语句
            if let SetExpr::Select(select) = body {
              // println!("{:#?}", select);
              // 构建 select_column_names
              // 拿到 x
              let projection = &select.projection;
              for select_item in projection {
                if let SelectItem::UnnamedExpr(expr) = select_item {
                  if let Expr::Identifier(ident) = expr {
                    select_column_names.push(ident.value.to_string());
                  }
                }
              }

              // 构建 select_table_names
              // 拿到 t1
              let from = &select.from;
              for table_with_joins in from {
                let relation = &table_with_joins.relation;
                if let TableFactor::Table {name, ..} = relation {
                  #[allow(irrefutable_let_patterns)]
                  if let ObjectName(ident) = name {
                    select_table_names.push(ident[0].value.to_string());
                  }
                }
              }

              // 构建 select_table_condition
              // 拿到 a = b
              let selection = &select.selection;
              if let Some(binary_op) = selection {
                if let Expr::BinaryOp { left, op, right } = binary_op {
                  println!("{:#?}", left);
                  println!("{:#?}", op);
                  println!("{:#?}", right);
                  if let Expr::Identifier(ident) = &**left {
                    select_table_condition.left = ident.value.to_string();
                  }

                  select_table_condition.op = op.clone();

                  if let Expr::Identifier(ident) = &**right {
                    select_table_condition.right = ident.value.to_string();
                  }
                }
              }

              println!("{:?}, {:?}", select_column_names, select_table_names);
            }
          }
        }
      },
      _ => return Err(NollaDBError::Internal("Parsing SELECT SQL query error".to_string())),
    }

    Ok(
      SelectQuery {
        select_column_names,
        select_table_names,
        select_table_condition,
      }
    )
  }
}
