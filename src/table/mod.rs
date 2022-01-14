mod datatype;
mod row;

use std::collections::{HashMap};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Table {
  pub table_name: String,
  pub indexes: HashMapM<String, String>,
  pub most_recent_row_id: i64,
  pub primary_key: String,
}
