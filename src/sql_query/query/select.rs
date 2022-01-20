use sqlparser::ast::{
  Statement,
};
use crate::error::{Result, NollaDBError};

#[derive(Debug)]
pub struct SelectQuery {

}

impl SelectQuery {
  pub fn new(statement: &Statement) -> Result<SelectQuery> {
    match statement {
      Statement::Query(x) => {
        println!("{:?}", x);
      },
      _ => return Err(NollaDBError::Internal("Parsing SELECT SQL query error".to_string())),
    }

    Ok(
      SelectQuery {
      }
    )
  }
}
